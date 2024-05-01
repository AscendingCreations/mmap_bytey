use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};

impl<T: MByteBufferRead, E: MByteBufferRead> MByteBufferRead for std::result::Result<T, E> {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read::<u8>()? {
            1 => Ok(buffer.read::<T>()?),
            2 => Err(buffer.read::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read_le::<u8>()? {
            1 => Ok(buffer.read_le::<T>()?),
            2 => Err(buffer.read_le::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read_be::<u8>()? {
            1 => Ok(buffer.read_be::<T>()?),
            2 => Err(buffer.read_be::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }
}

impl<E: MByteBufferRead> MByteBufferRead for std::result::Result<(), E> {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read::<u8>()? {
            1 => Ok(()),
            2 => Err(buffer.read::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read_le::<u8>()? {
            1 => Ok(()),
            2 => Err(buffer.read_le::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read_be::<u8>()? {
            1 => Ok(()),
            2 => Err(buffer.read_be::<E>()?),
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }
}
