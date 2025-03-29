#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::hal::timer::Timer;
use microbit::{hal::gpio, Board};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configure buttons
    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;

    // Configure LED (top-left LED at row1, col1)
    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(gpio::Level::Low);
    let _col1 = board
        .display_pins
        .col1
        .into_push_pull_output(gpio::Level::Low);

    loop {
        if button_a.is_low().unwrap() {
            // Button A pressed: LED on
            row1.set_high().unwrap();
        } else if button_b.is_low().unwrap() {
            // Button B pressed: LED off
            row1.set_low().unwrap();
        }
        timer.delay_ms(10_u32);
    }
}
