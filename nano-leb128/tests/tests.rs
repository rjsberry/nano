use nano_leb128::{LEB128DecodeError, LEB128EncodeError, SLEB128, ULEB128};

use quickcheck_macros::quickcheck;

#[quickcheck]
fn qc_sleb128(val: i64) -> bool {
    let mut buf = [0; 10];

    let n0 = SLEB128::from(val).write_into(&mut buf).expect("write");
    let (result, n1) = SLEB128::read_from(&buf).expect("read");

    i64::from(result) == val && n0 == n1
}

#[quickcheck]
fn qc_uleb128(val: u64) -> bool {
    let mut buf = [0; 10];

    let n0 = ULEB128::from(val).write_into(&mut buf).expect("write");
    let (result, n1) = ULEB128::read_from(&buf).expect("read");

    u64::from(result) == val && n0 == n1
}

#[cfg(feature = "std_io_extra")]
#[quickcheck]
fn qc_sleb128_std_io(val: i64) -> bool {
    let mut buf = Vec::with_capacity(10);

    let n0 = SLEB128::from(val)
        .write_into_std_io(&mut buf)
        .expect("write");

    let (result, n1) = SLEB128::read_from_std_io(&*buf).expect("read");

    i64::from(result) == val && n0 == n1
}

#[cfg(feature = "std_io_extra")]
#[quickcheck]
fn qc_uleb128_std_io(val: u64) -> bool {
    let mut buf = Vec::with_capacity(10);

    let n0 = ULEB128::from(val)
        .write_into_std_io(&mut buf)
        .expect("write");

    let (result, n1) = ULEB128::read_from_std_io(&*buf).expect("read");

    u64::from(result) == val && n0 == n1
}

#[cfg(feature = "byteio_extra")]
#[quickcheck]
fn qc_sleb128_byteio(val: i64) -> bool {
    let mut buf = Vec::with_capacity(10);

    let n0 = SLEB128::from(val)
        .write_into_byteio(&mut buf)
        .expect("write");

    let (result, n1) = SLEB128::read_from_byteio(&*buf).expect("read");

    i64::from(result) == val && n0 == n1
}

#[cfg(feature = "byteio_extra")]
#[quickcheck]
fn qc_uleb128_byteio(val: u64) -> bool {
    let mut buf = Vec::with_capacity(10);

    let n0 = ULEB128::from(val)
        .write_into_byteio(&mut buf)
        .expect("write");

    let (result, n1) = ULEB128::read_from_byteio(&*buf).expect("read");

    u64::from(result) == val && n0 == n1
}

#[test]
fn sleb128_decode_buffer_overflow() {
    let buf = [0x80];

    assert_eq!(
        SLEB128::read_from(&buf).unwrap_err(),
        LEB128DecodeError::BufferOverflow
    );
}

#[test]
fn uleb128_decode_buffer_overflow() {
    let buf = [0x80];

    assert_eq!(
        ULEB128::read_from(&buf).unwrap_err(),
        LEB128DecodeError::BufferOverflow
    );
}

#[test]
fn sleb128_decode_integer_overflow() {
    let buf = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];

    assert_eq!(
        SLEB128::read_from(&buf).unwrap_err(),
        LEB128DecodeError::IntegerOverflow
    );
}

#[test]
fn uleb128_decode_integer_overflow() {
    let buf = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02];

    assert_eq!(
        ULEB128::read_from(&buf).unwrap_err(),
        LEB128DecodeError::IntegerOverflow
    );
}

#[test]
fn sleb128_encode_buffer_overflow() {
    let val = i64::max_value();
    let mut buf = [0; 9];

    assert_eq!(
        SLEB128::from(val).write_into(&mut buf).unwrap_err(),
        LEB128EncodeError::BufferOverflow
    );
}

#[test]
fn uleb128_encode_buffer_overflow() {
    let val = u64::max_value();
    let mut buf = [0; 9];

    assert_eq!(
        ULEB128::from(val).write_into(&mut buf).unwrap_err(),
        LEB128EncodeError::BufferOverflow
    );
}
