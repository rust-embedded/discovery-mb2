# General protocol

The I2C protocol is more elaborate than the serial communication protocol because it supports
structured communication between several devices. Let's see how it works:

## Controller → Target

If the Controller wants to send data to the Target:

1. Controller: Broadcast START
2. C: Broadcast target address (7 bits) + the R/W (8th) bit set to WRITE
3. Target: Responds ACK (ACKnowledgement)
4. C: Send one byte
5. T: Responds ACK
6. Repeat steps 4 and 5 zero or more times
7. C: Broadcast STOP, or begin a new read transaction

> **NOTE** The target address could have been 10 bits instead of 7 bits long. Nothing else would
> have changed.

## Controller ← Target

If the controller wants to read data from the target:

1. C: Broadcast START
2. C: Broadcast target address (7 bits) + the R/W (8th) bit set to READ
3. T: Responds with ACK
4. T: Send byte
5. C: Responds with ACK
6. Repeat steps 4 and 5 zero or more times
7. C: Broadcast STOP, or begin a new write transaction

> **NOTE** The target address could have been 10 bits instead of 7 bits long. Nothing else would
> have changed.

## "Device Registers"

Many I2C targets are organized internally as having "device registers", each with an 8-bit address
and 8-bit contents. Typically, device registers are written with a two-byte write: the first byte is
the register address and the second the new register value.

A so-called "combined" or "split" transaction might consist of a write to the target followed by an
immediate read back from the target, as shown in the diagram above. Typically, device registers are
read in this way: the device register address is written and then the current device register value
is immediately read back.

Some I2C targets can read and write multiple device registers with adjacent addresses through some
form of "address auto-increment", which permits sending just the first device register address and
then relying on the device to increment the address for subsequent reads or writes.

I2C is a complex protocol, and there are many variations and special features out there. Read the
manual for your target carefully to see what needs to be done to talk to it.
