#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::{Measurement, calc_calibration, calibrated_measurement};

use embedded_hal::delay::DelayNs;
use microbit::{
    display::blocking::Display,
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr, MagMode, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer0 = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(
        &mut timer0,
        MagMode::HighResolution,
        MagOutputDataRate::Hz10,
    ).unwrap();
    sensor.set_accel_mode_and_odr(
        &mut timer0,
        AccelMode::HighResolution,
        AccelOutputDataRate::Hz10,
    ).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer0);
    rprintln!("Calibration: {:?}", calibration);
    rprintln!("Calibration done, entering busy loop");
    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {
            timer0.delay_ms(1u32);
        }
        let mut data = Measurement::new(
            sensor.magnetic_field().unwrap().xyz_nt()
        );
        data = calibrated_measurement(data, &calibration);
        rprintln!("x: {}, y: {}, z: {}", data.x, data.y, data.z);
    }
}
