# Module 10: Concurrence

## Forword

TODO:

## General Rules

* Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed.

* Every exercise must be part of a virtual Cargo workspace, a single `workspace.members` table must
be declared for the whole module.

* Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options.  You are *not* allowed to use `unsafe` code anywere in your
code.

* You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You may optionally allow the `dead_code`
lint to silence warnings about unused variables, functions, etc.

* You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

## Exercise 00:

TODO: Swap using cell and shared references.

## Exercise 01:

TODO: Spawn a thread (maybe see scopes). Send, Sync

## Exercise 02: Mutual Exclusion

TODO: Mutex and how that relates to cell.

## Exercise 03:

TODO: Barber Problem

## Exercise 04: PI * Rayon * Rayon

```txt
turn-in directory:
    ex04/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    rayon

allowed symbols:
    std::iter::* rayon::prelude::*
    std::println  std::env::args
```

To finish with this module, let's look at a popular third-party crate!

First, let's create a single threaded **program** that uses [monte carlo's method](https://en.wikipedia.org/wiki/Monte_Carlo_method#Overview)
to compute PI. The program takes a single argument: the number of sampled points.

Try to write this algorithm without a `for` loop. Instead, rely on chained iterators. This will
make it easier for you in the second part of the exercise.

```txt
>_ RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo run -- 1000000
pi: 3.1413
duration: 722ms
```

Even for as little as a million points, the aglorithm is already pretty slow. Try to speed it up a
little using the [`rayon`](https://crates.io/crates/rayon) crate.

```txt
>_ RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo run -- 1000000
pi: 3.144044
duration: 147ms
```

## Exercise 05:

TODO: 

## Exercise 06:

TODO:

## Exercise 07:

TODO: Create a multi-threaded webserver (with a thread pool).
