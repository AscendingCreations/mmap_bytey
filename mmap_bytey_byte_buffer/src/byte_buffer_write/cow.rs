use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::borrow::Cow;

impl<T: MByteBufferWrite + Clone> MByteBufferWrite for Cow<'_, T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.as_ref().write_to_buffer_be(buffer)
    }
}
