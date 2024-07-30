#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use registers::entry;

#[entry]
fn main() -> ! {
    registers::init();

    unsafe {
        ptr::read_volatile(0x5000_A784 as *const u32);
    }

    loop {}
}
