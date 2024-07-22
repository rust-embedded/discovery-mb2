pub mod interrupt;
pub mod show;

pub use show::{display_image, clear_display};

use core::cell::RefCell;
use cortex_m::interrupt::{free as interrupt_free, Mutex};
use microbit::display::nonblocking::Display;
use microbit::gpio::DisplayPins;
use microbit::pac;
use microbit::pac::TIMER1;

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

pub fn init_display(board_timer: TIMER1, board_display: DisplayPins) {
    let display = Display::new(board_timer, board_display);

    interrupt_free(move |cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER1)
    }
}

