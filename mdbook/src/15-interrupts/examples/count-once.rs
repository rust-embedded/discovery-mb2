#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        gpiote,
        pac::{self, interrupt},
    },
    Board,
};

static COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// This "function" will be called when an interrupt is received. For now, just
/// report and panic.
#[interrupt]
fn GPIOTE() {
    critical_section::with(|cs| {
        let mut count = COUNTER.borrow(cs).borrow_mut();
        *count += 1;
        rprintln!("count: {}", count);
    });
    panic!();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_floating_input();

    // Set up the GPIOTE to generate an interrupt when Button A is pressed (GPIO
    // wire goes low).
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel = gpiote.channel0();
    channel
        .input_pin(&button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel.reset_events();

    // Set up the NVIC to handle GPIO interrupts.
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        // "wait for interrupt": CPU goes to sleep until an interrupt.
        asm::wfi();
    }
}
