# Punch-o-meter

In this section we'll be playing with the accelerometer that's in the board.

What are we building this time? A punch-o-meter! We'll be measuring the power of your jabs. Well,
actually the maximum acceleration that you can reach because acceleration is what accelerometers
measure. Strength and acceleration are proportional though so it's a good approximation.

As we already know from previous chapters the accelerometer is built inside the LSM303AGR package.
And just like the magnetometer, it is accessible using the
I2C bus. 

The accelerometer also has the same coordinate system as the magnetometer. Here's a reminder:

<p align="center">
<img title="MB2 Axes" src="../assets/mb2-axes.jpg" width="500" />
</p>
