# Snake game

We're now going to implement a basic [snake](https://en.wikipedia.org/wiki/Snake_(video_game_genre))
game that you can play on an MB2 using its 5×5 LED matrix as a display and its two buttons as
controls. In doing so, we will build on some of the concepts covered in the earlier chapters of this
book, and also learn about some new peripherals and concepts.

## Modularity

The source code here is more modular than it probably should be. This fine-grained modularity allows
us to look at the source code a little at a time. We will build the code bottom-up: we will first
build three modules — `game`, `controls` and `display`, and then compose these to build the final
program. Each module will have a top-level source file and one or more included source files: for
example, the `game` module will consist of `src/game.rs`, `src/game/coords.rs`,
`src/game/movement.rs`, etc. The Rust `mod` statement is used to combine the various components of
the module. *The Rust Programming Language* has a good [description] of Rust's module system.

[description]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
