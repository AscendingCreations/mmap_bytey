use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};

impl MByteBufferRead for char {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_le::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_be::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }
}
