#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = microbit::Board::take().unwrap();

    loop {
        wfi();
    }
}
