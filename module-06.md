# Module 10: Concurrence

## Forword

```txt
error: pineapple doesn't go on pizza
 --> main.rs:6:18
  |
 6|     let _: Pizza<Pineapple>;
  |            ----- ^^^^^^^^^
  |            |
  |            this is the pizza you ruined
  |
  = note: `#[forbid(bad_taste)]` on by default
  = note: you're a monster
```

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

## Exercise 00: Cellular

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cell::Cell
```

Write a **function** with the following prototype:

```rust
fn swap_u32(a: &Cell<u32>, b: &Cell<u32>);
```

* The `swap_u32` function must swap the integers it is given.

Example:

```rust
let a = Cell::new(1);
let b = Cell::new(3);

swap_u32(&a, &b);

assert_eq!(a.get(), 3);
assert_eq!(b.get(), 1);
```

Let's complicate things a bit!

```rust
fn swap_string(a: &Cell<String>, b: &Cell<String>);
```

* The `swap_string` function must swap the strings it is given.

Example:

```rust
let a = Cell::new("ABC".into());
let b = Cell::new("DEF".into());

swap_string(&a, &b);

assert_eq!(a.into_inner(), "ABC");
assert_eq!(b.into_inner(), "ABC");
```

## Exercise 01: Counting

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::thread::{spawn, sleep, JoinHandle}
    std::time::Duration
```

Create a **program** that does the following:

* Start a thread.
* Within the thread, print numbers from 1 to 10.
* Wait a quarter of a second between each number.

Example:

```txt
>_ cargo run
1
2
3
4
5
6
7
8
9
10
```

## Exercise 02: Logger

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::sync::{Arc, Mutex}
    std::thread::spawn
    std::io::Write
```

Create a `Logger` type.

```rust
struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self;
}
```

 * `new` must create a new `Logger` with a buffer of size `threshold` and the given `W` instance.

In order to avoid perform too many `write` system calls, you want to first write stuff to an
internal `buffer`, and THEN, write every thing to a file.

```rust
impl<W: io::Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()>;
    pub fn flush(&mut self);
}
```

 * `log` must try to write `message` to its internal buffer. When the buffer is full, everything
   must be sent to the specified `io::Write` implementation. After thatm the buffer is cleared for
   new data to be added. A `\n` is automatically added at the end of the message.
 * `flush` must properly send the content of the buffer inconditionally and clears it.

Create a `main` function spawning 10 threads. Each thread must try to write to the standard output
using the same `Logger<Stdout>`.

```txt
>_ cargo run
hello from thread 1!
hello from thread 2!
hello from thread 0!
hello from thread 6!
...
```

## Exercise 03: A Philosopher's Tiny Brain

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::sync::Arc
    std::sync::mpsc::{sync_channel, SyncSender, Receiver}
    std::thread::{spawn, sleep}
    std::time::Duration
```

Create a **program** that works in the following way:

```txt
>_ cargo run -- 3
cakes
the philosopher is thinking about cakes
code
42
the philosopher is thinking about code
a
b
c
the philosopher's head is full
the philosopher is thinking about 42
the philosopher is thinking about a
the philosopher is thinking about b
^C
>_
```

* The program must ask the user to enter words in the standard input.
* Each time a word is entered, it is saved in the philosopher's brain.
* If the brain is full, an error is displayed and the word is not added to the brain.
* When a word is available in the brain, the philosopher thinks about it for 5 seconds.
* The program never ends.
* The size of the philosopher's brain is provided in command-line arguments.

## Exercise 04: Atomical

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::sync::atomic::{AtomicU8, Ordering}
```

Create a type named `Unique`.

```rust
#[derive(Debug, PartialEq, Eq)]
struct Unique(u8);

impl Unique {
    pub fn new() -> Self;
}
```

* There can be no two `Unique` instance with the same identifier (`u32`).
* `new` must create a new, unique instance of `Unique`.
* It must be possible to `Clone` a `Unique`, and the created `Unique` must still be unique.
* Trying to create a `Unique` when no more IDs are available causes the function to panic.

Example:

```rust
fn main()
    let a = Unique::new();
    let b = Unique::new();
    let c = Unique::new();

    assert_eq!("{a:?}");
    assert_eq!("{b:?}");
    assert_eq!("{c:?}");

    let d = a.clone();
    let e = c.clone();

    assert_eq!("{d:?}");
    assert_eq!("{e:?}");
}
```

Would produce the following output:

```txt
>_ cargo run
Unique(0)
Unique(1)
Unique(2)
Unique(3)
Unique(4)
```

What atomic memory ordering did you use? Why?

## Exercise 05: PI * Rayon * Rayon

```txt
turn-in directory:
    ex05/

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

## Exercise 06: Scoped Threads

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::thread::{scope, ScopedJoinHandle}
```

TODO:

## Exercise 07:

TODO: Create a multi-threaded webserver (with a thread pool).
