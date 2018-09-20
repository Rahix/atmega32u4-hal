use atmega32u4;
use hal::digital;
use core::marker;

pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

pub mod mode {
    use core::marker;

    pub struct Io<MODE> {
        _mode: marker::PhantomData<MODE>,
    }

    pub mod io {
        use core::marker;

        pub struct Input<MODE> {
            _mode: marker::PhantomData<MODE>,
        }
        pub struct Output;

        pub struct PullUp;
        pub struct Floating;
    }

    pub struct Pwm;
}

macro_rules! port {
    ($PortEnum:ident, $PORTX:ident, $portx:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        pub mod $portx {
            use core::marker;

            use atmega32u4;
            use hal::digital;
            use super::{PortExt, mode};

            pub struct Parts {
                pub ddr: DDR,
                $(
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl PortExt for atmega32u4::$PORTX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        ddr: DDR { _0: () },
                        $(
                            $pxi: $PXi { _mode: marker::PhantomData },
                        )+
                    }
                }
            }

            pub struct DDR {
                _0: (),
            }

            impl DDR {
                pub fn ddr(&mut self) -> &atmega32u4::$portx::DDR {
                    unsafe { &(*atmega32u4::$PORTX::ptr()).ddr }
                }
            }

            pub struct $PXx<MODE> {
                i: u8,
                _mode: marker::PhantomData<MODE>,
            }

            impl<MODE> $PXx<MODE> {
                pub fn downgrade(self) -> super::Pin<MODE> {
                    super::Pin {
                        i: self.i,
                        port: super::Port::$PortEnum,
                        _mode: marker::PhantomData,
                    }
                }
            }

            impl digital::OutputPin for $PXx<mode::io::Output> {
                fn set_high(&mut self) {
                    unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() | (1 << self.i))) }
                }

                fn set_low(&mut self) {
                    unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() & !(1 << self.i))) }
                }
            }

            impl<MODE> digital::InputPin for $PXx<mode::io::Input<MODE>> {
                fn is_high(&self) -> bool {
                    (unsafe { (*atmega32u4::$PORTX::ptr()).pin.read().bits() } & (1 << self.i)) != 0
                }

                fn is_low(&self) -> bool {
                    (unsafe { (*atmega32u4::$PORTX::ptr()).pin.read().bits() } & (1 << self.i)) == 0
                }
            }

            $(
                pub struct $PXi<MODE> {
                    pub(crate) _mode: marker::PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_floating_input(self, ddr: &mut DDR) -> $PXi<mode::io::Input<mode::io::Floating>> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() & !(1 << $i))) }

                        $PXi { _mode: marker::PhantomData }
                    }

                    pub fn into_pull_up_input(self, ddr: &mut DDR) -> $PXi<mode::io::Input<mode::io::PullUp>> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() | (1 << $i))) }

                        $PXi { _mode: marker::PhantomData }
                    }

                    pub fn into_output(self, ddr: &mut DDR) -> $PXi<mode::io::Output> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });

                        $PXi { _mode: marker::PhantomData }
                    }

                    pub fn downgrade(self) -> $PXx<MODE> {
                        $PXx {
                            i: $i,
                            _mode: marker::PhantomData,
                        }
                    }
                }

                impl digital::OutputPin for $PXi<mode::io::Output> {
                    fn set_high(&mut self) {
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() | (1 << $i))) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() & !(1 << $i))) }
                    }
                }

                impl<MODE> digital::InputPin for $PXi<mode::io::Input<MODE>> {
                    fn is_high(&self) -> bool {
                        (unsafe { (*atmega32u4::$PORTX::ptr()).pin.read().bits() } & (1 << $i)) != 0
                    }

                    fn is_low(&self) -> bool {
                        (unsafe { (*atmega32u4::$PORTX::ptr()).pin.read().bits() } & (1 << $i)) == 0
                    }
                }
            )+
        }
    }
}

macro_rules! impl_generic_pin {
    ($($PortEnum:ident: $Port:ident,)+) => {
        #[derive(Clone, Copy, Debug)]
        enum Port {
            $($PortEnum,)+
        }

        #[derive(Debug)]
        pub struct Pin<MODE> {
            i: u8,
            port: Port,
            _mode: marker::PhantomData<MODE>,
        }

        impl digital::OutputPin for Pin<mode::io::Output> {
            fn set_high(&mut self) {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            (*atmega32u4::$Port::ptr()).port.modify(|r, w| w.bits(r.bits() | (1 << self.i)))
                        },
                    )+
                }
            }

            fn set_low(&mut self) {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            (*atmega32u4::$Port::ptr()).port.modify(|r, w| w.bits(r.bits() & !(1 << self.i)))
                        },
                    )+
                }
            }
        }

        impl<MODE> digital::InputPin for Pin<mode::io::Input<MODE>> {
            fn is_high(&self) -> bool {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            ((*atmega32u4::$Port::ptr()).pin.read().bits() & (1 << self.i)) != 0
                        },
                    )+
                }
            }

            fn is_low(&self) -> bool {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            ((*atmega32u4::$Port::ptr()).pin.read().bits() & (1 << self.i)) == 0
                        },
                    )+
                }
            }
        }
    }
}

impl_generic_pin!(
    B: PORTB,
    C: PORTC,
    D: PORTD,
    E: PORTE,
    F: PORTF,
);

port! (B, PORTB, portb, PBx, [
    PB0: (pb0, 0, mode::io::Input<mode::io::Floating>),
    PB1: (pb1, 1, mode::io::Input<mode::io::Floating>),
    PB2: (pb2, 2, mode::io::Input<mode::io::Floating>),
    PB3: (pb3, 3, mode::io::Input<mode::io::Floating>),
    PB4: (pb4, 4, mode::io::Input<mode::io::Floating>),
    PB5: (pb5, 5, mode::io::Input<mode::io::Floating>),
    PB6: (pb6, 6, mode::io::Input<mode::io::Floating>),
    PB7: (pb7, 7, mode::io::Input<mode::io::Floating>),
]);

port! (C, PORTC, portc, PCx, [
    PC6: (pc6, 6, mode::io::Input<mode::io::Floating>),
    PC7: (pc7, 7, mode::io::Input<mode::io::Floating>),
]);

port! (D, PORTD, portd, PDx, [
    PD0: (pd0, 0, mode::io::Input<mode::io::Floating>),
    PD1: (pd1, 1, mode::io::Input<mode::io::Floating>),
    PD2: (pd2, 2, mode::io::Input<mode::io::Floating>),
    PD3: (pd3, 3, mode::io::Input<mode::io::Floating>),
    PD4: (pd4, 4, mode::io::Input<mode::io::Floating>),
    PD5: (pd5, 5, mode::io::Input<mode::io::Floating>),
    PD6: (pd6, 6, mode::io::Input<mode::io::Floating>),
    PD7: (pd7, 7, mode::io::Input<mode::io::Floating>),
]);

port! (E, PORTE, porte, PEx, [
    PE2: (pe2, 2, mode::io::Input<mode::io::Floating>),
    PE6: (pe6, 6, mode::io::Input<mode::io::Floating>),
]);

port! (F, PORTF, portf, PFx, [
    PF0: (pf0, 0, mode::io::Input<mode::io::Floating>),
    PF1: (pf1, 1, mode::io::Input<mode::io::Floating>),
    PF4: (pf4, 4, mode::io::Input<mode::io::Floating>),
    PF5: (pf5, 5, mode::io::Input<mode::io::Floating>),
    PF6: (pf6, 6, mode::io::Input<mode::io::Floating>),
    PF7: (pf7, 7, mode::io::Input<mode::io::Floating>),
]);
