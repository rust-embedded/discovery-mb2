#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const PORT_P0_OUT: u32 = 0x50000504;

        // Turn on the top row
        *(PORT_P0_OUT as *mut u32) |= 1 << 21;

        // Turn on the bottom row
        *(PORT_P0_OUT as *mut u32) |= 1 << 19;

        // Turn off the top row
        *(PORT_P0_OUT as *mut u32) &= !(1 << 21);

        // Turn off the bottom row
        *(PORT_P0_OUT as *mut u32) &= !(1 << 19);
    }

    loop {}
}
