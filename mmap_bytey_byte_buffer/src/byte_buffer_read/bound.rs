use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};
use std::ops::Bound;

impl<T: MByteBufferRead> MByteBufferRead for Bound<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Self> {
        match buffer.read::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read::<T>()?)),
            _ => Err(MByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        match buffer.read_le::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read_le::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read_le::<T>()?)),
            _ => Err(MByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        match buffer.read_be::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read_be::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read_be::<T>()?)),
            _ => Err(MByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }
}
