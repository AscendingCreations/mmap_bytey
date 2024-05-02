use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl MByteBufferWrite for std::num::NonZeroI8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i8::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i8::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i8::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroU8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u8::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u8::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u8::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroI16 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i16::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i16::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i16::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroU16 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u16::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u16::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u16::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroI32 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i32::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i32::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i32::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroU32 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u32::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroI64 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i64::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i64::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i64::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroU64 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u64::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u64::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u64::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroI128 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i128::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i128::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        i128::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroU128 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u128::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u128::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        u128::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroIsize {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        isize::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        isize::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        isize::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for std::num::NonZeroUsize {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        usize::from(*self).write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        usize::from(*self).write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        usize::from(*self).write_to_mbuffer_be(buffer)
    }
}

impl<T: MByteBufferWrite + Copy> MByteBufferWrite for std::num::Wrapping<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer_be(buffer)
    }
}

impl<T: MByteBufferWrite + Copy> MByteBufferWrite for std::num::Saturating<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.0.write_to_mbuffer_be(buffer)
    }
}
