use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};

impl<T: MByteBufferRead> MByteBufferRead for Box<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read_be::<T>()?))
    }
}
