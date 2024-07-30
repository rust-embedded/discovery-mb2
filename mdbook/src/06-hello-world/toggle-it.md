# Toggle it

Let's turn the LED on and off repeatedly. That's how you make it blink, right?

In `examples/fast-blink.rs` you'll find the next iteration of our blinky. I've decided to make it
blink the next LED over, while leaving the original LED on. That is an easy change.

```rust
{{#include examples/fast-blink.rs}}
```

The `embedded-hal` crate is being used here to provide the Rust traits needed to set and unset the
LED. This means that this part of the code is portable to any Rust HAL that implements the
`embedded-hal` traits as ours does.

But wait: neither LED is blinking! The second one is slightly dimmer than the first one, but they
are both solidly on… or are they? Out of the box, the MB2 executes 64 *million* instructions per
second. Let's assume it takes a few dozen instructions under the hood to turn the LED on or
off. (Maybe possibly that many compiled in debug mode, though way less in release mode. Though the
pins take a while to change state. I don't know.) Anyhow, that second LED is actually turning on and
off hundreds of thousands of times — perhaps millions of times — every second. Your eye just can't
keep up.

We'll need to wait a while between toggles. Turns out waiting is the hardest part.
