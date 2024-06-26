pub type Result<T> = std::result::Result<T, MByteBufferError>;

#[derive(thiserror::Error, Debug)]
pub enum MByteBufferError {
    #[error("Capacity cannot be greater than {} bytes", isize::MAX)]
    MaxCapacity,

    #[error("Failed to allocate {size} bytes")]
    AllocationFailure { size: usize },

    #[error("Failed to Generate Layout for Capacity {size}")]
    LayoutFailure { size: usize },

    #[error("Capacity cannot be less than 1 byte")]
    MinCapacity,

    #[error("Cursor out of bounds: {cursor} >= {length}")]
    CursorOutOfBounds { length: usize, cursor: usize },

    #[error("Read out of bounds: {start}..{end} >= {length}")]
    ReadOutOfBounds {
        length: usize,
        start: usize,
        end: usize,
    },

    #[error("Length out of bounds: {new} >= {current}")]
    LengthOutOfBounds { current: usize, new: usize },

    #[error("Other Error: {error}")]
    OtherError { error: String },

    #[error(transparent)]
    UnicodeError(#[from] std::str::Utf8Error),

    #[error("RefCellAlreadyBorrowed: {error} for type: {type_name}")]
    RefCellAlreadyBorrowed {
        /// The inner borrow error
        error: String,
        /// the type name of the RefCell being encoded that is currently borrowed.
        type_name: &'static str,
    },
    #[error("The NonZero Type is a 0. Maybe you have it set to the wrong position in the struct?")]
    NonZeroIsZero,
    #[error(
        "the value can not be a char. Maybe you have it set to the wrong position in the struct?"
    )]
    NotAChar,
    #[error(transparent)]
    Mmap(#[from] mmap_rs::Error),
}
