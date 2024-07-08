#![no_std]

use core::fmt;
use embedded_io::{Read, Write};
use microbit::hal::uarte::{self, Instance, Uarte, UarteRx, UarteTx};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

#[allow(unused)]
pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

impl<T: Instance> UartePort<T> {
    pub fn new(serial: Uarte<T>) -> UartePort<T> {
        // XXX Fix me: Need to use `split()` in a safe way if possible.
        // This may require an API change in the `nrf-hal` crate.
        #[allow(static_mut_refs)]
        let (tx, rx) = serial
            .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
            .unwrap();
        UartePort(tx, rx)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> UartePort<T> {
    pub fn write(&mut self, b: u8) -> Result<(), uarte::Error> {
        self.0.write(&[b])?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), uarte::Error> {
        self.0.flush()
    }

    #[allow(unused)]
    pub fn read(&mut self) -> Result<u8, uarte::Error> {
        let mut buf = [0u8; 1];
        self.1.read(&mut buf)?;
        Ok(buf[0])
    }
}
