#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{gpio, pac};
use panic_halt as _;

fn wait() {
    for _ in 0..4_000_000 {
        nop();
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    loop {
        wait();
        row1.set_high().unwrap();
        wait();
        row1.set_low().unwrap();
    }
}
