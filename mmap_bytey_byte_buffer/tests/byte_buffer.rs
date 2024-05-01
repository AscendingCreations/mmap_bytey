use mmap_bytey_byte_buffer::byte_buffer::MByteBuffer;
use mmap_bytey_byte_buffer::error::MByteBufferError;

fn is_error(x: bool) -> bool {
    x
}

#[test]
fn test_max_capacity() {
    let mut buffer = MByteBuffer::new().unwrap();

    let bytes = vec![0; 1028];
    let ret = buffer.write(bytes);
    assert!(ret.is_err());

    match ret.err().unwrap() {
        MByteBufferError::MaxCapacity => {}
        _ => panic!("did not get correct error!"),
    }
}

#[test]
fn test_capacity() {
    let buffer = MByteBuffer::new().unwrap();

    assert_eq!(buffer.capacity(), 1024);
}

#[test]
fn test_write_slice_capacity() {
    let mut buffer = MByteBuffer::new().unwrap();
    let i: u64 = u64::MAX / 2;
    let i2: u16 = u16::MAX / 2;

    assert_eq!(buffer.capacity(), 1024);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.cursor(), 8);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.cursor(), 10);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.cursor(), 12);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.cursor(), 14);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.cursor(), 16);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.cursor(), 18);
}

#[test]
fn test_write_slice_length() {
    let mut buffer = MByteBuffer::new().unwrap();
    let i: u64 = u64::MAX / 2;
    let i2: u32 = u32::MAX / 2;

    assert_eq!(buffer.length(), 0);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.length(), 8);

    let _ = buffer.move_cursor(6);
    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.length(), 10);
}

#[test]
fn test_write_slice_cursor() {
    let mut buffer = MByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    assert_eq!(buffer.cursor(), 0);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.cursor(), 4);
}

#[test]
fn test_read_slice_bounds() {
    let mut buffer = MByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    let _ = buffer.write(i);
    let _ = buffer.move_cursor(0);
    let ret = buffer.read_slice(5);

    assert!(ret.is_err());

    match ret.err().unwrap() {
        MByteBufferError::ReadOutOfBounds { length, start, end } => {
            assert_eq!(length, 4);
            assert_eq!(start, 0);
            assert_eq!(end, 5);
        }
        _ => assert!(is_error(false)),
    }
}

#[test]
fn test_read_slice_cursor() {
    let mut buffer = MByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    let _ = buffer.write(i);
    let _ = buffer.move_cursor(0);
    let _ = buffer.read_slice(2);

    assert_eq!(buffer.cursor(), 2);
}

#[test]
fn test_read_to_buffer() {
    let mut buffer = MByteBuffer::new().unwrap();
    let value: u64 = 64;

    let _ = buffer.write(value);
    let _ = buffer.move_cursor(0);

    let mut new_buffer = buffer.read_to_buffer(std::mem::size_of::<u64>()).unwrap();
    let read_arr = new_buffer.read::<u64>().unwrap();

    assert_eq!(read_arr, value);
    assert_eq!(buffer.cursor(), 8);
    assert_eq!(new_buffer.capacity(), buffer.capacity());
}
