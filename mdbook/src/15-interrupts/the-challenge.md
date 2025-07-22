# The Challenge

Let's make the MB2 into a siren! But not just any siren — an
interrupt-driven siren. That way we can turn the siren on
and the rest of our program can run on, ignoring it.

Make your siren sweep the pitch from 220Hz to 440Hz and back
over a one-second period. The main program should start the
siren, then print a ten-second countdown from 10 to 1, then
stop the siren and print "launch!". The main program should
not mess with the siren during countdown — it should just be
interrupt-driven.

*Hint:* I found it easiest to use a global locked `Siren`
struct that owned the state of the siren and the peripherals
it needed to operate.

This is a fancy program that introduces a lot of new
ideas. Don't be surprised if it takes you a bit to figure it
out.
