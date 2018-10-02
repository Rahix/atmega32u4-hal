//! Timers
//!
//! # PWM
//! `atmega32u4_hal` currently only implements timers for PWM.  Different uses
//! might get added later on.  To configure a timer for PWM, create a new corresponding
//! `Timer#Pwm` object:
//!
//! ```
//! let dp = atmega32u4::Peripherals::take().unwrap();
//! let mut pwm4 = atmega32u4_hal::timer::Timer4Pwm::new(dp.TIMER4);
//! ```
//!
//! Next up, convert your pin into a PWM output.  You can only configure PWM for pins
//! already configured as outputs:
//!
//! ```
//! let mut pin = portc.pc7.into_output(&mut portc.ddr).into_pwm(&mut pwm4);
//! ```
//!
//! ## Pins supporting PWM
//! Only the following pins support PWM:
//!
//! | Timer                | Channel | Port                | Pin     |
//! |----------------------|---------|---------------------|---------|
//! | [atmega32u4::TIMER0] | `OC0A`  | [atmega32u4::PORTB] | `PB7`   |
//! | [atmega32u4::TIMER0] | `OC0B`  | [atmega32u4::PORTD] | `PD0`   |
//! | [atmega32u4::TIMER1] | `OC1A`  | [atmega32u4::PORTB] | `PB5`   |
//! | [atmega32u4::TIMER1] | `OC1B`  | [atmega32u4::PORTB] | `PB6`   |
//! | [atmega32u4::TIMER1] | `OC1C`  | [atmega32u4::PORTB] | (`PB7`) |
//! | [atmega32u4::TIMER3] | `OC3A`  | [atmega32u4::PORTC] | `PC6`   |
//! | [atmega32u4::TIMER4] | `OC4A`  | [atmega32u4::PORTC] | `PC7`   |
//! | [atmega32u4::TIMER4] | `OC4D`  | [atmega32u4::PORTD] | `PD7`   |
//!
//! *Note*: `PB7` could technically also be PWM'd using `TIMER1` but that is
//! not yet implemented
//!
//! # Example
//! ```
//! let dp = atmega32u4::Peripherals::take().unwrap();
//!
//! // According to the manual, PC7(D13) is connected to Timer/Counter4
//! let mut pwm4 = atmega32u4_hal::timer::Timer4Pwm::new(dp.TIMER4);
//!
//! // Split portc into 8 pins
//! let mut portc = dp.PORTC.split();
//!
//! // First make the pin an output, then enable the PWM timer
//! let mut pin = portc.pc7.into_output(&mut portc.ddr).into_pwm(&mut pwm4);
//!
//! // Use the pin
//! pin.set_duty_cycle(128);
//! ```
use core::marker;
use hal;
use atmega32u4;
use port;

macro_rules! timer_impl {
    (
        Info: ($Timer:ident, $TIMER:ident, $tim:ident),
        Init: $init:block,
        Pins: [
            $(|$port:ident, $PIN:ident, $pwm:ident| ($ocr:ident, $setup:block),)+
        ]
    ) => {
        /// PWM Timer
        pub struct $Timer {
            $tim: atmega32u4::$TIMER,
        }

        impl $Timer {
            /// Initialize this PWM timer
            ///
            /// *Note*: Right now, once a timer is configured for PWM, it can't be used for
            /// anything else afterwards.
            pub fn new($tim: atmega32u4::$TIMER) -> $Timer {
                $init

                $Timer {
                    $tim: $tim,
                }
            }
        }

        $(
            impl port::$port::$PIN<port::mode::io::Output> {
                /// Make this pin a PWM pin
                ///
                /// Pin needs to be an output pin to be turned into a PWM pin.
                pub fn into_pwm(self, $pwm: &mut $Timer) -> port::$port::$PIN<port::mode::Pwm> {
                    $setup

                    port::$port::$PIN {
                        _mode: marker::PhantomData,
                    }
                }
            }

            impl hal::PwmPin for port::$port::$PIN<port::mode::Pwm> {
                type Duty = u8;

                fn disable(&mut self) {
                    unimplemented!()
                }

                fn enable(&mut self) {
                    unimplemented!()
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (&*atmega32u4::$TIMER::ptr()) }.$ocr.read().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    ::core::u8::MAX
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    unsafe { (&*atmega32u4::$TIMER::ptr()) }.$ocr.write(|w| w.bits(duty));
                }
            }
        )+
    }
}

