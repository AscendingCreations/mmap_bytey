use crate::byte_buffer_write::MByteBufferWrite;
use crate::{
    allocator::{Buffer, BUFFER_SIZE},
    byte_buffer_read::MByteBufferRead,
};
use std::{ptr, slice};

use crate::error::{MByteBufferError, Result};
/// A resizeable buffer to store data in.
///
/// Provides a resizeable buffer with an initial capacity of N bytes.
/// All data written to the [`MByteBuffer`] has to implement the [`ByteBufferWrite`] trait or be a slice of type [u8].
///
/// Data read from the [`MByteBuffer`] has to implement the [`ByteBufferRead`] trait.
///
/// # Examples
/// ```
/// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
///
/// let mut buffer = MByteBuffer::new().unwrap();
/// let value: u32 = 200;
///
/// // stores the value in the buffer and moves the cursor by 4
/// // due to u32 being 4 bytes in size
/// buffer.write(&value);
///
/// buffer.move_cursor(0);
///
/// // prints 200
/// println!("The stored value is: {}!", buffer.read::<u32>().unwrap());
/// ```
///
#[derive(Debug)]
pub struct MByteBuffer {
    length: usize,
    cursor: usize,
    buffer: Buffer,
}

/// `MByteBuffer` are `Send` Becuase `u8` is `Send` because the data they
/// reference is unaliased.
unsafe impl Send for MByteBuffer {}

/// `MByteBuffer` pointers are `Sync` if `u8` is `Sync` because the data they
/// reference is unaliased.
unsafe impl Sync for MByteBuffer {}

