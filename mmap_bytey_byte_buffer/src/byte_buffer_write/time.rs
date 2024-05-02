use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::time::Duration;

impl MByteBufferWrite for Duration {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_mbuffer(buffer)?;
        self.subsec_nanos().write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_mbuffer_le(buffer)?;
        self.subsec_nanos().write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_secs().write_to_mbuffer_be(buffer)?;
        self.subsec_nanos().write_to_mbuffer_be(buffer)
    }
}
