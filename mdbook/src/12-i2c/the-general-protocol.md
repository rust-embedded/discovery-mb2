# General protocol

The I2C protocol is more elaborate than the serial communication protocol because it has to support communication between several devices.

The schematic shows controller (a microcontroller), three target nodes (an ADC, a DAC, and a microcontroller), and pull-up resistors Rp.

The devices are all connected to a serial bus.
<p align="center">
  <img class="white_bg" height="360" title="I2C bus" alt="The schematic shows controller (a microcontroller), three target nodes (an ADC, a DAC, and a microcontroller), and pull-up resistors Rp." src="../assets/i2c-controller-target.svg" />
</p>

Below, two examples of I2C communication between devices are described. Keep in mind the schematic shown just above.

## Example 1: Controller → Target

If the Controller wants to send data to the Target:

1. Controller: Broadcast START
2. C: Broadcast target address (7 bits) + the R/W (8th) bit set to WRITE
3. Target: Responds ACK (ACKnowledgement)
4. C: Send one byte
5. T: Responds ACK
6. Repeat steps 4 and 5 zero or more times
7. C: Broadcast STOP OR (broadcast RESTART and go back to (2))

> **NOTE** The target address could have been 10 bits instead of 7 bits long. Nothing else would have
> changed.

## Example 2: Controller ← Target

If the controller wants to read data from the target:

1. C: Broadcast START
2. C: Broadcast target address (7 bits) + the R/W (8th) bit set to READ
3. T: Responds with ACK
4. T: Send byte
5. C: Responds with ACK
6. Repeat steps 4 and 5 zero or more times
7. C: Broadcast STOP OR (broadcast RESTART and go back to (2))

> **NOTE** The target address could have been 10 bits instead of 7 bits long. Nothing else would
> have changed.
