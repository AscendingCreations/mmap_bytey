use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
};

impl MByteBufferRead for NonZeroI8 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroU8 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroI16 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read::<i16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read_le::<i16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read_be::<i16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroU16 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read::<u16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read_le::<u16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read_be::<u16>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroI32 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read::<i32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read_le::<i32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read_be::<i32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroU32 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read::<u32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read_le::<u32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read_be::<u32>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroI64 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read::<i64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read_le::<i64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read_be::<i64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroU64 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read::<u64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read_le::<u64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read_be::<u64>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroI128 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read::<i128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read_le::<i128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read_be::<i128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroU128 {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read::<u128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read_le::<u128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read_be::<u128>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroIsize {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read::<isize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read_le::<isize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read_be::<isize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl MByteBufferRead for NonZeroUsize {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read::<usize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read_le::<usize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read_be::<usize>()?).ok_or(MByteBufferError::NonZeroIsZero)
    }
}

impl<T: MByteBufferRead> MByteBufferRead for Wrapping<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read_be::<T>()?))
    }
}

impl<T: MByteBufferRead> MByteBufferRead for Saturating<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read_be::<T>()?))
    }
}
