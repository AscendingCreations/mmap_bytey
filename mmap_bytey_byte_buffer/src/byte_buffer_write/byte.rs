use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};
use std::slice;

impl MByteBufferWrite for u8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}

impl MByteBufferWrite for &u8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}

impl MByteBufferWrite for i8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}

impl MByteBufferWrite for &i8 {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}

impl MByteBufferWrite for bool {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let data = if *self { 1u8 } else { 0u8 };
        buffer.write(data)?;

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}

impl MByteBufferWrite for &bool {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        let data = if **self { 1u8 } else { 0u8 };
        buffer.write(data)?;

        Ok(())
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.write_to_mbuffer(buffer)
    }
}
