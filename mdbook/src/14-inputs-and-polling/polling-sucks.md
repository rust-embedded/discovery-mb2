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

But the processor can only do one thing at a time.  If we press a button during the blink delay, the processor won't be able to respond until the delay is over and the loop starts again.  As a result, we get a much less responsive program (try for yourself and see how worse it is).

A "smarter" program would know that the processor isn't actually doing anything while the blink delay is running, so it could very well do other things while waiting for the delay to finish, namely checking for button presses.  

Doing multiple things at once is called *concurrent* programming, and shows up in many places in programming, but especially in embedded systems.  There's a whole host of techniques for implementing systems that concurrently interact with peripherals while maintaining a high degree of responsiveness (e.g. interrupt handling, cooperative multitasking, event-driven super-loops, etc.).  We'll explore some of these in later chapters.

In the next chapter, we'll look at a technique called *interrupts* that is better suited to doing multiple things at once.