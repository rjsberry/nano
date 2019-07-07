//! # nano-leb128
//!
//! Little endian base 128 variable-length code compression.
//!
//! # Usage
//!
//! Signed LEB128 compression/decompression:
//!
//! ```
//! use nano_leb128::SLEB128;
//!
//! fn rand_i64() -> i64 {
//!     // ...
//! #   0
//! }
//!
//! let mut buf = [0; 10];
//! let value = rand_i64();
//!
//! // Compress the value into the buffer.
//! let len = SLEB128::from(value).write_into(&mut buf).unwrap();
//!
//! // Decompress the value from the buffer.
//! let (decompressed, _len) = SLEB128::read_from(&buf[..len]).unwrap();
//!
//! assert_eq!(i64::from(decompressed), value);
//! ```
//!
//! Unsigned LEB128 compression/decompression:
//!
//! ```
//! use nano_leb128::ULEB128;
//!
//! fn rand_u64() -> u64 {
//!     // ...
//! #   0
//! }
//!
//! let mut buf = [0; 10];
//! let value = rand_u64();
//!
//! // Compress the value into the buffer.
//! let len = ULEB128::from(value).write_into(&mut buf).unwrap();
//!
//! // Decompress the value from the buffer.
//! let (decompressed, _len) = ULEB128::read_from(&buf[..len]).unwrap();
//!
//! assert_eq!(u64::from(decompressed), value);
//! ```
//!
//! # Features
//!
//! * `std` (enabled by default)
//!
//!    This enables extensions that are only available with the Rust standard
//!    library.
//!
//! * `std_io_ext`
//!
//!   Adds methods for reading/writing LEB128 compressed values from
//!   implementors of the traits in [`std::io`]. This feature requires the
//!   `std` feature and will automatically enable it if it is not already
//!   enabled.
//!
//! * `byteio_ext`
//!
//!   Adds methods for reading/writing LEB128 compressed values from
//!   implementors of the traits in [`byteio`]. This feature does not require
//!   the `std` feature.
//!
//! [`std::io`]: https://doc.rust-lang.org/std/io/index.html
//! [`byteio`]: https://docs.rs/byteio

#![no_std]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

#[cfg(feature = "std")]
extern crate std;

use core::mem;

use byteio::{ReadBytes, ReadBytesExt, WriteBytes, WriteBytesExt};

/// A value that can be (de)serialized using _signed_ LEB128 variable length
/// compression.
///
/// # Examples
///
/// Deserializing a value that was serialized using signed LEB128 variable
/// length compression:
///
/// ```
/// use nano_leb128::SLEB128;
///
/// let buf = [0xC0, 0xBB, 0x78];
///
/// let (val, len) = SLEB128::read_from(&buf).unwrap();
///
/// assert_eq!(i64::from(val), -123456);
/// assert_eq!(len, 3);
/// ```
///
/// Serializing a value using signed LEB128 variable length compression:
///
/// ```
/// use nano_leb128::SLEB128;
///
/// let mut buf = [0; 3];
///
/// assert_eq!(SLEB128::from(-123456).write_into(&mut buf).unwrap(), 3);
/// assert_eq!(buf, [0xC0, 0xBB, 0x78]);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SLEB128(i64);

impl From<SLEB128> for i64 {
    fn from(sleb128: SLEB128) -> Self {
        sleb128.0
    }
}

impl From<i64> for SLEB128 {
    fn from(val: i64) -> Self {
        Self(val)
    }
}

