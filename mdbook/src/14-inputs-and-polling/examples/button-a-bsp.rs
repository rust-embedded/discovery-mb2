#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::Board;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let button_a = board.buttons.button_a;

    loop {
        if button_a.is_low().unwrap() {
            rprintln!("Button A pressed");
        } else {
            rprintln!("Button A not pressed");
        }
    }
}
