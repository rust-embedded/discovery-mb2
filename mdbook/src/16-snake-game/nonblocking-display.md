# Using the non-blocking display

We will next display the snake and food on the LEDs of the MB2 screen. So far, we have used the
blocking interface, which provides for LEDs to be either maximally bright or turned off. With this,
a basic functioning snake game would be possible. But you might find that when the snake got a bit
longer, it would be difficult to tell the snake from the food, and to tell which direction the snake
was heading. Let's figure out how to allow the LED brightness to vary: we can make the snake's body
a bit dimmer, which will help sort out the clutter.

The `microbit` library makes available two different interfaces to the LED matrix. There is the
blocking interface we've already seen in previous chapters. There is also a non-blocking interface
which allows you to customise the brightness of each LED. At the hardware level, each LED is either
"on" or "off", but the `microbit::display::nonblocking` module simulates ten levels of brightness
for each LED by rapidly switching the LED on and off.

(There is no great reason the two display modes of the `microbit` library crate have to be separate
and use separate code. A more complete design would allow either non-blocking or blocking use of a
single display API with variable brightness levels and refresh rates specified by the user. Never
assume that the stuff you have been handed is perfected, or even close. Always think about what you
might do differently. For now, though, we'll work with what we have, which is adequate for our
immediate purpose.)

The code to interact with the non-blocking interface (`src/display.rs`) is pretty simple and will
follow a similar structure to the code we used to interact with the buttons. This time we'll start
at the top level.

## Display module

```rust
{{#include src/display.rs}}
```

First, we initialize a `microbit::display::nonblocking::Display` struct representing the LED
display, passing it the board's `TIMER1` and `DisplayPins` peripherals. Then we store the display in
a Mutex. Finally, we unmask the `TIMER1` interrupt.

## Display API

We then define a couple of convenience functions which allow us to easily set (or unset) the image
to be displayed (`src/display/show.rs`).

```rust
{{#include src/display/show.rs}}
```

`display_image` takes an image and tells the display to show it. Like the `Display::show` method
that it calls, this function takes a struct that implements the `tiny_led_matrix::Render`
trait. That trait ensures that the struct contains the data and methods necessary for the `Display`
to render it on the LED matrix. The two implementations of `Render` provided by the
`microbit::display::nonblocking` module are `BitImage` and `GreyscaleImage`. In a `BitImage`, each
"pixel" (or LED) is either illuminated or not (like when we used the blocking interface), whereas in
a `GreyscaleImage` each "pixel" can have a different brightness.

`clear_display` does exactly as the name suggests.

## Display interrupt handling

Finally, we use the `interrupt` macro to define a handler for the `TIMER1` interrupt. This interrupt
fires many times a second, and this is what allows the `Display` to rapidly cycle the different LEDs
on and off to give the illusion of varying brightness levels. All our handler code does is call the
`Display::handle_display_event` method, which handles this (`src/display/interrupt.rs`).

```rust
{{#include src/display/interrupt.rs}}
```

Now we can understand how our `main` function will do display: we will call `init_display` and use
the new functions we have defined to interact with it.
