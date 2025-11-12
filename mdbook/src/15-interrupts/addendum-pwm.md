# Addendum: PWM

One last note before we move on.

Interrupts are kind of expensive. The processor must finish
or abort the currently-running instruction, then save enough
state to restart execution, then call an interrupt
handler. All this takes a few CPU cycles of precious
runtime.

The way the solution of the previous section is written, it
will take two interrupts per cycle of speaker output. That's
something like 1000 interrupts per second. On a processor
like our nRF52833, that works fine.

The nRF52833 does have an on-board peripheral that could cut
our siren's interrupt rate way down. The Pulse-Width
Modulation (PWM) unit can, among other things, generate
cycles on the speaker pin at a rate controlled by a PWM
register. This could be used to generate the basic square
wave used for our siren. We would still need an interrupt
every time we wanted to change the frequency, but this might
be more like 10 interrupts per second than 1000.

I did not use the PWM unit in my solution. This was partly
because I wanted to focus on interrupts. Another big reason,
though, was that the nRF52833 PWM unit is pretty complicated
and hard to understand. Getting something working a simple
way in the tight bare-metal environment is always
attractive.

If you are up for a challenge, I would encourage you to try
using the PWM unit for your siren.
