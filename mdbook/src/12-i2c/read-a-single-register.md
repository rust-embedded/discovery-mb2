# Read a single register

Let's put all that theory into practice!

First things first we need to know the target addresses of both the accelerometer and the
magnetometer inside the chip, these can be found in the LSM303AGR's datasheet on page 39 and are:

- 0011001 for the accelerometer
- 0011110 for the magnetometer

> **NOTE** Remember that these are only the 7 leading bits of the address, the 8th bit is going to
> be the bit that determines whether we are performing a read or write.

Next up we'll need a register to read from. Lots of I2C chips out there will provide some sort of
device identification register for their controllers to read. Considering the thousands (or even
millions) of I2C chips out there it is highly likely that at some point two chips with the same
address will end up being built (after all the address is "only" 7 bit wide). With this device ID
register a driver can make sure that it is indeed talking to a LSM303AGR and not some other chip
that just happens to have the same address.  As you can read in the LSM303AGR's datasheet
(specifically on page 46 and 61) this part does provide two registers — `WHO_AM_I_A` at address
`0x0f` and `WHO_AM_I_M` at address `0x4f` — which contain some bit patterns that are unique to the
device. (The "A" is for "Accelerometer" and the "M" is for "Magnetometer".)

The only thing missing now is the software part: we need to determin which API of the `microbit` or
a HAL crate we should use for this. If you read through the datasheet of the nRF chip you are using
you will soon find out that it doesn't actually have an I2C-specific peripheral.  Instead, it has
more general-purpose I2C-compatible peripherals called TWI ("Two-Wire Interface"), TWIM ("Two-Wire
Interface Master") and TWIS ("Two-Wire Interface Slave"). We will normally be operating in
controller mode and will use the newer TWIM, which supports "Easy DMA" — the TWI is provided mostly
for backward compatibility with older devices.

Now if we put the documentation of the [`twi(m)` module] from the `microbit` crate
together with all the other information we have gathered so far we'll end up with this
piece of code to read out and print the two device IDs (`examples/chip-id.rs`):

[`twi(m)` module]: https://docs.rs/microbit-v2/0.11.0/microbit/hal/twim/index.html

``` rust
{{#include examples/chip-id.rs}}
```

Apart from the initialization, this piece of code should be straight forward if you understood the
I2C protocol as described before. The initialization here works similarly to the one from the UART
chapter.  We pass the peripheral as well as the pins that are used to communicate with the chip to
the constructor; and then the frequency we wish the bus to operate on, in this case 100 kHz (`K100`,
since identifiers can't start with a digit).

## Testing it
As usual

```console
$ cargo embed --example chip-id
```
in order to test our little example program.
