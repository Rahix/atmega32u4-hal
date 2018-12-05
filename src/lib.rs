//! # Hardware Abstraction Layer for ATmega32U4
//!
//! Built ontop of [`atmega32u4`](https://crates.io/crates/atmega32u4) for register definitions
//! and [`embedded-hal`](https://crates.io/crates/embedded-hal) for hardware abstractions.
//!
//! The following features are implemented as of now:
//!
//! * Port digital IO: Digital input and output using `embedded-hal` traits.
//!   Examples can be found in the [port] module.
//! * Port PWM: Using the 4 builtin timers, pwm can be configured for a few
//!   pins.  For more info, take a look at the [timer] module.
//! * Delay: Delay using a busy loop.  Implementation taken from the ArduinoCore
//!   library. Examples in the [delay] module.
//!
//! ## Easy Globals
//! Because a lot of times you need to exchange data between your application code
//! and interrupt handlers, this crate contains a safe abstraction for globals.  While
//! a global is accessed interrupts are disabled, so you don't need to worry about
//! data races.  For more info, take a look at the [global] module.
#![feature(asm, const_fn)]
#![cfg_attr(feature = "docs", feature(extern_prelude))]
#![no_std]
#![deny(missing_docs)]

pub extern crate embedded_hal as hal;
extern crate atmega32u4;

pub mod port;
pub mod delay;
pub mod prelude;
pub mod timer;

pub mod global;
pub use global::Global;
