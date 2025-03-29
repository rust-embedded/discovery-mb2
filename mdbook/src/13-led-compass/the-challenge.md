# The Challenge

We'll use some fancy math to get the precise angle that the magnetic field forms with the X and Y
axes of the magnetometer. This will allow us to figure out which LED is pointing north.

We'll use the `atan2` function. This function returns an angle in the `-PI` to `PI` range. The
graphic below shows how this angle is measured:

<p align="center">
<img class="white_bg" title="atan2" src="https://upload.wikimedia.org/wikipedia/commons/0/03/Atan2_60.svg" />
</p>

Although not explicitly shown, in this graph the X axis points to the right and the Y axis points
up. Note that our coordinate system is rotated 180° from this.

Here's the starter code (in `templates/compass.rs`). `theta`, in radians, has already been
computed. You need to pick which LED to turn on based on the value of `theta`.

```rs
{{#include templates/compass.rs}}
```

Suggestions/tips:

- A whole circle rotation equals 360 degrees.
- `PI` radians is equivalent to 180 degrees.
- If `theta` is zero, which direction are you pointing at?
- If `theta` is instead very close to zero, which direction are you pointing at?
- If `theta` keeps increasing, at what value should you change the direction
