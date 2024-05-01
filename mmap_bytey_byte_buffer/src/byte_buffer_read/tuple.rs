use crate::{
    byte_buffer::MByteBuffer,
    byte_buffer_read::MByteBufferRead,
    error::{MByteBufferError, Result},
};

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T: MByteBufferRead),+> MByteBufferRead for ($($T,)+)
            {
                #[inline]
                fn read_from_buffer(buffer: &mut MByteBuffer) -> Result<Self> {
                    Ok(($(match buffer.read::<$T>() {
                        Ok(v) => v,
                        Err(e) => return Err(MByteBufferError::OtherError {
                            error: format!("{} occured at tuple read location {}", e, $idx),
                        })
                    },)+))
                }

                #[inline]
                fn read_from_buffer_le(buffer: &mut MByteBuffer) -> Result<Self> {
                    Ok(($(match buffer.read_le::<$T>() {
                        Ok(v) => v,
                        Err(e) => return Err(MByteBufferError::OtherError {
                            error: format!("{} occured at tuple read location {}", e, $idx),
                        })
                    },)+))
                }

                #[inline]
                fn read_from_buffer_be(buffer: &mut MByteBuffer) -> Result<Self> {
                    Ok(($(match buffer.read_be::<$T>() {
                        Ok(v) => v,
                        Err(e) => return Err(MByteBufferError::OtherError {
                            error: format!("{} occured at tuple read location {}", e, $idx),
                        })
                    },)+))
                }
            }
        )+
    }
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
