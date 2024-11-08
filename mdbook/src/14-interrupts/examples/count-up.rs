#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    Board,
    hal::{
        gpiote,
        pac::{self, interrupt},
    },
};

struct Counter {
    count: usize,
    gpiote: gpiote::Gpiote,
}

static COUNTER: LockMut<Counter> = LockMut::new();

#[interrupt]
fn GPIOTE() {
    COUNTER.with_lock(|counter| {
        counter.count += 1;
        rprintln!("isr count: {}", counter.count);
        counter.gpiote.channel0().reset_events();
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
        .hi_to_lo()
        .enable_interrupt();
    channel.reset_events();
    let counter = Counter {
        count: 0,
        gpiote,
    };
    COUNTER.init(counter);

    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        asm::wfi();
        
        COUNTER.with_lock(|counter| {
            rprintln!("host count: {}", counter.count);
        });
    }
}
