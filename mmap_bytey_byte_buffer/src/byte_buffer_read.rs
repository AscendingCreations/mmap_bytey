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
mod result;
mod string;
mod time;
mod tuple;

pub trait MByteBufferRead {
    fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Self>
    where
        Self: Sized;
    fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Self>
    where
        Self: Sized;
    fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_byte_buffer_read_types {
    ($($type:ty),*) => {
        $(
            impl MByteBufferRead for $type {
                fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_ne_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }

                fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_le_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }

                fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_be_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }
            }
        )*
    }
}

impl_byte_buffer_read_types!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
