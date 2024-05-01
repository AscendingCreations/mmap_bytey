use crate::{byte_buffer::MByteBuffer, byte_buffer_read::MByteBufferRead, error::Result};

impl MByteBufferRead for String {
    #[inline]
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Self> {
        let len = buffer.read::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
        let len = buffer.read_le::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
        let len = buffer.read_be::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }
}
