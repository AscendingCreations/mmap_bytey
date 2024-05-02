use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};

impl MByteBufferRead for char {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_le::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_be::<u32>()?).ok_or(MByteBufferError::NotAChar)
    }
}
