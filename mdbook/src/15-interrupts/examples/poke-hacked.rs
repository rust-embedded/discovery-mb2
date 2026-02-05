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

static mut GPIOTE_PERIPHERAL: Option<gpiote::Gpiote> = None;

/// This "function" will be called when an interrupt is
/// received. For now, just report and panic.
#[interrupt]
fn GPIOTE() {
    rprintln!("ouch");
    // # Safety
    // Ideally, this ISR cannot be called until the global
    // has been properly initialized. This value is never
    // touched globally after that.
    unsafe {
        if let Some(ref mut gpiote) = GPIOTE_PERIPHERAL {
            let channel = gpiote.channel0();
            channel.reset_events();
            // panic!();
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_floating_input();
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);

    // Set up the GPIOTE to generate an interrupt when Button A is pressed (GPIO
    // wire goes low).
    let channel = gpiote.channel0();
    channel
        .input_pin(&button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel.reset_events();
    // # Safety
    // GPIOTE cannot be used from interrupt handler yet,
    // since it's not turned on. (I think this is bogus; the
    // write is not guaranteed to complete before the end of
    // the function maybe? An `UnsafeCell` is probably needed
    // here, at bare minimum.)
    unsafe { GPIOTE_PERIPHERAL = Some(gpiote) };

    // Set up the NVIC to handle GPIO interrupts.
    // # Safety
    // Interrupt handler is set up properly.
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        // "wait for interrupt": CPU goes to sleep until an interrupt.
        asm::wfi();
    }
}
