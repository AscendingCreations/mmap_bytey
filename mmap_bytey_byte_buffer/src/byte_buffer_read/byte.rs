use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};

impl MByteBufferRead for bool {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }
}
