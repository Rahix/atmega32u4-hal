//! Port{B,C,D,E,F}
//!
//! Abstraction of the IO pins.
//!
//! # Design
//! For each port, you can call `.split()` on the raw periperal to separate the pins.
//!
//! By default, each pin is `Input<Floating>`.  There are three methods to change the
//! mode:
//! * `into_floating_input()`: Turn a pin into a floating input
//! * `into_pull_up_input()`: Turn a pin into a pull-up input
//! * `into_output()`: Turn a pin into an output
//!
//! For input pins [embedded_hal::digital::InputPin] is implemented, for output
//! pins [embedded_hal::digital::OutputPin] & [embedded_hal::digital::StatefulOutputPin].
//!
//! ## Downgrading
//! After `.split()` each pin is of a separate type.  This means you can't store them
//! in an array.  To allow doing so you can `.downgrade()` a pin.  This can be done
//! twice:  The first downgrade makes the pin generic for its port, the second downgrade
//! makes it fully generic.
//!
//! *Note*: After downgrading a pin, you can no longer change its mode!
//!
//! ## PWM
//! Some pins can be configured to output a PWM signal.  This is not implemented in the port
//! module but in the [timer] module.
//!
//! # Example
//! ```
//! // Get the raw peripherals
//! let dp = atmega32u4::Peripherals::take().unwrap();
//!
//! // Split the port and create an output pin
//! let mut portc = dp.PORTC.split();
//! let mut pc7 = portc.pc7.into_output(&mut portc.ddr);
//!
//! // Use the pin
//! pc7.set_high();
//! pc7.set_low();
//! ```
use atmega32u4;
use core::marker;
use hal::digital;

/// A splittable port
pub trait PortExt {
    /// Type that contains the split result
    type Parts;

    /// Split this port into 8 pins
    fn split(self) -> Self::Parts;
}

/// Pin modes
pub mod mode {
    use core::marker;

    /// Any digital IO mode
    pub trait Io {}

    /// Digital IO modes
    pub mod io {
        use core::marker;

        /// Input
        pub struct Input<MODE> {
            _mode: marker::PhantomData<MODE>,
        }
        /// Output
        pub struct Output;

        /// PullUp Input
        pub struct PullUp;

        /// Floating Input
        pub struct Floating;

        impl<MODE> super::Io for Input<MODE> {}
        impl super::Io for Output {}
    }

    /// Pulse Width Modulated Output
    pub struct Pwm<TIMER> {
        pub(crate) _tim: marker::PhantomData<TIMER>,
    }
}

