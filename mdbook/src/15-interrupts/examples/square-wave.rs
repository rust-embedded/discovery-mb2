#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::{gpio, timer},
    Board,
};

/// The "period" is the time per cycle. It is
/// 1/f where f is the frequency in Hz. In this
/// case we measure time in milliseconds.
const PERIOD: u32 = 1000 / 220;

/// Number of cycles for 5 seconds of output.
const CYCLES: u32 = 5000 / PERIOD;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
    let mut timer = timer::Timer::new(board.TIMER0);

    for _ in 0..CYCLES {
        speaker_pin.set_high().unwrap();
        timer.delay_ms(PERIOD / 2);
        speaker_pin.set_low().unwrap();
        timer.delay_ms(PERIOD / 2);
    }

    loop {
        asm::wfi();
    }
}
