use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::ops::{Range, RangeInclusive};

impl<T: MByteBufferWrite> MByteBufferWrite for Range<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start.write_to_mbuffer(buffer)?;
        self.end.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start.write_to_mbuffer_le(buffer)?;
        self.end.write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start.write_to_mbuffer_be(buffer)?;
        self.end.write_to_mbuffer_be(buffer)
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for RangeInclusive<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start().write_to_mbuffer(buffer)?;
        self.end().write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start().write_to_mbuffer_le(buffer)?;
        self.end().write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.start().write_to_mbuffer_be(buffer)?;
        self.end().write_to_mbuffer_be(buffer)
    }
}
