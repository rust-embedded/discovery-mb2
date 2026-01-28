#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// You'll find these useful ;-).
use core::f32::consts::PI;
use libm::{atan2f, floorf};

use microbit::{
    display::blocking::Display,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{Lsm303agr, MagMode, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer0 = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut timer0,
            MagMode::HighResolution,
            MagOutputDataRate::Hz10,
        )
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut leds = [[0u8; 5]; 5];

    // Indexes of the 16 LEDs to be used in the display, and their
    // compass directions.
    #[rustfmt::skip]
    let indices = [
        (2, 0), /* W */
        (3, 0), /* W-SW */
        (3, 1), /* SW */
        (4, 1), /* S-SW */
        (4, 2), /* S */
        (4, 3), /* S-SE */
        (3, 3), /* SE */
        (3, 4), /* E-SE */
        (2, 4), /* E */
        (1, 4), /* E-NE */
        (1, 3), /* NE */
        (0, 3), /* N-NE */
        (0, 2), /* N */
        (0, 1), /* N-NW */
        (1, 1), /* NW */
        (1, 0), /* W-NW */
    ];

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {
            timer0.delay_ms(1u32);
        }
        let (x, y, _) = sensor.magnetic_field().unwrap().xyz_nt();

        // Get an angle between -180° and 180° from the x axis.
        let theta = atan2f(y as f32, x as f32);

        // Cut the unit circle into thirty-two segments,
        // with pairs of adjacent segments corresponding to
        // each compass direction.
        let seg = floorf(16.0 * theta / PI) as i8;

        // Figure out what LED index to blink.
        let index = if seg >= 15 || seg <= -15 {
            8
        } else if seg >= 0 {
            (seg / 2) as usize
        } else {
            ((31 + seg) / 2) as usize
        };

        // Blink the given LED.
        let (r, c) = indices[index];
        leds[r][c] = 255u8;
        display.show(&mut timer0, leds, 50);
        leds[r][c] = 0u8;
        display.show(&mut timer0, leds, 50);
    }
}
