#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::hal::{
    gpio,
    gpiote,
    pac::{self, interrupt},
    timer,
};

struct Blinker {
    period: u32,
    timer: timer::Timer<>

static BLINKER: Option<Mutex<RefCell

#[interrupt]
fn TIMER0() {
    rprintln!("tick");
}

#[entry]
fn main() -> ! {
    
}
