#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{board::Board, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[rustfmt::skip]
const PIXELS: [(usize, usize); 16] = [
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

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let (mut rows, mut cols) = board.display_pins.degrade();

    for r in &mut rows {
        r.set_high().unwrap();
    }

    for c in &mut cols {
        c.set_low().unwrap();
    }

    let mut switch_led = |(r, c): (usize, usize), set: bool| {
        if set {
            rows[r].set_low().unwrap();
            cols[c].set_high().unwrap();
        } else {
            rows[r].set_high().unwrap();
            cols[c].set_low().unwrap();
        }
    };

    let mut last_led = (0usize, 0usize);

    loop {
        for current_led in PIXELS.into_iter() {
            switch_led(last_led, false);
            switch_led(current_led, true);
            timer.delay_ms(30);
            last_led = current_led;
        }
    }
}