macro_rules! port_impl {
    ($PortEnum:ident, $PORTX:ident, $portx:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// Port Types
        pub mod $portx {
            use core::marker;

            use atmega32u4;
            use hal::digital;
            use super::{PortExt, mode};

            /// Splitted port parts
            pub struct Parts {
                /// Data direction register
                pub ddr: DDR,
                $(
                    /// Pin
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

            /// Type that can export this ports data direction register
            pub trait PortDDR {
                #[doc(hidden)]
                fn ddr(&mut self) -> &atmega32u4::$portx::DDR;
            }

            /// Data direction register
            pub struct DDR {
                _0: (),
            }

            impl PortDDR for DDR {
                /// Access the ddr register
                fn ddr(&mut self) -> &atmega32u4::$portx::DDR {
                    unsafe { &(*atmega32u4::$PORTX::ptr()).ddr }
                }
            }

            /// Generalized pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: marker::PhantomData<MODE>,
            }

            impl digital::OutputPin for $PXx<mode::io::Output> {
                fn set_high(&mut self) {
                    unsafe {
                        (*atmega32u4::$PORTX::ptr())
                            .port.modify(|r, w| w.bits(r.bits() | (1 << self.i)))
                    }
                }

                fn set_low(&mut self) {
                    unsafe {
                        (*atmega32u4::$PORTX::ptr())
                            .port.modify(|r, w| w.bits(r.bits() & !(1 << self.i)))
                    }
                }
            }

            impl digital::StatefulOutputPin for $PXx<mode::io::Output> {
                fn is_set_high(&self) -> bool {
                    (unsafe {
                        (*atmega32u4::$PORTX::ptr()).port.read().bits()
                    } & (1 << self.i)) != 0
                }

                fn is_set_low(&self) -> bool {
                    (unsafe {
                        (*atmega32u4::$PORTX::ptr()).port.read().bits()
                    } & (1 << self.i)) == 0
                }
            }

            impl digital::toggleable::Default for $PXx<mode::io::Output> { }

            impl<MODE> digital::InputPin for $PXx<mode::io::Input<MODE>> {
                fn is_high(&self) -> bool {
                    (unsafe {
                        (*atmega32u4::$PORTX::ptr()).pin.read().bits()
                    } & (1 << self.i)) != 0
                }

                fn is_low(&self) -> bool {
                    (unsafe {
                        (*atmega32u4::$PORTX::ptr()).pin.read().bits()
                    } & (1 << self.i)) == 0
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    pub(crate) _mode: marker::PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Downgrade this pin into a more generic pin type
                    ///
                    /// This allows storing multiple pins in an array. It does however
                    /// come with some runtime overhead, so choose `downgrade_port` if
                    /// possible.
                    ///
                    /// *Note*: The mode of downgraded pins can no longer be changed.
                    pub fn downgrade(self) -> super::Pin<MODE> {
                        super::Pin {
                            i: $i,
                            port: super::Port::$PortEnum,
                            _mode: marker::PhantomData,
                        }
                    }

                    /// Downgrade this pin into a more type generic over all pins of this port
                    ///
                    /// This allows storing multiple pins of a port in an array
                    ///
                    /// *Note*: The mode of downgraded pins can no longer be changed.
                    pub fn downgrade_port(self) -> $PXx<MODE> {
                        $PXx {
                            i: $i,
                            _mode: marker::PhantomData,
                        }
                    }
                }

                impl<MODE: mode::Io> $PXi<MODE> {
                    /// Turn this pin into a floating input
                    pub fn into_floating_input<D: PortDDR>(
                        self,
                        ddr: &mut D,
                    ) -> $PXi<mode::io::Input<mode::io::Floating>> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });

                        unsafe {
                            (*atmega32u4::$PORTX::ptr())
                                .port.modify(|r, w| w.bits(r.bits() & !(1 << $i)))
                        }

                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Turn this pin into a pull up input
                    pub fn into_pull_up_input<D: PortDDR>(
                        self,
                        ddr: &mut D,
                    ) -> $PXi<mode::io::Input<mode::io::PullUp>> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });

                        unsafe {
                            (*atmega32u4::$PORTX::ptr())
                                .port.modify(|r, w| w.bits(r.bits() | (1 << $i)))
                        }

                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Turn this pin into an output input
                    pub fn into_output<D: PortDDR>(self, ddr: &mut D) -> $PXi<mode::io::Output> {
                        ddr.ddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });

                        $PXi { _mode: marker::PhantomData }
                    }
                }

                impl digital::OutputPin for $PXi<mode::io::Output> {
                    fn set_high(&mut self) {
                        unsafe {
                            (*atmega32u4::$PORTX::ptr())
                                .port.modify(|r, w| w.bits(r.bits() | (1 << $i)))
                        }
                    }

                    fn set_low(&mut self) {
                        unsafe {
                            (*atmega32u4::$PORTX::ptr())
                                .port.modify(|r, w| w.bits(r.bits() & !(1 << $i)))
                        }
                    }
                }

                impl digital::StatefulOutputPin for $PXi<mode::io::Output> {
                    fn is_set_high(&self) -> bool {
                        (unsafe {
                            (*atmega32u4::$PORTX::ptr()).port.read().bits()
                        } & (1 << $i)) != 0
                    }

                    fn is_set_low(&self) -> bool {
                        (unsafe {
                            (*atmega32u4::$PORTX::ptr()).port.read().bits()
                        } & (1 << $i)) == 0
                    }
                }

                impl digital::toggleable::Default for $PXi<mode::io::Output> { }

                impl<MODE> digital::InputPin for $PXi<mode::io::Input<MODE>> {
                    fn is_high(&self) -> bool {
                        (unsafe {
                            (*atmega32u4::$PORTX::ptr()).pin.read().bits()
                        } & (1 << $i)) != 0
                    }

                    fn is_low(&self) -> bool {
                        (unsafe {
                            (*atmega32u4::$PORTX::ptr()).pin.read().bits()
                        } & (1 << $i)) == 0
                    }
                }
            )+
        }
    }
}

macro_rules! generic_pin_impl {
    ($($PortEnum:ident: $Port:ident,)+) => {
        #[derive(Clone, Copy, Debug)]
        enum Port {
            $($PortEnum,)+
        }

        /// A completely generic pin
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
                            (*atmega32u4::$Port::ptr())
                                .port.modify(|r, w| w.bits(r.bits() | (1 << self.i)))
                        },
                    )+
                }
            }

            fn set_low(&mut self) {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            (*atmega32u4::$Port::ptr())
                                .port.modify(|r, w| w.bits(r.bits() & !(1 << self.i)))
                        },
                    )+
                }
            }
        }

        impl digital::StatefulOutputPin for Pin<mode::io::Output> {
            fn is_set_high(&self) -> bool {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            ((*atmega32u4::$Port::ptr()).port.read().bits() & (1 << self.i)) != 0
                        },
                    )+
                }
            }

            fn is_set_low(&self) -> bool {
                match self.port {
                    $(
                        Port::$PortEnum => unsafe {
                            ((*atmega32u4::$Port::ptr()).port.read().bits() & (1 << self.i)) == 0
                        },
                    )+
                }
            }
        }

        impl digital::toggleable::Default for Pin<mode::io::Output> { }

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

