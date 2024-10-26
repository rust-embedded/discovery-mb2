#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    Board,
    hal::{
        gpiote,
        pac::{self, interrupt},
    },
};

static TE: Mutex<RefCell<Option<gpiote::Gpiote>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    rprintln!("ouch");
    critical_section::with(|cs| {
        let cell = TE.borrow(cs).borrow();
        let channel = cell.as_ref().unwrap().channel0();
        channel.reset_events();
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_floating_input();
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel = gpiote.channel0();
    channel
        .input_pin(&button_a.degrade())
        .lo_to_hi()
        .enable_interrupt();
    channel.reset_events();
    
    critical_section::with(|cs| {
        let mut cell = TE.borrow(cs).borrow_mut();
        *cell = Some(gpiote);
    });
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    loop {
        asm::wfi();
        rprintln!("got poked");
    }
}
