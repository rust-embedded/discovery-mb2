#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{display::blocking::Display, Board};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// Define LED patterns
const LEFT_ARROW: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
];

const RIGHT_ARROW: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

const CENTER_LED: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;

    loop {
        if button_a.is_low().unwrap() {
            display.show(&LEFT_ARROW);
        } else if button_b.is_low().unwrap() {
            display.show(&RIGHT_ARROW);
        } else {
            display.show(&CENTER_LED);
        }
        timer.delay_ms(10_u32);
    }
}
