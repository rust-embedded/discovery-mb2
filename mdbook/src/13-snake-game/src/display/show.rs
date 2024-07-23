use super::DISPLAY;

use cortex_m::interrupt::free as interrupt_free;

use tiny_led_matrix::Render;

/// Display an image.
pub fn display_image(image: &impl Render) {
    interrupt_free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(image);
        }
    })
}

/// Clear the display (turn off all LEDs).
pub fn clear_display() {
    interrupt_free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}
