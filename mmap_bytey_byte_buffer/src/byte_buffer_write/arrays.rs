use crate::{byte_buffer::MByteBuffer, byte_buffer_write::MByteBufferWrite, error::Result};

impl<T: MByteBufferWrite, const N: usize> MByteBufferWrite for [T; N] {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer(buffer)?;

        for e in self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_le(buffer)?;

        for e in self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_be(buffer)?;

        for e in self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for [T] {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer(buffer)?;

        for e in self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_le(buffer)?;

        for e in self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_be(buffer)?;

        for e in self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for Vec<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer(buffer)?;

        for e in self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_le(buffer)?;

        for e in self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_be(buffer)?;

        for e in self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}

impl<T: MByteBufferWrite> MByteBufferWrite for &Vec<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer(buffer)?;

        for e in *self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_le(buffer)?;

        for e in *self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        (self.len() as u64).write_to_buffer_be(buffer)?;

        for e in *self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}