impl SLEB128 {
    /// Attempts to read a signed LEB128 compressed value from a buffer.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    pub fn read_from(buf: &[u8]) -> Result<(Self, usize), LEB128DecodeError> {
        <Self as LEB128>::read_from(buf)
    }

    /// Attempts to write a value into a buffer using signed LEB128
    /// compression.
    ///
    /// On success this will return the number of bytes that were written.
    pub fn write_into(self, buf: &mut [u8]) -> Result<usize, LEB128EncodeError> {
        <Self as LEB128>::write_into(self, buf)
    }

    /// Attempts to read a signed LEB128 compressed value from an implementor
    /// of [`std::io::Read`].
    ///
    /// **Note**: Requires the feature `std_io_ext`.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    ///
    /// [`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    #[cfg(feature = "std_io_ext")]
    pub fn read_from_std_io<R: ::std::io::Read>(reader: R) -> ::std::io::Result<(Self, usize)> {
        <Self as LEB128>::read_from_std_io(reader)
    }

    /// Attempts to write a value into an implementor of [`std::io::Write`]
    /// using signed LEB128 compression.
    ///
    /// **Note**: Requires the feature `std_io_ext`.
    ///
    /// On success this will return the number of bytes that were written.
    ///
    /// [`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    #[cfg(feature = "std_io_ext")]
    pub fn write_into_std_io<W: ::std::io::Write>(self, writer: W) -> ::std::io::Result<usize> {
        <Self as LEB128>::write_into_std_io(self, writer)
    }

    /// Attempts to read a signed LEB128 compressed value from an implementor
    /// of [`byteio::ReadBytes`].
    ///
    /// **Note**: Requires the feature `byteio_ext`.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    ///
    /// [`byteio::ReadBytes`]: https://docs.rs/byteio/latest/trait.ReadBytes.html
    #[cfg(feature = "byteio_ext")]
    pub fn read_from_byteio<'a, R: ReadBytes<'a>>(
        reader: R,
    ) -> Result<(Self, usize), LEB128DecodeError> {
        <Self as LEB128>::read_from_byteio(reader)
    }

    /// Attempts to write a value into an implementor of [`byteio::WriteBytes`]
    /// using signed LEB128 compression.
    ///
    /// **Note**: Requires the feature `byteio_ext`.
    ///
    /// On success this will return the number of bytes that were written.
    ///
    /// [`byteio::WriteBytes`]: https://docs.rs/byteio/latest/trait.WriteBytes.html
    #[cfg(feature = "byteio_ext")]
    pub fn write_into_byteio<W: WriteBytes>(self, writer: W) -> Result<usize, LEB128EncodeError> {
        <Self as LEB128>::write_into_byteio(self, writer)
    }
}

/// A value that can be (de)serialized using _unsigned_ LEB128 variable length
/// compression.
///
/// # Examples
///
/// Deserializing a value that was serialized using unsigned LEB128 variable
/// length compression:
///
/// ```
/// use nano_leb128::ULEB128;
///
/// let buf = [0xE5, 0x8E, 0x26];
///
/// let (val, len) = ULEB128::read_from(&buf).unwrap();
///
/// assert_eq!(u64::from(val), 624485);
/// assert_eq!(len, 3);
/// ```
///
/// Serializing a value using unsigned LEB128 variable length compression:
///
/// ```
/// use nano_leb128::ULEB128;
///
/// let mut buf = [0; 3];
///
/// assert_eq!(ULEB128::from(624485).write_into(&mut buf).unwrap(), 3);
/// assert_eq!(buf, [0xE5, 0x8E, 0x26]);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ULEB128(u64);

impl From<ULEB128> for u64 {
    fn from(uleb128: ULEB128) -> Self {
        uleb128.0
    }
}

impl From<u64> for ULEB128 {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl ULEB128 {
    /// Attempts to read an unsigned LEB128 compressed value from a buffer.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    pub fn read_from(buf: &[u8]) -> Result<(Self, usize), LEB128DecodeError> {
        <Self as LEB128>::read_from(buf)
    }

    /// Attempts to write a value into a buffer using unsigned LEB128
    /// compression.
    ///
    /// On success this will return the number of bytes that were written.
    pub fn write_into(self, buf: &mut [u8]) -> Result<usize, LEB128EncodeError> {
        <Self as LEB128>::write_into(self, buf)
    }

    /// Attempts to read an unsigned LEB128 compressed value from an
    /// implementor of [`std::io::Read`].
    ///
    /// **Note**: Requires the feature `std_io_ext`.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    ///
    /// [`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    #[cfg(feature = "std_io_ext")]
    pub fn read_from_std_io<R: ::std::io::Read>(reader: R) -> ::std::io::Result<(Self, usize)> {
        <Self as LEB128>::read_from_std_io(reader)
    }

    /// Attempts to write a value into an implementor of [`std::io::Write`]
    /// using unsigned LEB128 compression.
    ///
    /// **Note**: Requires the feature `std_io_ext`.
    ///
    /// On success this will return the number of bytes that were written.
    ///
    /// [`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    #[cfg(feature = "std_io_ext")]
    pub fn write_into_std_io<W: ::std::io::Write>(self, writer: W) -> ::std::io::Result<usize> {
        <Self as LEB128>::write_into_std_io(self, writer)
    }

