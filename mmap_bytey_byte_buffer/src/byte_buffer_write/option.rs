use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl<T: MByteBufferWrite> MByteBufferWrite for Option<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_mbuffer(buffer)?;
                v.write_to_mbuffer(buffer)
            }
            None => 2u8.write_to_mbuffer(buffer),
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_mbuffer_le(buffer)?;
                v.write_to_mbuffer_le(buffer)
            }
            None => 2u8.write_to_mbuffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_mbuffer_be(buffer)?;
                v.write_to_mbuffer_be(buffer)
            }
            None => 2u8.write_to_mbuffer_be(buffer),
        }
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for &Option<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_mbuffer(buffer)?;
                v.write_to_mbuffer(buffer)
            }
            None => 2u8.write_to_mbuffer(buffer),
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_mbuffer_le(buffer)?;
                v.write_to_mbuffer_le(buffer)
            }
            None => 2u8.write_to_mbuffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_mbuffer_be(buffer)?;
                v.write_to_mbuffer_be(buffer)
            }
            None => 2u8.write_to_mbuffer_be(buffer),
        }
    }
}
