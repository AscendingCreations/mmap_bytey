use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};
use std::borrow::Cow;

impl<T: MByteBufferRead + ToOwned<Owned = T>> MByteBufferRead for Cow<'_, T> {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_be::<T>()?))
    }
}
