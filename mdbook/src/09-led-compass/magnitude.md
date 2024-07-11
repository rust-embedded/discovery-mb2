# Magnitude

We have been working with the direction of the magnetic field, but how strong is it?  According to
the documentation about the [`magnetic_field()`] method the `x` `y` `z` values we are getting are in
nanoteslas. That means the only thing we have to compute in order to get the magnitude of the
magnetic field in nanoteslas is the magnitude of the 3D vector that our `x` `y` `z` values
describe. As you might remember from school this is simply:

``` rust
// core doesn't have this function yet so we use libm, just like with
// atan2f from before.
use libm::sqrtf;
let magnitude = sqrtf(x * x + y * y + z * z);
```

[`magnetic_field()`]: https://docs.rs/lsm303agr/1.1.0/lsm303agr/struct.Lsm303agr.html#method.magnetic_field


Putting all this together in a program (`src/bin/magnitude.rs`):

``` rust
{{#include src/bin/magnitude.rs}}
```

Run this with `cargo run --bin magnitude`.

This program will report the magnitude (strength) of the magnetic field in nanotesla (`nT`) and
milligauss (`mG`). The magnitude of the Earth's magnetic field is in the range of `250 mG` to `650
mG` (the magnitude varies depending on your geographical location) so you should see a value in that
range or close to that range -- I see a magnitude of around `340 mG`.

Some questions:

- Without moving the board, what value do you see? Do you always see the same value?

- If you rotate the board, does the magnitude change? Should it change?
