#![deny(unsafe_code)]
#![no_main]
#![no_std]

const TICKS_PER_SEC: u32 = 400;
const THRESHOLD: f32 = 1.5;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, AccelScale, Lsm303agr};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut delay = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz400)
        .unwrap();
    // Allow the sensor to measure up to 16 G since human punches
    // can actually be quite fast
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut countdown_ticks = None;

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data() {
            nop();
        }
        // x acceleration in g
        let (x, _, _) = sensor.acceleration().unwrap().xyz_mg();
        let g_x = x as f32 / 1000.0;

        if let Some(ticks) = countdown_ticks {
            if ticks > 0 {
                // countdown isn't done yet
                if g_x > max_g {
                    max_g = g_x;
                }
                countdown_ticks = Some(ticks - 1);
            } else {
                // Countdown is done: report max value
                rprintln!("Max acceleration: {}g", max_g);

                // Reset
                max_g = 0.;
                countdown_ticks = None;
            }
        } else {
            // If acceleration goes above a threshold, we start measuring
            if g_x > THRESHOLD {
                rprintln!("START!");

                max_g = g_x;
                countdown_ticks = Some(TICKS_PER_SEC);
            }
        }
    }
}
