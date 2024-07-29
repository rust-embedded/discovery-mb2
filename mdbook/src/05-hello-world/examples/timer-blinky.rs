#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{gpio, pac, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    loop {
        timer0.delay_ms(500);
        row1.set_high().unwrap();
        timer0.delay_ms(500);
        row1.set_low().unwrap();
    }
}
