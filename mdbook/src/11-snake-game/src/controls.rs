mod init;
mod interrupt;

pub use init::init_buttons;

use crate::game::Turn;
use core::cell::RefCell;
use cortex_m::interrupt::{free as interrupt_free, Mutex};
use microbit::{board::Buttons, hal::gpiote::Gpiote};
pub static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
pub static TURN: Mutex<RefCell<Turn>> = Mutex::new(RefCell::new(Turn::None));

/// Get the next turn (ie, the turn corresponding to the most recently pressed button).
pub fn get_turn(reset: bool) -> Turn {
    interrupt_free(|cs| {
        let turn = *TURN.borrow(cs).borrow();
        if reset {
            *TURN.borrow(cs).borrow_mut() = Turn::None
        }
        turn
    })
}
