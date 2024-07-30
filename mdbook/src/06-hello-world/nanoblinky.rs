#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;
use gd32vf103xx_hal::{pac, prelude::*, delay::McycleDelay};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure().ext_hf_clock(8.mhz()).sysclk(108.mhz()).freeze();

    let gpioc = dp.GPIOC.split(&mut rcu);
    let mut led = gpioc.pc13.into_push_pull_output();
    let mut delay = McycleDelay::new(&rcu.clocks);

    loop {
        delay.delay_ms(500);
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
    }
}
