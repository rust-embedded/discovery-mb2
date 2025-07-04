#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    loop {
        timer.delay_ms(1000u32);
        rprintln!("1000 ms passed");
    }
}
