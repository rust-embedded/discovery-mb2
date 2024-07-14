# Take 1

What's the simplest way in which we can implement the LED compass, even if it's not perfect?

For starters, we'd only care about the X and Y components of the magnetic field because when you
look at a compass you always hold it in horizontal position and thus the compass is in the XY plane.

<p align="center">
<img class="white_bg" title="Quadrants" src="../assets/quadrants.jpg" width="500" />
</p>

If we only looked at the signs of the X and Y components we could determine to which quadrant the
magnetic field belongs to. Now the question of course is which direction (north, north-east, etc.)
do the 4 quadrants represent. In order to figure this out we can just rotate the micro:bit and
observe how the quadrant changes whenever we point in another direction.

After experimenting a bit we can find out that if we point the micro:bit in e.g. north-east
direction, both the X and the Y component are always negative (quadrant III). Based on this
information you should be able to figure out which direction the other quadrants represent.

Once you figured out the relation between quadrant and direction you should be able to complete the
template in `templates/take_1.rs` to make the compass arrow point in the correct direction.
