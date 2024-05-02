use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_write::MByteBufferWrite,
    error::{MByteBufferError, Result},
};
use std::cell::{Cell, RefCell};

impl<T: MByteBufferWrite + Copy> MByteBufferWrite for Cell<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.get().write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.get().write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.get().write_to_mbuffer_be(buffer)
    }
}

impl<T: MByteBufferWrite + ?Sized> MByteBufferWrite for RefCell<T> {
    #[inline]
    fn write_to_mbuffer(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| MByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_mbuffer(buffer)
    }

    #[inline]
    fn write_to_mbuffer_le(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| MByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_mbuffer_le(buffer)
    }

    #[inline]
    fn write_to_mbuffer_be(&self, buffer: &mut MByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| MByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_mbuffer_be(buffer)
    }
}
