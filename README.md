# esp32c3

## Important:

This fork was created to enable rapid prototyping. SVD patching has been enabled via [svdtools](https://github.com/stm32-rs/svdtools) so that changes can be made to the PAC as needed. As patches accumulate, I will eventually upstream them to the official Espressif SVDs and make the corresponding updates to this repository.

---

[![crates.io](https://img.shields.io/crates/v/esp32c3.svg)](https://crates.io/crates/esp32c3)

A peripheral access crate the ESP32-C3. See the [`svd2rust repo`](https://github.com/rust-embedded/svd2rust) for more infomation on how to use this crate.

If you find any problems with the SVD please report them upstream to the [espressif/svd](https://github.com/espressif/svd) repository so they can be corrected.

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!

## [`Documentation`](https://docs.rs/esp32c3)

## Building

Required dependencies:

- [form](https://crates.io/crates/form)
- [svd2rust](https://github.com/rust-embedded/svd2rust)

```
$ make
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
