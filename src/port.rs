pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

pub struct Input;

pub struct Output;

macro_rules! port {
    ($PORTX:ident, $portx:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        pub mod $portx {
            use core::marker;

            use atmega32u4;
            use hal::digital;
            use super::{PortExt, Input, Output};

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

            $(
                pub struct $PXi<MODE> {
                    _mode: marker::PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_input(self, ddr: &mut DDR) -> $PXi<Input> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });

                        $PXi { _mode: marker::PhantomData }
                    }

                    pub fn into_output(self, ddr: &mut DDR) -> $PXi<Output> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });

                        $PXi { _mode: marker::PhantomData }
                    }
                }

                impl digital::OutputPin for $PXi<Output> {
                    fn set_high(&mut self) {
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() | (1 << $i))) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*atmega32u4::$PORTX::ptr()).port.modify(|r, w| w.bits(r.bits() & !(1 << $i))) }
                    }
                }

                impl digital::InputPin for $PXi<Input> {
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

port! (PORTB, portb, PBx, [
    PB0: (pb0, 0, Input),
    PB1: (pb1, 1, Input),
    PB2: (pb2, 2, Input),
    PB3: (pb3, 3, Input),
    PB4: (pb4, 4, Input),
    PB5: (pb5, 5, Input),
    PB6: (pb6, 6, Input),
    PB7: (pb7, 7, Input),
]);

port! (PORTC, portc, PCx, [
    PC0: (pc0, 0, Input),
    PC1: (pc1, 1, Input),
    PC2: (pc2, 2, Input),
    PC3: (pc3, 3, Input),
    PC4: (pc4, 4, Input),
    PC5: (pc5, 5, Input),
    PC6: (pc6, 6, Input),
    PC7: (pc7, 7, Input),
]);

port! (PORTD, portd, PDx, [
    PD0: (pd0, 0, Input),
    PD1: (pd1, 1, Input),
    PD2: (pd2, 2, Input),
    PD3: (pd3, 3, Input),
    PD4: (pd4, 4, Input),
    PD5: (pd5, 5, Input),
    PD6: (pd6, 6, Input),
    PD7: (pd7, 7, Input),
]);
