use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use dashmap::DashMap;
use intrusive_collections::{intrusive_adapter, SinglyLinkedList, SinglyLinkedListLink, UnsafeRef};
use mmap_rs::{MmapMut, MmapOptions};
use std::sync::{Arc, OnceLock};
use std::thread::ThreadId;

///The Max and Minimal size the buffer will statically be.
pub const BUFFER_SIZE: usize = 1536;
///The amount of Pages per buffer allocation is generated.
pub const PAGE_CREATION_COUNT: usize = 12;

struct List {
    list: SinglyLinkedList<Adapter>,
    dropping: bool,
}

unsafe impl Send for List {}
unsafe impl Sync for List {}

impl List {
    pub fn new() -> Self {
        Self {
            list: SinglyLinkedList::new(Adapter::default()),
            dropping: false,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        self.dropping = true;

        while let Some(node) = self.list.pop_front() {
            let _mapping = unsafe { Arc::from_raw(node.mapping) };
        }
    }
}

thread_local! {
    static FREE_LIST: RefCell<List> = RefCell::new(List::new());
}

static MAILBOX: OnceLock<DashMap<std::thread::ThreadId, List>> = OnceLock::new();

fn get_mailbox<'a>() -> &'a DashMap<std::thread::ThreadId, List> {
    MAILBOX.get_or_init(DashMap::new)
}

/// The allocated mmap buffer. This carries over its threadID just in case you do happen
/// to pass the buffer to another thread. that way it can return to its main thread upon drop.
#[derive(Debug)]
pub struct Buffer {
    mapping: Arc<MmapMut>,
    thread_id: ThreadId,
    offset: usize,
}

unsafe impl Sync for Buffer {}
unsafe impl Send for Buffer {}

impl Buffer {
    /// Retrived a Buffer from the list of preallocated buffers. if fails will allocate more buffers.
    pub fn new() -> Result<Self, mmap_rs::Error> {
        let thread_id = std::thread::current().id();

        // Try to allocate from the thread-local free list first. This should be the cheapest
        // option as we don't have to call VirtualAlloc/mmap or rely on thread synchronization.
        if let Some(node) = FREE_LIST.with_borrow_mut(|free_list| free_list.list.pop_front()) {
            let buffer = Buffer {
                mapping: unsafe { Arc::from_raw(node.mapping) },
                thread_id,
                offset: node.offset,
            };

            return Ok(buffer);
        }

        // Try to collect any buffers from the thread's mailbox in case any other thread returned
        // buffers to this thread.
        if let Some((_, mut list)) = get_mailbox().remove(&thread_id) {
            let mut buffer = None;

            // In case of multiple buffers, we simply take them all and override the buffer in
            // order to drop the previous one, thus returning n - 1 buffers to the free list.
            while let Some(node) = list.list.pop_front() {
                buffer = Some(Buffer {
                    mapping: unsafe { Arc::from_raw(node.mapping) },
                    thread_id,
                    offset: node.offset,
                });
            }

            // Return the last buffer in the list.
            if let Some(buffer) = buffer {
                return Ok(buffer);
            }
        }

        // Call VirtualAlloc/mmap to allocate a number of pages and split it up into as many
        // buffers as possible. Return the first, and put the remaining buffers on the thread-local
        // free list.
        let mapping =
            Arc::new(MmapOptions::new(PAGE_CREATION_COUNT * MmapOptions::page_size())?.map_mut()?);

        for offset in (0..PAGE_CREATION_COUNT * MmapOptions::page_size()).step_by(BUFFER_SIZE) {
            if offset == 0 {
                continue;
            }

            let buffer = Buffer {
                mapping: mapping.clone(),
                thread_id,
                offset,
            };

            drop(buffer);
        }

        Ok(Buffer {
            mapping,
            thread_id,
            offset: 0,
        })
    }

    /// Clones the buffer by copying its internal data into a new buffer.
    pub fn try_clone(&self) -> Result<Self, mmap_rs::Error> {
        let mut buffer = Buffer::new()?;

        buffer[..].copy_from_slice(&self[..]);

        Ok(buffer)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // Do nothing if the free list is being dropped. This is to avoid pushing back any buffers
        // onto the free list, which would result in an infinite loop.
        if FREE_LIST.with_borrow(|free_list| free_list.dropping) {
            return;
        }

        // Initialize the buffer as a linked list node.
        let node: &mut Node =
            unsafe { &mut *(self.mapping.as_ptr().add(self.offset) as *mut Node) };

        node.link = SinglyLinkedListLink::new();
        node.mapping = Arc::into_raw(self.mapping.clone());
        node.thread_id = self.thread_id;
        node.offset = self.offset;

        // Store the node onto the thread-local free list or return it to the thread the buffer
        // belongs to through the appropriate mailbox.
        let node =
            unsafe { UnsafeRef::from_raw(self.mapping.as_ptr().add(self.offset) as *mut Node) };

        let thread_id = std::thread::current().id();

        if self.thread_id == thread_id {
            FREE_LIST.with_borrow_mut(|free_list| free_list.list.push_front(node));
        } else if let Some(mut list) = get_mailbox().get_mut(&thread_id) {
            list.list.push_front(node);
        } else {
            let mut list = List::new();
            list.list.push_front(node);

            // Set up the list for the thread ID if there is none set up yet.
            if let Some(mut old_list) = get_mailbox().insert(thread_id, list) {
                // In case we override an existing list, we simply push the nodes of the old
                // list onto the new list.
                if let Some(mut list) = get_mailbox().get_mut(&thread_id) {
                    while let Some(node) = old_list.list.pop_front() {
                        list.list.push_front(node);
                    }
                }
            }
        }
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.mapping.as_ptr().add(self.offset), BUFFER_SIZE) }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.mapping.as_ptr().add(self.offset) as *mut u8,
                BUFFER_SIZE,
            )
        }
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.mapping.as_ptr().add(self.offset), BUFFER_SIZE) }
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.mapping.as_ptr().add(self.offset) as *mut u8,
                BUFFER_SIZE,
            )
        }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        let mut buffer = Buffer::new().unwrap();

        buffer[..].copy_from_slice(&self[..]);

        buffer
    }

    fn clone_from(&mut self, source: &Self) {
        self.copy_from_slice(&source[..]);
    }
}

/// The node in the mmap list to stored our buffers for reuse.
pub struct Node {
    link: SinglyLinkedListLink,
    mapping: *const MmapMut,
    thread_id: ThreadId,
    offset: usize,
}

intrusive_adapter!(Adapter = UnsafeRef<Node>: Node { link: SinglyLinkedListLink });
