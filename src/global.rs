//! Global variable abstraction
//!
//! # Example
//! ```
//! static MY_GLOBAL: atmega32u4_hal::Global<u64> = atmega32u4_hal::Global::new();
//!
//! fn main() {
//!     MY_GLOBAL.set(0xC0FFEE);
//!
//!     loop { }
//! }
//!
//! interrupt!(INT1, int1_isr);
//! fn int1_isr() {
//!     MY_GLOBAL.get(|v| {
//!         // Do something
//!     }).expect("Interrupt fired before initialisation!");
//! }
//! ```
use atmega32u4;
use core::cell;

/// A global variable store
///
/// Safe abstraction for global variables
///
/// # Example
/// ```
/// static MY_GLOBAL: atmega32u4_hal::Global<u64> = atmega32u4_hal::Global::new();
///
/// fn main() {
///     MY_GLOBAL.set(0xC0FFEE);
///
///     loop { }
/// }
///
/// interrupt!(INT1, int1_isr);
/// fn int1_isr() {
///     MY_GLOBAL.get(|v| {
///         // Do something
///     }).expect("Interrupt fired before initialisation!");
/// }
/// ```
pub struct Global<T>(cell::UnsafeCell<Option<T>>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    /// Create a new global variable
    pub const fn new() -> Global<T> {
        Global(cell::UnsafeCell::new(None))
    }

    /// Set this global to some value
    ///
    /// Used for initialization
    pub fn set(&self, val: T) {
        atmega32u4::interrupt::free(|_| unsafe {
            *self.0.get() = Some(val);
        })
    }

    /// Get the value of this global
    ///
    /// Will execute `f` with the value of the global if the global
    /// has been initialized.  If it hasn't been, return `Err(())`.
    ///
    /// While the closure is executed, interrupts are disabled.
    pub fn get<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> Result<R, ()> {
        atmega32u4::interrupt::free(|_| {
            let val = unsafe { &mut *self.0.get() };
            if let &mut Some(ref mut v) = val {
                Ok(f(v))
            } else {
                Err(())
            }
        })
    }
}
