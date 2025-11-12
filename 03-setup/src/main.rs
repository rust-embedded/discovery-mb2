#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use panic_rtt_target as _;
use nrf52833_pac as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");
    loop {
        wfi();
    }
}
