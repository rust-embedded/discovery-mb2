//! Initialization code

#![deny(warnings)]
#![no_std]

use panic_rtt as _; // panic handler

pub use cortex_m_rt::entry;


use microbit::hal::{self,pac};
pub use pac::{p0, p1};


#[inline(never)]
pub fn init() -> (&'static p0::RegisterBlock, &'static p1::RegisterBlock) {
    let device_periphs = pac::Peripherals::take().unwrap();
    
    // `display_pins!` initializes the display pins as outputs in push-pull mode
    let port0 = hal::gpio::p0::Parts::new(device_periphs.P0);
    let port1 = hal::gpio::p1::Parts::new(device_periphs.P1);
    let _display_pins = microbit::display_pins!(port0, port1);

    (unsafe { &*pac::P0::ptr() }, unsafe { &*pac::P1::ptr() })
}