    /// Attempts to read an unsigned LEB128 compressed value from an
    /// implementor of [`byteio::ReadBytes`].
    ///
    /// **Note**: Requires the feature `byteio_ext`.
    ///
    /// On success this will return the decompressed value and the number of
    /// bytes that were read.
    ///
    /// [`byteio::ReadBytes`]: https://docs.rs/byteio/latest/trait.ReadBytes.html
    #[cfg(feature = "byteio_ext")]
    pub fn read_from_byteio<'a, R: ReadBytes<'a>>(
        reader: R,
    ) -> Result<(Self, usize), LEB128DecodeError> {
        <Self as LEB128>::read_from_byteio(reader)
    }

    /// Attempts to write a value into an implementor of [`byteio::WriteBytes`]
    /// using unsigned LEB128 compression.
    ///
    /// **Note**: Requires the feature `byteio_ext`.
    ///
    /// On success this will return the number of bytes that were written.
    ///
    /// [`byteio::WriteBytes`]: https://docs.rs/byteio/latest/trait.WriteBytes.html
    #[cfg(feature = "byteio_ext")]
    pub fn write_into_byteio<W: WriteBytes>(self, writer: W) -> Result<usize, LEB128EncodeError> {
        <Self as LEB128>::write_into_byteio(self, writer)
    }
}

/// Errors that can occur when decoding LEB128 compressed values.
///
/// When compiled with the `std` feature this error implements
/// [`std::error::Error`], and [`Into`] for [`std::io::Error`].
///
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LEB128DecodeError {
    /// More bytes required than are available to complete the deserialization.
    BufferOverflow,
    /// The compressed value represents a larger number than can be decoded.
    IntegerOverflow,
}

#[cfg(feature = "std")]
impl From<LEB128DecodeError> for ::std::io::Error {
    fn from(err: LEB128DecodeError) -> Self {
        match err {
            LEB128DecodeError::BufferOverflow => ::std::io::ErrorKind::UnexpectedEof.into(),
            LEB128DecodeError::IntegerOverflow => ::std::io::ErrorKind::InvalidData.into(),
        }
    }
}

