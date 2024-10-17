# Magnetometer Calibration

One very important thing to do before using a sensor and trying to develop an application using it
is verifying that it's output is actually correct.  If this does not happen to be the case we need
to calibrate the sensor. Alternatively the sensor could be broken: health-checking sensors before
and during use is a really good idea when possible.

In my case, on two different MB2s the LSM303AGR's magnetometer without calibration is quite a bit
off.  (I also have one where the z-axis appears to be broken; the manufacturer has some extra
hardware and a process to help detect this, but we won't deal with that complexity here.)

There is a manufacturer-specified procedure for calibrating the magnetometer.  The calibration
involves quite a bit of math (matrices) so we won't cover it in detail here: this [Design Note]
describes the procedure if you are interested in the details.

[Design Note]: https://www.st.com/resource/en/design_tip/dt0103-compensating-for-magnetometer-installation-error-and-hardiron-effects-using-accelerometerassisted-2d-calibration-stmicroelectronics.pdf

Luckily for us, the CODAL group that built the original C++ software for the micro:bit already
implemented the manufacturer calibration mechanism (or something similar) in C++ over [here].

[here]: https://github.com/lancaster-university/codal-microbit-v2/blob/006abf5566774fbcf674c0c7df27e8a9d20013de/source/MicroBitCompassCalibrator.cpp

You can find a translation of this C++ calibration to Rust in `src/lib.rs`. Note that this is a
translation from Matlab to C++ to Rust, and that it makes some interesting choices.  In particular,
when reading calibrated values *the axes are flipped* so that viewed from the top with the USB
connector forward the X, Y and Z axes of the calibrated value are in "standard" (right, forward, up)
orientation.

The usage of this calibrator is demonstrated in `src/main.rs` here.

The way the user does the calibration is shown in this video from the C++ version. (Ignore the
initial printing — the calibration starts about halfway through.)

<p align="center">
<video src="https://video.microbit.org/support/compass+calibration.mp4" loop autoplay>
</p>

You have to tilt the micro:bit until all the LEDs on the LED matrix light up. The blinking cursor
shows the current target LED.

Note that the calibration matrix is printed by the demo program. This matrix can be hard-coded into
a program such as the [chapter 12] compass program (or stored in flash somewhere somehow) to avoid
the need to recalibrate every time the user runs the program.

[chapter 12]: ../../12-led-compass/index.html
