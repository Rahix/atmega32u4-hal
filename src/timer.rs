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

// Timer0
timer! {
    Info: (Timer0Pwm, TIMER0, tim),
    Init: {
        // Fast PWM Mode
        tim.tccr_a.modify(|_, w| w.wgm0().pwm_fast());
        // Enable Timer
        tim.tccr_b.modify(|_, w| w.cs().io_64());
    },
    Pins: [
        |portb, PB7, pwm, dc| ({
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }, {
            unsafe { (&*atmega32u4::TIMER0::ptr()) }.ocr_a.write(|w| w.bits(dc));
        }),
        |portd, PD0, pwm, dc| ({
            // Use OCR_B as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_b().match_clear());
        }, {
            unsafe { (&*atmega32u4::TIMER0::ptr()) }.ocr_b.write(|w| w.bits(dc));
        }),
    ]
}

// Timer1
timer! {
    Info: (Timer1Pwm, TIMER1, tim),
    Init: {
        tim.tccr_a.modify(|_, w| unsafe { w.wgm0().bits(0b01) });
        tim.tccr_b.modify(|_, w| unsafe { w.wgm2().bits(0b01)}.cs().io_64());
    },
    Pins: [
        |portb, PB5, pwm, dc| ({
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }, {
            unsafe { (&*atmega32u4::TIMER1::ptr()) }.ocr_a_l.write(|w| w.bits(dc));
        }),
        |portb, PB6, pwm, dc| ({
            // Use OCR_B as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_b().match_clear());
        }, {
            unsafe { (&*atmega32u4::TIMER1::ptr()) }.ocr_b_l.write(|w| w.bits(dc));
        }),
        //////////////////////////////////////////////////////////////////
        // The following can be used instead of Timer0.ocr_a:
        //
        // |portb, PB7, pwm, dc| ({
        //     // Use OCR_C as Duty Cycle
        //     pwm.tim.tccr_a.modify(|_, w| w.com_c().match_clear());
        // }, {
        //     unsafe { (&*atmega32u4::TIMER1::ptr()) }.ocr_c_l.write(|w| w.bits(dc));
        // }),
    ]
}

// Timer3
timer! {
    Info: (Timer3Pwm, TIMER3, tim),
    Init: {
        tim.tccr_a.modify(|_, w| unsafe { w.wgm0().bits(0b01) });
        tim.tccr_b.modify(|_, w| unsafe { w.wgm2().bits(0b01) }.cs().io_64());
    },
    Pins: [
        |portc, PC6, pwm, dc| ({
            // Use OCR_A as Duty Cycle
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear());
        }, {
            unsafe { (&*atmega32u4::TIMER3::ptr()) }.ocr_a_l.write(|w| w.bits(dc));
        }),
    ]
}

// Timer4
timer! {
    Info: (Timer4Pwm, TIMER4, tim),
    Init: {
        // Prescale/64
        tim.tccr_b.modify(|_, w| w.cs().clk_64());
        // Set WGM to Phase-Correct PWM Mode
        tim.tccr_d.modify(|_, w| unsafe { w.wgm().bits(0b01) });
    },
    Pins: [
        |portc, PC7, pwm, dc| ({
            // Use OCR_A as Duty Cycle
            // Enable PWM for OCR_A
            pwm.tim.tccr_a.modify(|_, w| w.com_a().match_clear().pwm_a().set_bit());
        }, {
            unsafe { (&*atmega32u4::TIMER4::ptr()) }.ocr_a.write(|w| w.bits(dc));
        }),
        |portd, PD7, pwm, dc| ({
            // Use OCR_D as Duty Cycle
            // Enable PWM for OCR_D
            pwm.tim.tccr_c.modify(|_, w| w.com_d().match_clear().pwm_d().set_bit());
        }, {
            unsafe { (&*atmega32u4::TIMER4::ptr()) }.ocr_d.write(|w| w.bits(dc));
        }),
    ]
}
