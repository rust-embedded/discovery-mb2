# My Solution

I found it a bit tricky to figure out how the interrupt
handler should calculate the next interrupt time to keep
the siren going. I ended up with a couple of state variables
to keep track of whether the speaker pin was on or off
(could have checked the hardware) and to keep track of what
time the siren was at in its up-down cycle.

My code contains all the details (`src/main.rs`).


```rust
{{#include src/main.rs}}
```
