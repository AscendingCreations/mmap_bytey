use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl MByteBufferWrite for char {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer_be(buffer)
    }
}
