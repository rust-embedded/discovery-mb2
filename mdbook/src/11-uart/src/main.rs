#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use microbit::hal::uarte::{self, Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            // We assume that the receiving cannot fail
            let byte = serial.read().unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }

            if byte == b'\r' {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    serial.write(*byte).unwrap();
                }
                break;
            }
        }
        serial.flush().unwrap()
    }
}
