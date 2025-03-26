## Debouncing

As I mentioned in the last section, hardware can be a little… special. This is definitely the case
for the buttons on the MB2, and really for almost any pushbutton or switch in almost any system. If
you are seeing several interrupts for a single keypress, it is probably the result of what is known
as switch "bouncing". This is literally what the name implies: as the electrical contacts of the
switch come together, they may bounce apart and then recontact several times rather quickly before
establishing a solid connection. Unfortunately, our microprocessor is *very* fast by mechanical
standards: each one of these bounces makes a new interrupt.

To "debounce" the switch, you need to *not* process button press interrupts for a short time after
you receive one. 50-100ms is typically a good debounce interval. Debounce timing seems hard: you
definitely don't want to spin in an interrupt handler, and yet it would be hard to deal with this in
the main program.

The solution comes through another form of hardware concurrency: the `TIMER` peripheral we have used
a bunch already. You can set the timer when a "good" button interrupt is received, and not respond
to further interrupts until the timer peripheral has counted enough time off. The timers in
`nrf-hal` come configured with a 32-bit count value and a "tick rate" of 1 MHz: a million ticks per
second. For a 100ms debounce, just let the timer count off 100,000 ticks. Anytime the button
interrupt handler sees that the timer is running, it can just do nothing.

The implementation of all this can be seen in the next example (`examples/count-debounce.rs`). When
you run the example you should see one count per button press.

> **NOTE** The buttons on the MB2 are a little fiddly: it's pretty easy to push one down enough to
feel a "click" but not enough to actually make contact with the switch. I recommend using a
fingernail to press the button when testing.
