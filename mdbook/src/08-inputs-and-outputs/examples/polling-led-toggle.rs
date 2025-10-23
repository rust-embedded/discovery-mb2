#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::timer::Timer;
use microbit::{hal::gpio, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configure buttons
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

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
        let on_pressed = button_a.is_low().unwrap();
        let off_pressed = button_b.is_low().unwrap();
        match (on_pressed, off_pressed) {
            // Stay in current state until something is pressed.
            (false, false) => (),
            // Change to on state.
            (true, false) => row1.set_high().unwrap(),
            // Change to off state.
            (false, true) => row1.set_low().unwrap(),
            // Stay in current state until something is released.
            (true, true) => (),
        }
        timer.delay_ms(10_u32);
    }
}
