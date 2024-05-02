use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::ops::Bound;

impl<T: MByteBufferWrite> MByteBufferWrite for Bound<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_mbuffer(buffer),
            Self::Included(val) => {
                1u8.write_to_mbuffer(buffer)?;
                val.write_to_mbuffer(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_mbuffer(buffer)?;
                val.write_to_mbuffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_mbuffer_le(buffer),
            Self::Included(val) => {
                1u8.write_to_mbuffer_le(buffer)?;
                val.write_to_mbuffer_le(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_mbuffer_le(buffer)?;
                val.write_to_mbuffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_mbuffer_be(buffer),
            Self::Included(val) => {
                1u8.write_to_mbuffer_be(buffer)?;
                val.write_to_mbuffer_be(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_mbuffer_be(buffer)?;
                val.write_to_mbuffer_be(buffer)
            }
        }
    }
}
