# Meet your software

In this chapter we will learn how to build, run and debug some *very* simple programs. The goal here
is not to get into the details of MB2 Rust programming (yet), but to just familiarize yourself with
the mechanics of the process.

First, a quick note about the conventions used in the rest of this book. We expect you to get
a copy of the whole book with

```
git clone http://github.com/rust-embedded/discovery-mb2
```

The book's "source code" is in `discovery-mb2/mdbook/src`. You should go there in your copy and look
around a bit. Each chapter directory has both the source Markdown text *and* the complete source for
all the programs in that chapter. When we refer to some path like `src/main.rs`, we mean that place
starting from the chapter you are working in. For example, your `discovery-mb2` has a file called
`mdbook/src/05-meet-your-software/examples/init.rs`. We will refer to that file as just
`examples/init.rs` in this chapter.

There are two basic kinds of Rust code: "binary" executable programs, and "library" code. The
library code won't play a huge role in this book. Binary program source code can live in one of
several places:

* A program in `src/main.rs` will be automatically compiled and run by `cargo embed` or `cargo
  run`. No special flags are needed.

* A program in `examples/foo.rs` can be compiled and run by `cargo embed --example foo` or
  `cargo run --example foo`.
  
* A program in `src/bin/bar.rs` can be compiled and run by `cargo embed --bin bar` or
  `cargo run --bin bar`.

This is confusing, but it's a standard convention of Cargo.

Now let's move on and work with all this.
