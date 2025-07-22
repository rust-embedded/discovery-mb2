# Game logic

The first module we will build is the game logic. You are probably familiar with [snake] games, but
if not, the basic idea is that the player guides a snake around a 2D grid. At any given time, there
is some "food" at a random location on the grid and the goal of the game is to get the snake to
"eat" as much food as possible. Each time the snake eats food it grows in length. The player loses
if the snake crashes into its own tail.

[snake]: https://en.wikipedia.org/wiki/Snake_%28video_game_genre%29

In some variants of the game, the player also loses if the snake crashes into the edge of the grid,
but given the small size of our grid we are going to implement a "wraparound" rule: if the snake
goes off one edge of the grid, it will continue from the opposite edge.

## The `game` module

We will build up the game mechanics in the `game` module.

### Coordinates

We start by defining a coordinate system for our game (`src/game/coords.rs`).

```rust
{{#include src/game/coords.rs}}
```

We use a `Coords` struct to refer to a position on the grid. Because `Coords` only contains two
integers, we tell the compiler to derive an implementation of the `Copy` trait for it, so we can
pass around `Coords` structs without having to worry about ownership.

### Random Number Generation

We define an associated function, `Coords::random`, which will give us a random position on the
grid. We will use this later to determine where to place the snake's food.

To generate random coordinates, we need a source of random numbers. The nRF52833 has a hardware
random number generator (HWRNG) peripheral, documented at section 6.19 of the [nRF52833 spec]. The
HAL gives us a simple interface to the HWRNG via the `microbit::hal::rng::Rng` struct. The HWRNG may
not be fast enough for a game; it is also convenient for testing to be able to replicate the
sequence of random numbers produced by the generator between runs, which is impossible for the HWRNG
by design. We thus also define a [pseudo-random] number generator (PRNG). The PRNG uses an
[xorshift] algorithm to generate pseudo-random `u32` values. The algorithm is basic and not
cryptographically secure, but it is efficient, easy to implement and good enough for our humble
snake game. Our `Prng` struct requires an initial seed value, which we do get from the RNG
peripheral.

[nRF52833 spec]: https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf
[pseudo-random]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator
[xorshift]: https://en.wikipedia.org/wiki/Xorshift

All of this makes up `src/game/rng.rs`.

```rust
{{#include src/game/rng.rs}}
```

### Movement

We also need to define a few `enum`s that help us manage the game's state: direction of movement,
direction to turn, the current game status and the outcome of a particular "step" in the game (ie, a
single movement of the snake). `src/game/movement.rs` contains these.

```rust
{{#include src/game/movement.rs}}
```

### A Snake (*A Snaaake!*)

Next up we define a `Snake` struct, which keeps track of the coordinates occupied by the snake and
its direction of travel. We use a queue (`heapless::spsc::Queue`) to keep track of the order of
coordinates and a hash set (`heapless::FnvIndexSet`) to allow for quick collision detection.  The
`Snake` has methods to allow it to move. `src/game/snake.rs` gets this.

```rust
{{#include src/game/snake.rs}}
```

### Game Module Top-Level

The `Game` struct keeps track of the game state. It holds a `Snake` object, the current coordinates
of the food, the speed of the game (which is used to determine the time that elapses between each
movement of the snake), the status of the game (whether the game is ongoing or the player has won or
lost) and the player's score.

This struct contains methods to handle each step of the game, determining the snake's next move and
updating the game state accordingly. It also contains two methods--`game_matrix` and
`score_matrix`--that output 2D arrays of values which can be used to display the game state or the
player score on the LED matrix (as we will see later).

We put the `Game` struct at the top of the `game` module, in `src/game.rs`.

```rust
{{#include src/game.rs}}
```

Next we will add the ability to control the snake's movements.
