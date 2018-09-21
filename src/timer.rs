use core::marker;
use atmega32u4;
use port;

pub struct Timer0Pwm {
    tim: atmega32u4::TIMER0,
}

impl Timer0Pwm {
    pub fn new(tim: atmega32u4::TIMER0) -> Timer0Pwm {
        tim.tccr_a.modify(
            |_, w| unsafe {
                w.wgm0().bits(0x11)  // Fast PWM Mode
            }
        );

        tim.tccr_b.modify(
            |_, w| unsafe {
                w.cs().bits(0b11)  // Enable timer
            }
        );

        Timer0Pwm {
            tim
        }
    }
}

impl port::portb::PB7<port::mode::io::Output> {
    pub fn into_pwm(self, pwm: &mut Timer0Pwm) -> port::portb::PB7<port::mode::Pwm> {
        pwm.tim.tccr_a.modify(
            |_, w| unsafe {
                w.com_a().bits(0b10)  // Use OCR_A as Duty Cycle
            }
        );

        pwm.tim.tccr_b.modify(
            |_, w| unsafe {
                w.cs().bits(0b11)  // Enable timer
            }
        );

        port::portb::PB7 {
            _mode: marker::PhantomData,
        }
    }
}

impl port::portb::PB7<port::mode::Pwm> {
    pub fn set_duty_cycle(&mut self, dc: u8) {
        unsafe {
            (*atmega32u4::TIMER0::ptr()).ocr_a.write(|w| w.bits(dc));
        }
    }
}

impl port::portd::PD0<port::mode::io::Output> {
    pub fn into_pwm(self, pwm: &mut Timer0Pwm) -> port::portd::PD0<port::mode::Pwm> {
        pwm.tim.tccr_a.modify(
            |_, w| unsafe {
                w.com_b().bits(0b10)  // Use OCR_B as Duty Cycle
            }
        );

        port::portd::PD0 {
            _mode: marker::PhantomData,
        }
    }
}

impl port::portd::PD0<port::mode::Pwm> {
    pub fn set_duty_cycle(&mut self, dc: u8) {
        unsafe {
            (*atmega32u4::TIMER0::ptr()).ocr_b.write(|w| w.bits(dc));
        }
    }
}
