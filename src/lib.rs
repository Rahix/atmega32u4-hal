#![feature(asm, const_fn, const_unsafe_cell_new)]
#![no_std]

extern crate embedded_hal as hal;
extern crate atmega32u4;

pub mod port;
pub mod delay;
pub mod prelude;
pub mod timer;

pub mod global;
pub use global::Global;
