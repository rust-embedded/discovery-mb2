# Polling

Now that we've learned how to read GPIO inputs, let's consider how we might use these reads practically. Suppose we want our program to turn on an LED when Button A is pressed and turn it off when Button B is pressed.  We can do this by polling the state of both buttons in a loop, and responding accordingly when a button is read to be pressed.  Here's how we might write this program:

```rust
{{#include examples/polling-led-toggle.rs}}
```

This method of repeatedly checking inputs in a loop is called polling.  When we check the state of some input, we say we are *polling* that input.  In this case, we are polling both Button A and Button B.

> **Note** The processor can perform this loop much faster than we need it to for our purposes, so we add a small delay that doesn't meaningfully impact the responsiveness of the program, but allows the MCU to take a breath every once in a while.  

Polling is simple but allows us to do interesting things based on the external world.  For all of our device's inputs, we can "poll" them in a loop, and respond to the results in some way, one by one.  This kind of method is very conceptually simple and is a good starting point for many projects.  We'll soon find out why polling might not be the best method for all (or even most) cases, but let's try it out first.