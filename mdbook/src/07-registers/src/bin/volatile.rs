#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const PORT_P0_OUT: u32 = 0x50000504;

        // Turn on the top row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 21);

        // Turn on the bottom row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 19);

        // Turn off the top row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 21));

        // Turn off the bottom row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 19));
    }

    loop {}
}