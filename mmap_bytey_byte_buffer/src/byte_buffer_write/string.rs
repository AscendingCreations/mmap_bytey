use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl MByteBufferWrite for str {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl<'a> MByteBufferWrite for &'a str {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl MByteBufferWrite for String {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl MByteBufferWrite for &String {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_mbuffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}
