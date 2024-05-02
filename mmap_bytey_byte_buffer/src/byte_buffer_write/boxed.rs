use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl<T: MByteBufferWrite> MByteBufferWrite for Box<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_mbuffer_be(buffer)
    }
}
