# Send a single byte

Our first task will be to send a single byte from the microcontroller to the computer over the
serial connection.

In order to do that we will use the following snippet (this one is already in
`11-uart/examples/send-byte.rs`):

``` rust
{{#include examples/send-byte.rs}}
```

You might notice that one of the libraries used here, the `serial_setup` module, is not from
`crates.io`, but was written for this project. The purpose of `serial_setup` is to provide a nice
wrapper around the UARTE peripheral. If you want, you can check out what exactly the module does,
but it is not required to understand this chapter in general.

We'll next discuss the initialization of UARTE. The UARTE is initialized with this piece of code:

```rs
uarte::Uarte::new(
    board.UARTE0,
    board.uart.into(),
    Parity::EXCLUDED,
    Baudrate::BAUD115200,
);
```

This function takes ownership of the UARTE peripheral representation in Rust (`board.UARTE0`) and
the TX/RX pins on the board (`board.uart.into()`) so nobody else can mess with either the UARTE
peripheral or our pins while we are using them. After that we pass two configuration options to the
constructor: the baud rate (that one should be familiar) as well as an option called
"parity". Parity is a way to allow serial communication lines to check whether the data they
received was corrupted during transmission. We don't want to use that here so we simply exclude it.
Then we wrap it up in the `UartePort` type so we can use it.

After the initialization, we send our `X` (as ASCII byte value 88) via the newly created uart
instance. These serial functions are "blocking": they wait for the data to be sent before
returning. This is not always what is wanted: the microcontroller can do a lot of work while
waiting for the byte to go out on the wire. However, in our case it is convenient and we didn't
have other work to do anyway.

Last but not least, we `flush()` the serial port. This is because the UARTE may decide to buffer
output until it has received a certain number of bytes to send.  Calling `flush()` forces it to
write the bytes it currently has right now instead of waiting for more.

## Testing it

Before flashing this you should make sure to start your minicom/PuTTY as the data we receive via
our serial communication is not backed up or anything: we have to view it live. Once your serial
monitor is up you can flash the program just like in chapter 5:

```
$ cargo embed --example send-byte
  (...)
```

And after the flashing is finished, you should see the character `X` show up on your minicom/PuTTY
terminal, congrats!

If you missed it, you can hit the reset button on the back of the MB2. This will cause the program
to start from the beginning and send an `X` again.
