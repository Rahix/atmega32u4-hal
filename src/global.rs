use atmega32u4;
use core::cell;

pub struct Global<T>(cell::UnsafeCell<Option<T>>);

unsafe impl<T> Sync for Global<T> { }

impl<T> Global<T> {
    pub const fn new() -> Global<T> {
        Global(cell::UnsafeCell::new(None))
    }

    pub fn set(&self, val: T) {
        atmega32u4::interrupt::free(|_| {
            unsafe {
                *self.0.get() = Some(val);
            }
        })
    }

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
