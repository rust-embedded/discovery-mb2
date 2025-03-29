# LED roulette

Alright, let's build a "real" application. The goal is to get to this display of spinning lights:

<p align="center">
<video src="../assets/roulette_fast.mp4" width="500" loop autoplay/>
</p>

Since working with the LED pins separately is quite annoying (especially if you have to use
basically all of them like here) you can use the `microbit-v2` BSP crate, discussed previously, to
work with the MB2's LED "display". It works like this (`examples/light-it-all.rs`):

```rust
{{#include examples/light-it-all.rs}}
```

The Rust array `light_it_all` shown in the example contains 1 where the LED is on and 0 where it is
off.  The call to `show()` takes a timer for the BSP display code to use for delaying, a *copy* of
the array, and a length of time in milliseconds to show this display before returning.
