#![no_main]
#![no_std]

use aux9::{entry, pac, PIXELS};
use embedded_hal::digital::OutputPin;

#[inline(never)]
fn delay(timer0: &pac::timer0::RegisterBlock, ms: u16) {
    const K: u16 = 3; // this value needs to be tweaked
    for _ in 0..(K * ms) {
        aux9::nop()
    }
}

#[entry]
fn main() -> ! {
    let (mut leds, timer0) = aux9::init();

    // TODO initialize TIMER0
    let mut last_led = aux9::PIXELS[0];
    let ms = 50;

    loop {
        for current_led in PIXELS.iter() {
            leds.0[last_led.0].set_high();
            leds.1[last_led.1].set_low();

            leds.0[current_led.0].set_low();
            leds.1[current_led.1].set_high();

            delay(timer0, ms);

            last_led = *current_led;
        }
    }
}
