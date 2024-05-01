use mmap_bytey::MByteBuffer;
use mmap_bytey_derive::{MByteBufferRead, MByteBufferWrite};

#[test]
fn test_struct_named_read() {
    #[derive(MByteBufferWrite, MByteBufferRead, PartialEq, Debug)]
    struct Test {
        a: u16,
        b: u8,
        c: i64,
        d: usize,
    }

    let mut buffer = MByteBuffer::new().unwrap();
    let val = Test {
        a: 128,
        b: 255,
        c: -255,
        d: usize::MAX,
    };

    buffer.write(&val).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(val, buffer.read::<Test>().unwrap());
}

#[test]
fn test_struct_unnamed_read() {
    #[derive(MByteBufferWrite, MByteBufferRead, PartialEq, Debug)]
    struct Test(u16, u8, i64, usize);

    let mut buffer = MByteBuffer::new().unwrap();
    let val = Test(128, 255, -255, usize::MAX);

    buffer.write(&val).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(val, buffer.read::<Test>().unwrap());
}
