use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};
use std::cell::{Cell, RefCell};

impl<T: MByteBufferRead> MByteBufferRead for Cell<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read_be::<T>()?))
    }
}

impl<T: MByteBufferRead> MByteBufferRead for RefCell<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read_be::<T>()?))
    }
}
