use super::{Buttons, GPIO};

use cortex_m::interrupt::free as interrupt_free;
use microbit::{
    hal::{
        gpio::{Floating, Input, Pin},
        gpiote::{Gpiote, GpioteChannel},
    },
    pac,
};

/// Initialise the buttons and enable interrupts.
pub fn init_buttons(board_gpiote: pac::GPIOTE, board_buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    fn init_channel(channel: &GpioteChannel<'_>, button: &Pin<Input<Floating>>) {
        channel.input_pin(button).hi_to_lo().enable_interrupt();
        channel.reset_events();
    }

    let channel0 = gpiote.channel0();
    init_channel(&channel0, &board_buttons.button_a.degrade());

    let channel1 = gpiote.channel1();
    init_channel(&channel1, &board_buttons.button_b.degrade());

    interrupt_free(move |cs| {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });
}