impl MByteBuffer {
    /// Constructs a new [`MByteBuffer`] with a capacity of 1024 bytes.
    /// This uses a custom allocator to stage a default Buffer Size for Efficiency.
    /// If you need more than 1024 bytes I suggest spliting the data into multiple buffers.
    ///
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self {
            length: 0,
            cursor: 0,
            buffer: Buffer::new()?,
        })
    }

    /// Writes a slice of type [u8] to the [`MByteBuffer`] **without safety checks**.
    ///
    /// # Safety
    ///
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure all of the following:
    /// - The length of the slice doesn't exceed the capacity.
    /// - The cursor position + length of the slice does not exceed the capacity.
    /// - The cursor position is not out of bounds
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the length of the slice.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let values: [u8; 4] = [0, 1, 2, 3];
    ///
    /// unsafe {
    ///     buffer.write_slice_unchecked(&values);
    /// }
    /// ```
    pub unsafe fn write_slice_unchecked(&mut self, source: &[u8]) -> &mut Self {
        let source_length = source.len();

        ptr::copy_nonoverlapping(
            source.as_ptr(),
            self.buffer.as_mut().as_mut_ptr().add(self.cursor),
            source_length,
        );
        self.cursor += source.len();

        self
    }

    /// Writes a slice of type [u8] to the [`MByteBuffer`].
    ///
    /// # Behaviour
    /// - If the result of the **current cursor position + length of the slice** exceeds the capacity of the buffer,
    ///   the buffer will resize to the next power of two that fits the result.
    /// - The current cursor position will be increased by the length of the slice.
    ///
    /// # Errors
    /// - [`MByteBufferError::MaxCapacity`] is returned if the buffer has to resize to a capacity larger than [`MAX_SIZE`](Self::MAX_SIZE)
    ///   or if the resulting capacity overflows.
    /// - [`MByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let values: [u8; 4] = [0, 1, 2, 3];
    ///
    /// buffer.write_slice(&values);
    /// ```
    pub fn write_slice(&mut self, source: &[u8]) -> Result<&mut Self> {
        if self.cursor + source.len() > BUFFER_SIZE {
            return Err(MByteBufferError::MaxCapacity);
        }

        unsafe {
            self.write_slice_unchecked(source);
        }

        if self.cursor > self.length {
            self.length += self.cursor - self.length
        }

        Ok(self)
    }

    /// Writes the given value to the [`MByteBuffer`].
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// ```
    pub fn write<T: MByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_mbuffer(self)?;

        Ok(self)
    }

    /// Writes the given value to the [`MByteBuffer`] in **little endian** ordering.
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_le(&value);
    /// ```
    pub fn write_le<T: MByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_mbuffer_le(self)?;

        Ok(self)
    }

    /// Writes the given value to the [`MByteBuffer`] in **big endian** ordering.
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_be(&value);
    /// ```
    pub fn write_be<T: MByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_mbuffer_be(self)?;

        Ok(self)
    }

    /// Reads a slice of type [u8] from the [`MByteBuffer`] of the given size **without safety checks**.
    ///
    /// # Safety
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure all of the following:
    /// - The size does not exceed the capacity of the buffer.
    /// - The result of cursor position + the given size does not exceed the length of the buffer.
    /// - The cursor position is not out of bounds
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the given size.
    ///
    /// # Examples
    ///```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// unsafe {
    //      println!("{:?}", buffer.read_slice_unchecked(4));
    /// }
    ///```
    pub unsafe fn read_slice_unchecked(&mut self, size: usize) -> &[u8] {
        let ret = slice::from_raw_parts(self.buffer.as_ref().as_ptr().add(self.cursor), size);
        self.cursor += size;

        ret
    }

    /// Reads a slice of type [u8] from the [`MByteBuffer`] of the given size.
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the given size.
    ///
    /// # Errors
    /// - [`MByteBufferError::ReadOutOfBounds`] is returned if the result of the current cursor position + the given size exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{:?}", buffer.read_slice(4));
    /// ```
    pub fn read_slice(&mut self, size: usize) -> Result<&[u8]> {
        if self.cursor + size > self.length {
            return Err(MByteBufferError::ReadOutOfBounds {
                length: self.length,
                start: self.cursor,
                end: self.cursor + size,
            });
        }

        Ok(unsafe { self.read_slice_unchecked(size) })
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read().unwrap();
    /// ```
    pub fn read<T: MByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_mbuffer(self)
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer in **little endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_le(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read_le::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read_le().unwrap();
    /// ```
    pub fn read_le<T: MByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_mbuffer_le(self)
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer in **big endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_be(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read_be::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read_be().unwrap();
    /// ```
    pub fn read_be<T: MByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_mbuffer_be(self)
    }

    /// Moves the current cursor position **without safety checks**.
    ///
    /// # Safety
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure the given location does not exceed the buffer's length.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// unsafe {
    ///     buffer.move_cursor_unchecked(2);
    /// }
    /// ```
    pub unsafe fn move_cursor_unchecked(&mut self, location: usize) -> &mut Self {
        self.cursor = location;

        self
    }

    /// Moves the current cursor position.
    ///
    /// # Errors
    /// - [`MByteBufferError::CursorOutOfBounds`] if the cursor exceeds the buffers length
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor(2);
    /// ```
    pub fn move_cursor(&mut self, location: usize) -> Result<&mut Self> {
        if location > self.length {
            return Err(MByteBufferError::CursorOutOfBounds {
                length: self.length,
                cursor: location,
            });
        }

        self.cursor = location;

        Ok(self)
    }

    /// Moves the current cursor position to the length of the buffer.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor_to_end();
    /// ```
    pub fn move_cursor_to_end(&mut self) -> &mut Self {
        self.cursor = self.length;
        self
    }

    /// Moves the current cursor position to 0.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor_to_start();
    /// ```
    pub fn move_cursor_to_start(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }

    /// Resets length without resizing array.
    ///
    /// # Behaviour
    /// buffer's length will be set to length and buffer's cursor will be set to length
    /// if greater than length.
    ///
    /// # Errors
    /// - [`MByteBufferError::LengthOutOfBounds`] is returned if length exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let _ = buffer.truncate(0).unwrap();
    /// ```
    pub fn truncate(&mut self, length: usize) -> Result<&mut Self> {
        if length > self.length {
            return Err(MByteBufferError::LengthOutOfBounds {
                current: self.length,
                new: length,
            });
        }

        self.length = length;

        if self.cursor > length {
            self.cursor = length;
        }

        Ok(self)
    }

    /// Returns the length of the [`MByteBuffer`].
    ///
    /// The length of the buffer is the last index written to - 1.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// println!("{}", buffer.length());
    /// ```
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the capacity of the [`MByteBuffer`].
    ///
    /// The capacity of the buffer is the size of the heap allocation used to store data.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    ///
    /// println!("{}", buffer.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        BUFFER_SIZE
    }

    /// Returns the current cursor position of the [`MByteBuffer`].
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// println!("{}", buffer.cursor());
    /// ```
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns a const pointer to the allocation.
    ///
    /// # Safety
    /// This method is unsafe due to the unsafe nature of pointers itself.
    ///
    /// This method can result in undefined behaviour if the buffer is resized and the underlying heap allocator moves
    /// the pointer
    pub unsafe fn pointer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    /// Returns a mutable pointer to the allocation.
    ///
    /// # Safety
    /// This method is unsafe due to the unsafe nature of pointers itself.
    ///
    /// This method can result in undefined behaviour if the buffer is resized and the underlying heap allocator moves
    /// the pointer
    pub unsafe fn mut_pointer(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }

    /// Returns true if the length is 0
    ///
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns a new [`MByteBuffer`] with data at the old [`MByteBuffer`]'s Cursor
    /// to the length of the New Buffer. This will move the old buffers cursor.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice), [`with_capacity`](Self::with_capacity) and [`write_slice`](Self::write_slice)
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor(0);
    ///
    /// let mut new_buffer = buffer.read_to_buffer(4).unwrap();
    /// let value = new_buffer.read::<u32>().unwrap();
    /// assert_eq!(value, 12345);
    /// ```
    #[inline]
    pub fn read_to_buffer(&mut self, len: usize) -> Result<Self> {
        let mut buffer = MByteBuffer::new()?;
        let bytes = self.read_slice(len)?;
        buffer.write_slice(bytes)?;
        buffer.cursor = 0;
        Ok(buffer)
    }

    /// Returns a slice of the entire [`MByteBuffer`] from cursor position 0.
    ///
    /// # Behaviour
    /// The current cursor position will be self.length.
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let slice = buffer.as_slice();
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn as_slice(&mut self) -> &[u8] {
        self.cursor = 0;
        unsafe { self.read_slice_unchecked(self.length()) }
    }

    /// Returns a slice of the entire [`MByteBuffer`] from a given cursor position to a given size.
    ///
    /// # Behaviour
    /// The current cursor position will be the given cursor + given size.
    ///
    /// # Errors
    /// - [`MByteBufferError::ReadOutOfBounds`] is returned if the result of the current cursor position + the given size exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
    ///
    /// let mut buffer = MByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let slice = buffer.slice_from(0, 4).unwrap();
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn slice_from(&mut self, cursor: usize, size: usize) -> Result<&[u8]> {
        if cursor + size > self.length {
            return Err(MByteBufferError::ReadOutOfBounds {
                length: self.length,
                start: cursor,
                end: cursor + size,
            });
        }

        self.cursor = cursor;
        Ok(unsafe { self.read_slice_unchecked(size) })
    }

    /// A Panicless Clone that returns a Error instead.
    pub fn try_clone(&self) -> Result<Self> {
        let mut buffer = Buffer::new()?;

        buffer[..].copy_from_slice(&self.buffer[..]);

        Ok(Self {
            length: self.length,
            cursor: self.cursor,
            buffer,
        })
    }
}

impl Clone for MByteBuffer {
    fn clone(&self) -> Self {
        Self {
            length: self.length,
            cursor: self.cursor,
            buffer: self.buffer.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.length = source.length;
        self.cursor = source.cursor;
        self.buffer.copy_from_slice(&source.buffer[..]);
    }
}
