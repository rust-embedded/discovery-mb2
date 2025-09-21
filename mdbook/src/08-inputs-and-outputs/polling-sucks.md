# Polling sucks, actually

Oh yeah, turn signals usually blink, right?  How could we extend our program to blink the turn signal LED when a button is pressed.  We know how to blink an LED from our Hello World program; we turn on the LED, wait for some time, and then turn it off.  But how can we do this in our main loop while also checking for button presses?  We could try something like this:

```rust
    loop {
        if button_a.is_low().unwrap() {
            // Blink left arrow
            display.show(&LEFT_ARROW);
            timer.delay_ms(500_u32);
            display.show(&BLANK);
            timer.delay_ms(500_u32);
        } else if button_b.is_low().unwrap() {
            // Blink right arrow
            display.show(&RIGHT_ARROW);
            timer.delay_ms(500_u32);
            display.show(&BLANK);
            timer.delay_ms(500_u32);
        } else {
            display.show(&BLANK);
        }
        timer.delay_ms(10_u32);
    }
```

Can you see the problem?  We're trying to do two things at once here: 

1. Check for button presses
2. Blink the LED

But the processor can only do one thing at a time.  If we press a button during the blink delay, the processor won't be able to respond until the delay is over and the loop starts again.  As a result, we get a barely-responsive program (try for yourself and see how slow the button is).

A "smarter" program would know that the processor isn't actually doing anything while the blink delay is running. The program could very well do other things while waiting for the delay to finish — namely, checking for button presses.

## Superloops

The term *superloop* in embedded systems is used to refer to a main control loop that does a bunch of things in sequence.  It's the natural extension of the simple control flow we've been using so far.  To handle logic that could be perceived as multiple things happening at once, we need to be a bit more clever in how we structure the program so that we can be reasonably responsive to events.

In the case of our turn signal program, where we want to blink the LEDs when a button is pressed, and be quick to stop blinking when the button is released, we can create a "state machine" to represent the various states of the program.  We have three states for the buttons:

1. No button is pressed
2. Button A is pressed
3. Button B is pressed

We also have three states for the display:

1. No LEDs are on
2. We are in the active blink state for the display (the LEDs are on)
3. We are in the inactive blink state for the display (the LEDs are off and waiting to be turned on once the blinking period is over)

Since we need to ensure responsiveness, we have to combine these different states.  To fully represent all states of our program, we would have the following:

1. No button is pressed
2. Button A is pressed, and we are in the active blink state (the left arrow is showing on the display)
3. Button A is pressed, and we are in the inactive blink state (nothing is showing on the display)
4. Button B is pressed, and we are in the active blink state (the right arrow is showing on the display)
5. Button B is pressed, and we are in the inactive blink state (nothing is showing on the display)

When either button is first pressed, and we transition from state (1) to either state (2) or (4), we will initialize a timer counter that counts up starting from the moment a button is pressed.  When the timer reaches some threshold amount (like half a second) and the buttons are still pressed, we will then transition to state (3) or (5), respectively, and reinitialize the timer counter.  When the timer again reaches some threshold amount, we will transition back to state (2) or (4), respectively.  If at any time during states (2), (3), (4), or (5) we see that the button is no longer pressed, we transition back to state (1).

Our main superloop control flow will repeatedly poll the buttons, compare our current timer counter (if we have one) to a threshold, and change states if any of the above conditions are met.

We have implemented this superloop as a demonstration (`examples/blink-held.rs`), but with the state machine simplified only to blink an LED when button A is held.

```rust
{{#include examples/blink-held.rs}}
```

This is still a bit complex. The 10ms loop delay is more
than adequate to catch button changes.

Superloops work and are often used in embedded systems, but the programmer has to be careful to maintain a high degree of responsiveness to events.  Note how our superloop program is different from the previous simple polling example.  Any state transition step in the superloop as written above should take a fairly small amount of time (e.g. we no longer have delays that could block the processor for long periods of time and cause us to miss any events).  It's not always easy to transform a simple polling program into a superloop where all state transitions are quick and relatively non-blocking, and in these cases, we will have the rely on alternative techniques for handling the different events being executed at the same time.

## Concurrency

Doing multiple things at once is called *concurrent* programming. Concurrency shows up in many places in programming, but especially in embedded systems.  There's a whole host of techniques for implementing systems that interact with peripherals while maintaining a high degree of responsiveness (e.g. interrupt handling, cooperative multitasking, event queues, etc.).  We'll explore some of these in later chapters.

There is a good introduction to concurrency in an embedded context [here] that
you might read through before proceeding.

[here]: https://docs.rust-embedded.org/book/concurrency/index.html


For now, let's take a deeper look into what's happening when we call `button_a.is_low()` or `display_pins.row1.set_high()`.
