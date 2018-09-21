use core::marker;
use atmega32u4;
use port;

macro_rules! timer {
    (
        Info: ($Timer:ident, $TIMER:ident, $tim:ident),
        Init: $init:block,
        Pins: [
            $(|$port:ident, $PIN:ident, $pwm:ident, $dc:ident| ($setup:block, $set_dc:block),)+
        ]
    ) => {
        pub struct $Timer {
            $tim: atmega32u4::$TIMER,
        }

        impl $Timer {
            pub fn new($tim: atmega32u4::$TIMER) -> $Timer {
                $init

                $Timer {
                    $tim: $tim,
                }
            }
        }

        $(
            impl port::$port::$PIN<port::mode::io::Output> {
                pub fn into_pwm(self, $pwm: &mut $Timer) -> port::$port::$PIN<port::mode::Pwm> {
                    $setup

                    port::$port::$PIN {
                        _mode: marker::PhantomData,
                    }
                }
            }

            impl port::$port::$PIN<port::mode::Pwm> {
                pub fn set_duty_cycle(&mut self, $dc: u8) {
                    $set_dc
                }
            }
        )+
    }
}

timer! {
    Info: (Timer0Pwm, TIMER0, tim),
    Init: {
        // Fast PWM Mode
        tim.tccr_a.modify( |_, w| unsafe { w.wgm0().bits(0x11) });
        // Enable timer
        tim.tccr_b.modify( |_, w| unsafe { w.cs().bits(0b11) });
    },
    Pins: [
        |portb, PB7, pwm, dc| ({
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify( |_, w| unsafe { w.com_a().bits(0b10) });
        }, {
            unsafe { (*atmega32u4::TIMER0::ptr()).ocr_a.write(|w| w.bits(dc)); }
        }),
        |portd, PD0, pwm, dc| ({
            // Use OCR_B as Duty Cycle
            pwm.tim.tccr_a.modify( |_, w| unsafe { w.com_b().bits(0b10) });
        }, {
            unsafe { (*atmega32u4::TIMER0::ptr()).ocr_b.write(|w| w.bits(dc)); }
        }),
    ]
}
