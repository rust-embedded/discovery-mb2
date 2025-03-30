use super::{Turn, GPIO, TURN};

use cortex_m::interrupt::free as interrupt_free;
use microbit::pac::{self, interrupt};

#[pac::interrupt]
fn GPIOTE() {
    interrupt_free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let turn = match (a_pressed, b_pressed) {
                (true, false) => Turn::Left,
                (false, true) => Turn::Right,
                _ => Turn::None,
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            *TURN.borrow(cs).borrow_mut() = turn;
        }
    });
}
