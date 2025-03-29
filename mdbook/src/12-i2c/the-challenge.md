# The challenge

The challenge for this chapter is, to build a small application that communicates with the outside
world via the serial interface introduced in the last chapter. It should be able to receive the
commands "mag" for magnetometer as well as "acc" for accelerometer. It should then print the
corresponding sensor data to the serial port in response. This time no template code will be
provided since all you need is already provided in the [UART](../10-uart/index.html) and this
chapter. However, here are a few clues:

-   You might be interested in `core::str::from_utf8` to convert the bytes in the buffer to a `&str`, since we need to compare with `"mag"` and `"acc"`.
-   You will have to read the documentation for the magnetometer API and functionality. While the `lsm303agr` crate provides the API interface, the [LSM303AGR datasheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf) details the sensor's magnetic field measurement parameters. See pages 13-15 for sensor characteristics and, importantly, pages 66-67 for the output register format.
