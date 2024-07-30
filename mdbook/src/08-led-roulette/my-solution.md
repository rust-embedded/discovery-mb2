# My solution

What solution did you come up with?

Here's mine. It's probably one of the simplest (but of course not most beautiful) ways to generate
the required matrix:

``` rust
{{#include src/main.rs}}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
$ cargo embed --release
```

If you want to debug your "release" mode binary you'll have to use a different GDB command:

``` console
$ gdb ../../../target/thumbv7em-none-eabihf/release/led-roulette
```

The Rust compiler modifies the machine instructions generated in a release build (sometimes by a
lot) in order to try to make the code faster or smaller. Unfortunately, GDB has a hard time figuring
out what is going on after this. As a result, debugging release builds with GDB can be difficult.

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
$ cargo size --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.02s
led-roulette  :
section              size        addr
.vector_table         256         0x0
.text                6332       0x100
.rodata               648      0x19bc
.data                   0  0x20000000
.bss                 1076  0x20000000
.uninit                 0  0x20000434
.debug_loc           9036         0x0
.debug_abbrev        2754         0x0
.debug_info         96460         0x0
.debug_aranges       1120         0x0
.debug_ranges       11520         0x0
.debug_str          71325         0x0
.debug_pubnames     32316         0x0
.debug_pubtypes     29294         0x0
.ARM.attributes        58         0x0
.debug_frame         2108         0x0
.debug_line         19303         0x0
.comment              109         0x0
Total              283715
```

Your numbers may differ somewhat depending on how your code is built: this is OK.

Know how to read this output? The `text` section contains the program instructions. The `rodata`
section contains read-only data stored with the program instructions. The `data` and `bss` sections
contain variables statically allocated in RAM (`static` variables).  If you remember the
specification of the microcontroller on your micro:bit, you should notice that its flash memory is
less than double the size of this extremely simple binary: can this be right?  As we can see from
the size statistics most of the binary is actually made up of debugging related sections. However,
those are not flashed to the microcontroller at any time — after all they aren't relevant for the
execution.
