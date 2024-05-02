use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};
use std::ops::{Range, RangeInclusive};

impl<T: MByteBufferRead> MByteBufferRead for Range<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }
}

impl<T: MByteBufferRead> MByteBufferRead for RangeInclusive<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }
}
