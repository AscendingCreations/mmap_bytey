use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};
use std::borrow::Cow;

impl<T: MByteBufferRead + ToOwned<Owned = T>> MByteBufferRead for Cow<'_, T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_be::<T>()?))
    }
}
