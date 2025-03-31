# Gravity is up?

What's the first thing we'll do?

Perform a sanity check!

You should already be able to write a program that continuously prints the accelerometer data on the
RTT console from the [I2C chapter](../12-i2c/index.md). Mine is in `examples/show-accel.rs`. Do you
observe something interesting even when holding the board parallel to the floor with the back side
facing up?  (Remember that the accelerometer is mounted on the back of the board, so holding it
upside-down like this makes the Z axis point up.)

What you should see when holding the board like this is that both the X and Y values are rather
close to 0, while the Z value is at around 1000. Which is weird: the board is not moving, yet its
acceleration is non-zero. What's going on? This must be related to the gravity, right? Because the
acceleration of gravity is `1 g` (aha, `1 g` = -1000 from the accelerometer). But the gravity pulls
objects downwards so the acceleration along the Z axis should be positive, not negative.

Did the program get the Z axis backwards? Nope, you can test rotating the board to align the gravity
to the X or Y axis but the acceleration measured by the accelerometer is always pointing up.

What happens here is that the accelerometer is measuring the *proper acceleration* of the board, not
the acceleration *you* are observing. This proper acceleration is the acceleration of the board as
seen from an observer that's in free fall. An observer that's in free fall is moving toward the
center of the Earth with an acceleration of `1g`; from its point of view the board is actually
moving upwards (away from the center of the Earth) with an acceleration of `1g`. And that's why the
proper acceleration is pointing up. This also means that if the board was in free fall, the
accelerometer would report a proper acceleration of zero. Please, don't try that at home. Or do, if
you're willing to risk your MB2 by dropping it.

Yes, physics is hard. Let's move on.