// Timer0
timer_impl! {
    Info: (Timer0Pwm, TIMER0, tim),
    Init: {
        // Fast PWM Mode
        tim.tccr_a.modify(|_, w| w.wgm0().pwm_fast());
        // Enable Timer
        tim.tccr_b.modify(|_, w| w.cs().io_64());
    },
    Pins: [
        |portb, PB7, pwm| (ocr_a, {
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }),
        |portd, PD0, pwm| (ocr_b, {
            // Use OCR_B as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_b().match_clear());
        }),
    ]
}

// Timer1
timer_impl! {
    Info: (Timer1Pwm, TIMER1, tim),
    Init: {
        tim.tccr_a.modify(|_, w| unsafe { w.wgm0().bits(0b01) });
        tim.tccr_b.modify(|_, w| unsafe { w.wgm2().bits(0b01)}.cs().io_64());
    },
    Pins: [
        |portb, PB5, pwm| (ocr_a_l, {
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }),
        |portb, PB6, pwm| (ocr_b_l, {
            // Use OCR_B as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_b().match_clear());
        }),
        //////////////////////////////////////////////////////////////////
        // The following can be used instead of Timer0.ocr_a:
        //
        // |portb, PB7, pwm, dc| (ocr_c_l, {
        //     // Use OCR_C as Duty Cycle
        //     pwm.tim.tccr_a.modify(|_, w| w.com_c().match_clear());
        // }),
    ]
}

/// Marker for `PB7` pwm using Timer1
pub struct Pwm1;

// Manual second implementation
impl port::portb::PB7<port::mode::io::Output> {
    /// Make this pin  a PWM pin, but using Timer1 instead of Timer0
    ///
    /// `PB7` can be PWM'd by both Timer0 and Timer1.
    pub fn into_pwm1(self, pwm: &mut Timer1Pwm) -> port::portb::PB7<Pwm1> {
        pwm.tim.tccr_a.modify(|_, w| w.com_c().match_clear());

        port::portb::PB7 { _mode: marker::PhantomData }
    }
}

impl hal::PwmPin for port::portb::PB7<Pwm1> {
    type Duty = u8;

    fn disable(&mut self) {
        unimplemented!()
    }

    fn enable(&mut self) {
        unimplemented!()
    }

    fn get_duty(&self) -> Self::Duty {
        unsafe { (&*atmega32u4::TIMER1::ptr()) }
            .ocr_c_l
            .read()
            .bits()
    }

    fn get_max_duty(&self) -> Self::Duty {
        ::core::u8::MAX
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        unsafe { (&*atmega32u4::TIMER1::ptr()) }.ocr_c_l.write(
            |w| {
                w.bits(duty)
            },
        );
    }
}

// Timer3
timer_impl! {
    Info: (Timer3Pwm, TIMER3, tim),
    Init: {
        tim.tccr_a.modify(|_, w| unsafe { w.wgm0().bits(0b01) });
        tim.tccr_b.modify(|_, w| unsafe { w.wgm2().bits(0b01) }.cs().io_64());
    },
    Pins: [
        |portc, PC6, pwm| (ocr_a_l, {
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }),
    ]
}

// Timer4
timer_impl! {
    Info: (Timer4Pwm, TIMER4, tim),
    Init: {
        // Prescale/64
        tim.tccr_b.modify(|_, w| w.cs().clk_64());
        // Set WGM to Phase-Correct PWM Mode
        tim.tccr_d.modify(|_, w| unsafe { w.wgm().bits(0b01) });
    },
    Pins: [
        |portc, PC7, pwm| (ocr_a, {
            // Use OCR_A as Duty Cycle
            // Enable PWM for OCR_A
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear().pwm_a().set_bit());
        }),
        |portd, PD7, pwm| (ocr_d, {
            // Use OCR_D as Duty Cycle
            // Enable PWM for OCR_D
            pwm.tim.tccr_c.modify(|_, w| w.com_d().match_clear().pwm_d().set_bit());
        }),
    ]
}
