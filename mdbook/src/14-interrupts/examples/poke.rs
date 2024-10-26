#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    Board,
    hal::{
        gpiote,
        pac::{self, interrupt},
    },
};

#[interrupt]
fn GPIOTE() {
    rprintln!("ouch");
    asm::bkpt();
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
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    loop {
        asm::wfe();
    }
}
