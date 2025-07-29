# Snake game: final assembly

The code in our `src/main.rs` file brings all the previously-discussed machinery together to make
our final game.

```rust
{{#include src/main.rs}}
```

After initializing the board and its timer and RNG peripherals, we initialize a `Game` struct and a
`Display` from the `microbit::display::blocking` module.

In our "game loop" (which runs inside of the "main loop" we place in our `main` function), we
repeatedly perform the following steps:

1. Get a 5×5 array of bytes representing the grid. The `Game::get_matrix` method takes three integer
   arguments (which should be between 0 and 9, inclusive) which will, eventually, represent how
   brightly the head, tail and food should be displayed.

2. Display the matrix, for an amount of time determined by the `Game::step_len_ms` method. As
   currently implemented, this method basically provides for 1 second between steps, reducing by
   200ms every time the player scores 5 points (eating 1 piece of food = 1 point), subject to a
   floor of 200ms.

3. Check the game status. If it is `Ongoing` (which is its initial value), run a step of the game
   and update the game state (including its `status` property). Otherwise, the game is over, so
   flash the current image three times, then show the player's score (represented as a number of
   illuminated LEDs corresponding to the score), and exit the game loop.

Our main loop just runs the game loop repeatedly, resetting the game's state after each iteration.
