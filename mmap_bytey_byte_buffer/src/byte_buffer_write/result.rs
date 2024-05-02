use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl<T: MByteBufferWrite, E: MByteBufferWrite> MByteBufferWrite for std::result::Result<T, E> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_mbuffer(buffer)?;
                v.write_to_mbuffer(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer(buffer)?;
                e.write_to_mbuffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_mbuffer_le(buffer)?;
                v.write_to_mbuffer_le(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer_le(buffer)?;
                e.write_to_mbuffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_mbuffer_be(buffer)?;
                v.write_to_mbuffer_be(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer_be(buffer)?;
                e.write_to_mbuffer_be(buffer)
            }
        }
    }
}

impl<T: MByteBufferWrite, E: MByteBufferWrite> MByteBufferWrite for &std::result::Result<T, E> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u8.write_to_mbuffer(buffer)?;
                v.write_to_mbuffer(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer(buffer)?;
                e.write_to_mbuffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u8.write_to_mbuffer_le(buffer)?;
                v.write_to_mbuffer_le(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer_le(buffer)?;
                e.write_to_mbuffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u8.write_to_mbuffer_be(buffer)?;
                v.write_to_mbuffer_be(buffer)
            }
            Err(e) => {
                2u8.write_to_mbuffer_be(buffer)?;
                e.write_to_mbuffer_be(buffer)
            }
        }
    }
}

impl<E: MByteBufferWrite> MByteBufferWrite for std::result::Result<(), E> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_mbuffer(buffer),
            Err(e) => {
                2u8.write_to_mbuffer(buffer)?;
                e.write_to_mbuffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_mbuffer_le(buffer),
            Err(e) => {
                2u8.write_to_mbuffer_le(buffer)?;
                e.write_to_mbuffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_mbuffer_be(buffer),
            Err(e) => {
                2u8.write_to_mbuffer_be(buffer)?;
                e.write_to_mbuffer_be(buffer)
            }
        }
    }
}

impl<E: MByteBufferWrite> MByteBufferWrite for &std::result::Result<(), E> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_mbuffer(buffer),
            Err(e) => {
                2u8.write_to_mbuffer(buffer)?;
                e.write_to_mbuffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_mbuffer_le(buffer),
            Err(e) => {
                2u8.write_to_mbuffer_le(buffer)?;
                e.write_to_mbuffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_mbuffer_be(buffer),
            Err(e) => {
                2u8.write_to_mbuffer_be(buffer)?;
                e.write_to_mbuffer_be(buffer)
            }
        }
    }
}