generic_pin_impl!(B: PORTB, C: PORTC, D: PORTD, E: PORTE, F: PORTF,);

port_impl! (B, PORTB, portb, PBx, [
    PB0: (pb0, 0, mode::io::Input<mode::io::Floating>),
    PB1: (pb1, 1, mode::io::Input<mode::io::Floating>),
    PB2: (pb2, 2, mode::io::Input<mode::io::Floating>),
    PB3: (pb3, 3, mode::io::Input<mode::io::Floating>),
    PB4: (pb4, 4, mode::io::Input<mode::io::Floating>),
    PB5: (pb5, 5, mode::io::Input<mode::io::Floating>),
    PB6: (pb6, 6, mode::io::Input<mode::io::Floating>),
    PB7: (pb7, 7, mode::io::Input<mode::io::Floating>),
]);

port_impl! (C, PORTC, portc, PCx, [
    PC6: (pc6, 6, mode::io::Input<mode::io::Floating>),
    PC7: (pc7, 7, mode::io::Input<mode::io::Floating>),
]);

port_impl! (D, PORTD, portd, PDx, [
    PD0: (pd0, 0, mode::io::Input<mode::io::Floating>),
    PD1: (pd1, 1, mode::io::Input<mode::io::Floating>),
    PD2: (pd2, 2, mode::io::Input<mode::io::Floating>),
    PD3: (pd3, 3, mode::io::Input<mode::io::Floating>),
    PD4: (pd4, 4, mode::io::Input<mode::io::Floating>),
    PD5: (pd5, 5, mode::io::Input<mode::io::Floating>),
    PD6: (pd6, 6, mode::io::Input<mode::io::Floating>),
    PD7: (pd7, 7, mode::io::Input<mode::io::Floating>),
]);

port_impl! (E, PORTE, porte, PEx, [
    PE2: (pe2, 2, mode::io::Input<mode::io::Floating>),
    PE6: (pe6, 6, mode::io::Input<mode::io::Floating>),
]);

port_impl! (F, PORTF, portf, PFx, [
    PF0: (pf0, 0, mode::io::Input<mode::io::Floating>),
    PF1: (pf1, 1, mode::io::Input<mode::io::Floating>),
    PF4: (pf4, 4, mode::io::Input<mode::io::Floating>),
    PF5: (pf5, 5, mode::io::Input<mode::io::Floating>),
    PF6: (pf6, 6, mode::io::Input<mode::io::Floating>),
    PF7: (pf7, 7, mode::io::Input<mode::io::Floating>),
]);

// Inspired by the macro from wez/atsamd21-rs
#[doc(hidden)]
#[macro_export]
macro_rules! define_pins {
    (
        $(#[$pins_attr:meta])*
        name: $Pins:ident,
        ddr: $DDR:ident {
            $($portx:ident: $PORTX:ty,)+
        },
        pins: {
            $(
                $(#[$attr:meta])*
                $name:ident: ($port:ident, $pin:ident, $PIN:ident),
            )+
        }
    ) => {
        /// Generic DDR type that can be used for all ports
        pub struct $DDR {
            $(
                $portx: $crate::port::$portx::DDR,
            )+
        }

        $(
            impl $crate::port::$portx::PortDDR for $DDR {
                fn ddr(&mut self) -> &atmega32u4::$portx::DDR {
                    self.$portx.ddr()
                }
            }
        )+

        $(#[$pins_attr])*
        pub struct $Pins {
            $(
                $(#[$attr])*
                pub $name: $crate::port::$port::$PIN<
                    $crate::port::mode::io::Input<$crate::port::mode::io::Floating>
                >,
            )+
            /// Data Direction Register
            ///
            /// This ddr is generic and can be used for all pins
            pub ddr: $DDR,
        }

        impl $Pins {
            /// Initialize pins
            pub fn new(
                $( $portx: $PORTX, )+
            ) -> Pins {
                use $crate::port::PortExt;

                $( let $portx = $portx.split(); )+

                Pins {
                    $(
                        $name: $port.$pin,
                    )+
                    ddr: $DDR {
                        $(
                            $portx: $portx.ddr,
                        )+
                    }
                }
            }
        }
    };
}
