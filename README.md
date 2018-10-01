# `ATmega32U4-HAL` [![crates.io page](http://meritbadge.herokuapp.com/atmega32u4-hal)](https://crates.io/crates/atmega32u4-hal) [![docs.rs](https://docs.rs/atmega32u4-hal/badge.svg)](https://docs.rs/atmega32u4-hal)

Hardware Abstraction Layer for ATmega32U4.  Built ontop of [`atmega32u4`](https://crates.io/crates/atmega32u4) for
register definitions and [`embedded-hal`](https://crates.io/crates/embedded-hal) for hardware abstractions.

The following features are implemented as of now:

- [x] Port digital IO: Digital input and output using `embedded-hal` traits.
- [x] Port PWM: Using the 4 builtin timers, PWM can be configured for a few pins. Namely
      `PB5`, `PB6`, `PB7`, `PC6`, `PC7` & `PD0`.
- [x] Delay: Delay using a busy loop.  Implementation taken from the ArduinoCore library.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
