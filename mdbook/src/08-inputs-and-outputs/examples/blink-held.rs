#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::timer::Timer;
use microbit::{hal::gpio, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

const ON_TICKS: u16 = 25;
const OFF_TICKS: u16 = 75;

#[derive(Clone, Copy)]
enum Light {
    Lit(u16),
    Unlit(u16),
}

impl Light {
    fn flip(self) -> Self {
        match self {
            Light::Lit(_) => Light::Unlit(OFF_TICKS),
            Light::Unlit(_) => Light::Lit(ON_TICKS),
        }
    }

    fn tick_down(self) -> Self {
        match self {
            Light::Lit(ticks) => Light::Lit(ticks.max(1) - 1),
            Light::Unlit(ticks) => Light::Unlit(ticks.max(1) - 1),
        }
    }
}

#[derive(Clone, Copy)]
enum Indicator {
    Off,
    Blinking(Light),
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configure buttons
    let mut button_a = board.buttons.button_a;

    // Configure LED (top-left LED at row1, col1)
    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(gpio::Level::Low);
    let _col1 = board
        .display_pins
        .col1
        .into_push_pull_output(gpio::Level::Low);

    let mut state = Indicator::Off;
    loop {
        let button_pressed = button_a.is_low().unwrap();
        match (button_pressed, state) {
            // Turn indicator off when no button.
            (false, _) => {
                row1.set_low().unwrap();
                state = Indicator::Off;
            }
            //
            (true, Indicator::Off) => {
                row1.set_high().unwrap();
                state = Indicator::Blinking(Light::Lit(ON_TICKS));
            }
            (true, Indicator::Blinking(light)) => match light {
                Light::Lit(0) | Light::Unlit(0) => {
                    let light = light.flip();
                    match light {
                        Light::Lit(_) => row1.set_high().unwrap(),
                        Light::Unlit(_) => row1.set_low().unwrap(),
                    }
                    state = Indicator::Blinking(light);
                }
                Light::Lit(_) | Light::Unlit(_) => {
                    state = Indicator::Blinking(light.tick_down());
                }
            },
        }
        timer.delay_ms(10_u32);
    }
}
