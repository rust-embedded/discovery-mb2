#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        ptr::read_volatile(0x5000_A784 as *const u32);
    }

    loop {}
}