/// Errors that can occur when encoding values using LEB128 compression.
///
/// When compiled with the `std` feature this error implements
/// [`std::error::Error`], and [`Into`] for [`std::io::Error`].
///
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`std::io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LEB128EncodeError {
    /// More bytes required than are available to complete the serialization.
    BufferOverflow,
}

#[cfg(feature = "std")]
impl From<LEB128EncodeError> for ::std::io::Error {
    fn from(err: LEB128EncodeError) -> Self {
        match err {
            LEB128EncodeError::BufferOverflow => ::std::io::ErrorKind::UnexpectedEof.into(),
        }
    }
}

/*
 *
 * impl
 *
 */

const LEB128_HIGH_ORDER_BIT: u8 = 1 << 7;
const LEB128_SIGN_BIT: u8 = 1 << 6;

trait LEB128Decode: Sized {
    fn leb128_decode<'a, R: ReadBytes<'a>>(reader: R) -> Result<Self, LEB128DecodeError>;
}

trait LEB128Encode {
    fn leb128_encode<W: WriteBytes>(self, writer: W) -> Result<(), LEB128EncodeError>;
}

trait LEB128: LEB128Decode + LEB128Encode {
    fn read_from(buf: &[u8]) -> Result<(Self, usize), LEB128DecodeError> {
        let mut reader = ::byteio::Reader::new(buf);
        let value = Self::leb128_decode(&mut reader)?;

        Ok((value, reader.num_bytes_read()))
    }

    fn write_into(self, buf: &mut [u8]) -> Result<usize, LEB128EncodeError> {
        let mut writer = ::byteio::Writer::new(buf);
        self.leb128_encode(&mut writer)?;

        Ok(writer.num_bytes_written())
    }

    #[cfg(feature = "std_io_ext")]
    fn read_from_std_io<R: ::std::io::Read>(mut reader: R) -> ::std::io::Result<(Self, usize)> {
        let mut buf = ::std::vec::Vec::with_capacity(10);

        loop {
            let mut byte = [0];
            reader.read_exact(&mut byte)?;
            buf.push(byte[0]);

            match Self::leb128_decode(&*buf) {
                Ok(val) => {
                    return Ok((val, buf.len()));
                }
                Err(err) if err == LEB128DecodeError::IntegerOverflow => {
                    return Err(err.into());
                }
                _ => (),
            }
        }
    }

    #[cfg(feature = "std_io_ext")]
    fn write_into_std_io<W: ::std::io::Write>(self, mut writer: W) -> ::std::io::Result<usize> {
        let mut buf = ::std::vec::Vec::with_capacity(10);
        self.leb128_encode(&mut buf)?;
        writer.write_all(&buf)?;

        Ok(buf.len())
    }

    #[cfg(feature = "byteio_ext")]
    fn read_from_byteio<'a, R: ReadBytes<'a>>(
        reader: R,
    ) -> Result<(Self, usize), LEB128DecodeError> {
        let mut reader = ::byteio::Reader::new(reader);
        let value = Self::leb128_decode(&mut reader)?;

        Ok((value, reader.num_bytes_read()))
    }

    #[cfg(feature = "byteio_ext")]
    fn write_into_byteio<W: WriteBytes>(self, writer: W) -> Result<usize, LEB128EncodeError> {
        let mut writer = ::byteio::Writer::new(writer);
        self.leb128_encode(&mut writer)?;

        Ok(writer.num_bytes_written())
    }
}

impl LEB128Decode for SLEB128 {
    fn leb128_decode<'a, R: ReadBytes<'a>>(mut reader: R) -> Result<Self, LEB128DecodeError> {
        let mut result = 0;
        let mut shift = 0;

        let byte = loop {
            let byte = reader
                .try_read_u8()
                .map_err(|_| LEB128DecodeError::BufferOverflow)?;

            if shift == 63 && byte != 0x00 && byte != !LEB128_HIGH_ORDER_BIT {
                return Err(LEB128DecodeError::IntegerOverflow);
            }

            result |= i64::from(byte & !LEB128_HIGH_ORDER_BIT) << shift;
            shift += 7;

            if byte & LEB128_HIGH_ORDER_BIT == 0 {
                break byte;
            }
        };

        if shift < 8 * mem::size_of::<i64>() && (byte & LEB128_SIGN_BIT) != 0 {
            result |= !0 << shift;
        }

        Ok(Self(result))
    }
}

impl LEB128Encode for SLEB128 {
    fn leb128_encode<W: WriteBytes>(self, mut writer: W) -> Result<(), LEB128EncodeError> {
        let Self(mut value) = self;
        let mut more = true;

        while more {
            let mut byte = (value as u8) & !LEB128_HIGH_ORDER_BIT;
            value >>= 7;

            if value == 0 && (byte & LEB128_SIGN_BIT) == 0
                || value == -1 && (byte & LEB128_SIGN_BIT) != 0
            {
                more = false;
            } else {
                byte |= LEB128_HIGH_ORDER_BIT;
            }

            writer
                .try_write_u8(byte)
                .map_err(|_| LEB128EncodeError::BufferOverflow)?;
        }

        Ok(())
    }
}

impl LEB128 for SLEB128 {}

impl LEB128Decode for ULEB128 {
    fn leb128_decode<'a, R: ReadBytes<'a>>(mut reader: R) -> Result<Self, LEB128DecodeError> {
        let mut result = 0;
        let mut shift = 0;

        loop {
            let byte = reader
                .try_read_u8()
                .map_err(|_| LEB128DecodeError::BufferOverflow)?;

            if shift == 63 && byte > 1 {
                return Err(LEB128DecodeError::IntegerOverflow);
            }

            result |= u64::from(byte & !LEB128_HIGH_ORDER_BIT) << shift;

            if byte & LEB128_HIGH_ORDER_BIT == 0 {
                return Ok(Self(result));
            }

            shift += 7;
        }
    }
}

impl LEB128Encode for ULEB128 {
    fn leb128_encode<W: WriteBytes>(self, mut writer: W) -> Result<(), LEB128EncodeError> {
        let Self(mut value) = self;

        loop {
            let mut byte = (value as u8) & !LEB128_HIGH_ORDER_BIT;
            value >>= 7;

            if value != 0 {
                byte |= LEB128_HIGH_ORDER_BIT;
            }

            writer
                .try_write_u8(byte)
                .map_err(|_| LEB128EncodeError::BufferOverflow)?;

            if value == 0 {
                return Ok(());
            }
        }
    }
}

impl LEB128 for ULEB128 {}
