#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::i2c::I2c;
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const WHO_AM_I_A: u8 = 0x0f;
const WHO_AM_I_M: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    );

    let mut acc = [0u8];
    let mut mag = [0u8];

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[WHO_AM_I_A], &mut acc).unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[WHO_AM_I_M], &mut mag).unwrap();

    rprintln!("The accelerometer chip's id is: {:08b}", acc[0]);
    rprintln!("The magnetometer chip's id is: {:08b}", mag[0]);

    loop {
        wfi();
    }
}
