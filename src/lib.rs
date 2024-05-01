#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;

#[doc(inline)]
pub use mmap_bytey_byte_buffer::allocator::BUFFER_SIZE;

#[doc(inline)]
pub use mmap_bytey_byte_buffer::error::{MByteBufferError, Result};

#[doc(inline)]
pub use mmap_bytey_byte_buffer::byte_buffer_write::{self, MByteBufferWrite};

#[doc(inline)]
pub use mmap_bytey_byte_buffer::byte_buffer_read::{self, MByteBufferRead};

pub use mmap_bytey_derive::MByteBufferRead;
pub use mmap_bytey_derive::MByteBufferWrite;
