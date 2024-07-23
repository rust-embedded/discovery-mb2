#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    let (p0, _p1) = aux7::init();

    // Turn on the top row
    p0.out.modify(|_, w| w.pin21().set_bit());

    // Turn on the bottom row
    p0.out.modify(|_, w| w.pin19().set_bit());

    // Turn off the top row
    p0.out.modify(|_, w| w.pin21().clear_bit());

    // Turn off the bottom row
    p0.out.modify(|_, w| w.pin19().clear_bit());

    loop {}
}
