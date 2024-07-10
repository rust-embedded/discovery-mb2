# Receive a single byte

So far we can send data from the microcontroller to your computer. It's time to try the opposite:
receiving data from your computer (`examples/receive-byte.rs`).

``` rust
{{#include examples/receive-byte.rs}}
```

The only part that changed, compared to our send byte program, is the loop at the end of
`main()`. Here we use the `serial.read()` function in order to wait until a byte is available and
read it. Then we print that byte into our RTT debugging console to see whether stuff is actually
arriving.

Note that if you flash this program and start typing characters inside `minicom` to send them to
your microcontroller you'll only be able to see numbers inside your RTT console since we are not
converting the `u8` we received into an actual `char`.  Since the conversion from `u8` to `char` is
quite simple, I'll leave this task to you if you really do want to see the characters inside the RTT
console.
