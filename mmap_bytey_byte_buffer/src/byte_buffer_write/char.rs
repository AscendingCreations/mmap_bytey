use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl MByteBufferWrite for char {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer_be(buffer)
    }
}
