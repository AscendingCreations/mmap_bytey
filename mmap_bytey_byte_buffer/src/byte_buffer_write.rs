use crate::{byte_buffer::MByteBuffer, error::Result};

mod arrays;
mod bound;
mod boxed;
mod byte;
mod cell;
mod char;
mod cow;
mod num;
mod option;
mod phantom;
mod range;
mod result;
mod string;
mod time;
mod tuple;

pub trait MByteBufferWrite {
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()>;
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()>;
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()>;
}

macro_rules! impl_byte_buffer_write_types {
    ($($type:ty),*) => {
        $(
            impl MByteBufferWrite for $type {
                #[inline]
                fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "little")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "little"))]
                    unsafe {
                        let o = self.to_le_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "big")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "big"))]
                    unsafe {
                        let o = self.to_be_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }
            }

            impl MByteBufferWrite for &$type {
                #[inline]
                fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "little")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "little"))]
                    unsafe {
                        let o = self.to_le_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "big")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "big"))]
                    unsafe {
                        let o = self.to_be_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }
            }
        )*
    }
}

impl_byte_buffer_write_types!(u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
