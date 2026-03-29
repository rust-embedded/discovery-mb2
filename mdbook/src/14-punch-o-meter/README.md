# Punch-o-meter

In this section we'll be playing with the accelerometer that's in the board.

What are we building this time? A [punch-o-meter]! We'll be measuring the power of your jabs. Well,
actually the maximum acceleration that you can reach because acceleration is what accelerometers
measure. Strength and acceleration are proportional though so it's a good approximation.

As we already know from [previous chapters](../12-i2c/index.html) the accelerometer is built inside
the [`lsm303agr`] package. Just like the magnetometer, it is accessible using the I2C bus. The
accelerometer also has the same coordinate system as the magnetometer.

[punch-o-meter]: https://en.wikipedia.org/wiki/Strength_tester_machine
[`lsm303agr`]: https://crates.io/crates/lsm303agr
