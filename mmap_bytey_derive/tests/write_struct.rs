use mmap_bytey::MByteBuffer;
use mmap_bytey_derive::MByteBufferWrite;

#[test]
fn test_struct_named_write() {
    #[derive(MByteBufferWrite)]
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

    buffer.write(val).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(
        [
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(19).unwrap()
    );
}

#[test]
fn test_struct_unnamed_write() {
    #[derive(MByteBufferWrite)]
    struct Test(u16, u8, i64, usize);

    let mut buffer = MByteBuffer::new().unwrap();
    let val = Test(128, 255, -255, usize::MAX);

    buffer.write(val).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(
        [
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(19).unwrap()
    );
}
