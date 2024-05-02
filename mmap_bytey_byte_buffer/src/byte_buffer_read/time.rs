use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};
use std::time::Duration;

impl MByteBufferRead for Duration {
    #[inline]
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> Result<Self> {
        let secs = buffer.read::<u64>()?;
        let nanos = buffer.read::<u32>()?;

        new_duration(secs, nanos)
    }

    #[inline]
    fn read_from_mbuffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        let secs = buffer.read_le::<u64>()?;
        let nanos = buffer.read_le::<u32>()?;

        new_duration(secs, nanos)
    }

    #[inline]
    fn read_from_mbuffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        let secs = buffer.read_be::<u64>()?;
        let nanos = buffer.read_be::<u32>()?;

        new_duration(secs, nanos)
    }
}

fn new_duration(secs: u64, nanos: u32) -> Result<Duration> {
    if secs
        .checked_add(u64::from(nanos) / 1_000_000_000u64)
        .is_none()
    {
        return Err(MByteBufferError::OtherError {
            error: format!("Invalid Duration: secs: {} nanos: {}", secs, nanos),
        });
    }

    Ok(Duration::new(secs, nanos))
}
