use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use intrusive_collections::{intrusive_adapter, SinglyLinkedList, SinglyLinkedListLink};
use mmap_rs::{MmapMut, MmapOptions};
use std::sync::Arc;

pub const BUFFER_SIZE: usize = 1024;

struct List(SinglyLinkedList<Adapter>);

impl List {
    pub fn new() -> Self {
        Self(SinglyLinkedList::new(Adapter::default()))
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Some(mut node) = self.0.pop_front() {
            let mut buffer = node.buffer.take().unwrap();
            drop(buffer.mapping.take());
            std::mem::forget(buffer);
            std::mem::forget(node);
        }
    }
}

thread_local! {
    static FREE_LIST: RefCell<List> = RefCell::new(List::new());
}

#[derive(Debug)]
pub struct Buffer {
    mapping: Option<Arc<MmapMut>>,
    offset: usize,
}

impl Buffer {
    pub fn new() -> Result<Self, mmap_rs::Error> {
        if let Some(mut node) = FREE_LIST.with_borrow_mut(|free_list| free_list.0.pop_front()) {
            let buffer = node.buffer.take().unwrap();
            std::mem::forget(node);
            return Ok(buffer);
        }

        let mapping = Arc::new(MmapOptions::new(8 * MmapOptions::page_size())?.map_mut()?);

        for offset in (0..8 * MmapOptions::page_size()).step_by(BUFFER_SIZE) {
            if offset == 0 {
                continue;
            }

            let buffer = Buffer {
                mapping: Some(mapping.clone()),
                offset,
            };

            drop(buffer);
        }

        Ok(Buffer {
            mapping: Some(mapping),
            offset: 0,
        })
    }

    pub fn try_clone(&self) -> Result<Self, mmap_rs::Error> {
        let mut buffer = Buffer::new()?;

        buffer[..].copy_from_slice(&self[..]);

        Ok(buffer)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let mapping = match self.mapping.take() {
            Some(mapping) => mapping,
            _ => return,
        };

        //unsafe { Box::from_raw(mapping.clone().as_ptr().add(self.offset) as *mut Node) };
        let node = Node {
            link: SinglyLinkedListLink::new(),
            buffer: Some(Self {
                mapping: Some(mapping),
                offset: self.offset,
            }),
        };

        FREE_LIST.with_borrow_mut(|free_list| free_list.0.push_front(Box::new(node)));
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.mapping.as_ref().unwrap().as_ptr().add(self.offset),
                BUFFER_SIZE,
            )
        }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.mapping.as_ref().unwrap().as_ptr().add(self.offset) as *mut u8,
                BUFFER_SIZE,
            )
        }
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.mapping.as_ref().unwrap().as_ptr().add(self.offset),
                BUFFER_SIZE,
            )
        }
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.mapping.as_ref().unwrap().as_ptr().add(self.offset) as *mut u8,
                BUFFER_SIZE,
            )
        }
    }
}

pub struct Node {
    link: SinglyLinkedListLink,
    buffer: Option<Buffer>,
}

intrusive_adapter!(Adapter = Box<Node>: Node { link: SinglyLinkedListLink });
