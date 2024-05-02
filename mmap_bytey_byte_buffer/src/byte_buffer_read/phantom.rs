use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};
use std::marker::PhantomData;

impl<T> MByteBufferRead for PhantomData<T> {
    #[inline]
    fn read_from_mbuffer(_buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }

    #[inline]
    fn read_from_mbuffer_le(_buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }

    #[inline]
    fn read_from_mbuffer_be(_buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }
}
