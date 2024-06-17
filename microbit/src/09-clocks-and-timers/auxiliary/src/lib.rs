//! Initialization code

#![no_std]

use microbit::hal::gpio::{Output, Pin, PushPull};
use panic_rtt_target as _;

pub use cortex_m::asm::{bkpt, nop};
pub use cortex_m_rt::entry;

pub use microbit::{pac, Board};
use rtt_target::rtt_init_print;

pub const PIXELS: [(usize, usize); 16] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
];

pub type DisplayLeds = ([Pin<Output<PushPull>>; 5], [Pin<Output<PushPull>>; 5]);

pub fn init() -> (DisplayLeds, &'static pac::timer0::RegisterBlock) {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let led_pins = board.display_pins.degrade();

    (led_pins, unsafe { &*microbit::pac::TIMER0::ptr() })
}
