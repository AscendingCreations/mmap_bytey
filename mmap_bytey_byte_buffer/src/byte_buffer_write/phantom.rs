use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::marker::PhantomData;

impl<T> MByteBufferWrite for PhantomData<T> {
    #[inline]
    fn write_to_mbuffer(&self, _buffer: &mut MByteBuffer) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, _buffer: &mut MByteBuffer) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_be(&self, _buffer: &mut MByteBuffer) -> Result<()> {
        Ok(())
    }
}
