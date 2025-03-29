#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::uarte::{self, Baudrate, Parity},
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use core::fmt::Write;
use heapless::Vec;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr, MagMode, MagOutputDataRate};

use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let mut serial = UartePort::new(serial);

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut timer0,
            MagMode::HighResolution,
            MagOutputDataRate::Hz50,
        )
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte = serial.read().unwrap();

            if byte == b'\r' {
                break;
            }

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }
        }

        if str::from_utf8(&buffer).unwrap().trim() == "accelerometer" {
            while !sensor.accel_status().unwrap().xyz_new_data() {
                timer0.delay_ms(1u32);
            }

            let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
            write!(serial, "Accelerometer: x {} y {} z {}\r\n", x, y, z).unwrap();
        } else if str::from_utf8(&buffer).unwrap().trim() == "magnetometer" {
            while !sensor.mag_status().unwrap().xyz_new_data() {
                timer0.delay_ms(1u32);
            }

            let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();
            write!(serial, "Magnetometer: x {} y {} z {}\r\n", x, y, z).unwrap();
        } else {
            write!(serial, "error: command not detected\r\n").unwrap();
        }
    }
}
