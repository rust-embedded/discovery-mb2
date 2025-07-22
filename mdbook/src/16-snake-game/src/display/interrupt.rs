use super::DISPLAY;

use cortex_m::interrupt::free as interrupt_free;
use microbit::pac::{self, interrupt};

#[pac::interrupt]
fn TIMER1() {
    interrupt_free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}
