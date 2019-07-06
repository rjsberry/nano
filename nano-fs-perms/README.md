<h1 align="center">nano: fs perms</h1>
<div align="center">
  <strong>
  POSIX filesystem access permissions.
  </strong>
</div>

<br />

## Usage

```rust
use nano_fs_perms::Perms;

let perms = Perms::OWNER_READ | Perms::OWNER_WRITE | Perms::GROUP_READ | Perms::OTHERS_READ;
assert_eq!(perms.to_string(), "rw-r--r--");

let perms: u32 = perms.into();
assert_eq!(perms, 0o644);
```

## License

This project is dual-licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
