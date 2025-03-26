#![no_main]
#![no_std]

use registers::{entry, rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let (p0, _p1) = registers::init();

    loop {
        // Read pin P0.14 from IN register; active low means pressed when bit is 0.
        let button_a_pressed = p0.in_.read().pin14().bit_is_clear();

        rprintln!("Button A pressed: {}", button_a_pressed);
    }
}
