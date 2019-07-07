<h1 align="center">nano: leb128</h1>
<div align="center">
  <strong>
  Little endian base 128 variable-length code compression.
  </strong>
</div>

<br />

## Usage

Signed LEB128 compression/decompression:

```rust
use nano_leb128::SLEB128;

fn rand_i64() -> i64 {
    // ...
}

let mut buf = [0; 10];
let value = rand_i64();

// Compress the value into the buffer.
let len = SLEB128::from(value).write_into(&mut buf).unwrap();

// Decompress the value from the buffer.
let (decompressed, _len) = SLEB128::read_from(&buf[..len]).unwrap();

assert_eq!(i64::from(decompressed), value);
```

Unsigned LEB128 compression/decompression:

```rust
use nano_leb128::ULEB128;

fn rand_u64() -> u64 {
    // ...
}

let mut buf = [0; 10];
let value = rand_u64();

// Compress the value into the buffer.
let len = ULEB128::from(value).write_into(&mut buf).unwrap();

// Decompress the value from the buffer.
let (decompressed, _len) = ULEB128::read_from(&buf[..len]).unwrap();

assert_eq!(u64::from(decompressed), value);
```

## Features

* `std` (enabled by default)

   This enables extensions that are only available with the Rust standard
   library.

* `std_io_ext`

  Adds methods for reading/writing LEB128 compressed values from
  implementors of the traits in [`std::io`]. This feature requires the
  `std` feature and will automatically enable it if it is not already
  enabled.

* `byteio_ext`

  Adds methods for reading/writing LEB128 compressed values from
  implementors of the traits in [`byteio`]. This feature does not require
  the `std` feature.

[`std::io`]: https://doc.rust-lang.org/std/io/index.html
[`byteio`]: https://docs.rs/byteio

## License

This project is dual-licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
