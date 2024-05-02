use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};

impl<T: MByteBufferRead> MByteBufferRead for Option<T> {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read::<u8>()? {
            1 => Some(buffer.read::<T>()?),
            2 => None,
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_le::<u8>()? {
            1 => Some(buffer.read_le::<T>()?),
            2 => None,
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_be::<u8>()? {
            1 => Some(buffer.read_be::<T>()?),
            2 => None,
            _ => {
                return Err(MByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }
}
