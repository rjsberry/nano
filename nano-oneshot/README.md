<h1 align="center">nano: oneshot</h1>
<div align="center">
  <strong>
  A one-shot channel.
  </strong>
</div>

<br />

## Usage

```rust
let (s, r) = ::nano_oneshot::channel();

let _ = s.send("hello");
assert_eq!(r.recv().unwrap(), "hello");
```

## License

This project is dual-licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
