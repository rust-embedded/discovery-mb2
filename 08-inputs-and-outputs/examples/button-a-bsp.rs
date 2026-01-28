#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::Board;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut button_a = board.buttons.button_a;
    let mut button_state = false;

    loop {
        if button_a.is_low().unwrap() {
            if button_state == false {
                button_state = true;
                rprintln!("Button A pressed");
            }
        } else {
            if button_state == true {
                button_state = false;
                rprintln!("Button A not pressed");
            }
        }
    }
}
