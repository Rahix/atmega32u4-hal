use hal::blocking::delay;
use core::marker;

pub struct Delay<SPEED> {
    _speed: marker::PhantomData<SPEED>,
}

impl<SPEED> Delay<SPEED> {
    pub fn new() -> Delay<SPEED> {
        Delay {
            _speed: marker::PhantomData,
        }
    }
}

pub struct MHz24;
pub struct MHz20;
pub struct MHz16;
pub struct MHz12;
pub struct MHz8;
pub struct MHz1;

// based on https://github.com/arduino/ArduinoCore-avr/blob/master/cores/arduino/wiring.c

#[allow(unused_assignments)]
fn busy_loop(mut us: u16) {
    unsafe {
        asm!("1: sbiw $0,1\n\tbrne 1b"
             : "=w"(us)
             : "0"(us)
             :
             : "volatile"
             );
    }
}

impl delay::DelayUs<u16> for Delay<MHz24> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 24 MHz clock for the aventurous ones, trying to overclock

        // zero delay fix
        if us == 0 { return } // = 3 cycles, (4 when true)

        // the following loop takes a 1/6 of a microsecond (4 cycles)
        // per iteration, so execute it six times for each microsecond of
        // delay requested.
        us *= 6; // x6 us, = 7 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 22 (24) cycles above, remove 5, (5*4=20)
        // us is at least 6 so we can substract 5
        us -= 5; //=2 cycles

        busy_loop(us);
    }
}

impl delay::DelayUs<u16> for Delay<MHz20> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 20 MHz clock on rare Arduino boards

        // for a one-microsecond delay, simply return.  the overhead
        // of the function call takes 18 (20) cycles, which is 1us
        unsafe {
            asm!("nop\nnop\nnop\nnop" :::: "volatile");
        } //just waiting 4 cycles

        if us <= 1 { return } // = 3 cycles, (4 when true)

        // the following loop takes a 1/5 of a microsecond (4 cycles)
        // per iteration, so execute it five times for each microsecond of
        // delay requested.
        us = (us << 2) + us; // x5 us, = 7 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 26 (28) cycles above, remove 7, (7*4=28)
        // us is at least 10 so we can substract 7
        us -= 7; // 2 cycles

        busy_loop(us);
    }
}

impl delay::DelayUs<u16> for Delay<MHz16> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 16 MHz clock on most Arduino boards

        // for a one-microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 1us
        if us <= 1 { return } // = 3 cycles, (4 when true)

        // the following loop takes 1/4 of a microsecond (4 cycles)
        // per iteration, so execute it four times for each microsecond of
        // delay requested.
        us <<= 2; // x4 us, = 4 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 19 (21) cycles above, remove 5, (5*4=20)
        // us is at least 8 so we can substract 5
        us -= 5; // = 2 cycles,

        busy_loop(us);
    }
}

impl delay::DelayUs<u16> for Delay<MHz12> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 12 MHz clock if somebody is working with USB

        // for a 1 microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 1.5us
        if us <= 1 { return } // = 3 cycles, (4 when true)

        // the following loop takes 1/3 of a microsecond (4 cycles)
        // per iteration, so execute it three times for each microsecond of
        // delay requested.
        us = (us << 1) + us; // x3 us, = 5 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 20 (22) cycles above, remove 5, (5*4=20)
        // us is at least 6 so we can substract 5
        us -= 5; //2 cycles

        busy_loop(us);
    }
}

impl delay::DelayUs<u16> for Delay<MHz8> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 8 MHz internal clock

        // for a 1 and 2 microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 2us
        if us <= 2 { return } // = 3 cycles, (4 when true)

        // the following loop takes 1/2 of a microsecond (4 cycles)
        // per iteration, so execute it twice for each microsecond of
        // delay requested.
        us <<= 1; //x2 us, = 2 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 17 (19) cycles above, remove 4, (4*4=16)
        // us is at least 6 so we can substract 4
        us -= 4; // = 2 cycles

        busy_loop(us);
    }
}

impl delay::DelayUs<u16> for Delay<MHz1> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 1 MHz internal clock (default settings for common Atmega microcontrollers)

        // the overhead of the function calls is 14 (16) cycles
        if us <= 16 { return } //= 3 cycles, (4 when true)
        if us <= 25 { return } //= 3 cycles, (4 when true), (must be at least 25 if we want to substract 22)

        // compensate for the time taken by the preceeding and next commands (about 22 cycles)
        us -= 22; // = 2 cycles
        // the following loop takes 4 microseconds (4 cycles)
        // per iteration, so execute it us/4 times
        // us is at least 4, divided by 4 gives us 1 (no zero delay bug)
        us >>= 2; // us div 4, = 4 cycles

        busy_loop(us);
    }
}

impl<SPEED> delay::DelayUs<u8> for Delay<SPEED>
    where Delay<SPEED>: delay::DelayUs<u16>
{
    fn delay_us(&mut self, us: u8) {
        delay::DelayUs::<u16>::delay_us(self, us as u16);
    }
}

impl<SPEED> delay::DelayUs<u32> for Delay<SPEED>
    where Delay<SPEED>: delay::DelayUs<u16>
{
    fn delay_us(&mut self, us: u32) {
        for _ in 0..(us >> 12) {
            delay::DelayUs::<u16>::delay_us(self, 0xfff);
        }
    }
}

impl<SPEED> delay::DelayMs<u16> for Delay<SPEED>
    where Delay<SPEED>: delay::DelayUs<u32>
{
    fn delay_ms(&mut self, ms: u16) {
        delay::DelayUs::<u32>::delay_us(self, ms as u32 * 1000);
    }
}
