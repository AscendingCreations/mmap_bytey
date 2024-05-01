use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::time::Duration;

impl MByteBufferWrite for Duration {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer(buffer)?;
        self.subsec_nanos().write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer_le(buffer)?;
        self.subsec_nanos().write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer_be(buffer)?;
        self.subsec_nanos().write_to_buffer_be(buffer)
    }
}
