# Magnitude

How strong is the Earth's magnetic field?  According to the documentation about the
[`magnetic_field()`] method the `x` `y` `z` values we are getting are in nanoteslas. That means the
only thing we have to compute in order to get the magnitude of the magnetic field in nanoteslas is
the magnitude of the 3D vector that our `x` `y` `z` values describe. As you might remember from
school this is simply:

[`magnetic_field()`]: https://docs.rs/lsm303agr/1.1.0/lsm303agr/struct.Lsm303agr.html#method.magnetic_field

Rust does not have floating-point math functions such as `sqrtf()` in `core`, so our `no_std`
program has to get an implementation from somewhere. We use the [libm] crate for this.

[libm]: https://crates.io/crates/libm

``` rust
use libm::sqrtf;
let magnitude = sqrtf(x * x + y * y + z * z);
```




Putting all this together in a program (`examples/magnitude.rs`):

``` rust
{{#include examples/magnitude.rs}}
```

Run this with `cargo run --bin magnitude`.

This program will report the magnitude (strength) of the magnetic field in nanotesla (`nT`) and
milligauss (`mG`, where 1 `mG` = 100 `nT`). The magnitude of the Earth's magnetic field is in the
range of `250 mG` to `650 mG` (the magnitude varies depending on your geographical location) so you
ideally would see a value vaguely in that range. Your value will likely be off quite a bit because
the sensor has not been calibrated: see [appendix 3] for calibration. With calibration, I see a
magnitude of around `340 mG`.

[appendix 3]: ../appendix/3-mag-calibration/index.html

Some questions:

- Without moving the board, what value do you see? Do you always see the same value?

- If you rotate the board, does the magnitude change? Should it change?
