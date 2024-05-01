use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl<T: MByteBufferWrite> MByteBufferWrite for Option<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            None => 2u8.write_to_buffer(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            None => 2u8.write_to_buffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u8.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            None => 2u8.write_to_buffer_be(buffer),
        }
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for &Option<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            None => 2u8.write_to_buffer(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            None => 2u8.write_to_buffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u8.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            None => 2u8.write_to_buffer_be(buffer),
        }
    }
}
