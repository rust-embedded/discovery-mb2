#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, rprintln};

// Print the current contents of P0.OUT
fn print_out() {
    const P0_OUT: u32 = 0x5000_0504;

    let out = unsafe { ptr::read_volatile(P0_OUT as *const u32) };

    rprintln!("P0.OUT = {:#08x}", out);
}

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A bunch of magic addresses!
        const P0_OUTSET: u32 = 0x5000_0508;
        const P0_OUTCLR: u32 = 0x5000_050C;

        // Print the initial contents of OUT
        print_out();

        // Turn on the top LED row
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 21);
        print_out();

        // Turn on the bottom LED row
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 19);
        print_out();

        // Turn off the top LED row
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 21);
        print_out();

        // Turn off the bottom LED row
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 19);
        print_out();
    }

    loop {}
}
