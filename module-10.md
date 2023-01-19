# Module 10: Threads And Concurency

## Introduction

Nowdays, CPUs tend to have more than one core. And even without that, modern operating systems provide ways to execute multiple programs at the some time. Those programs may even share the same memory! In that case, we call those sub-programs "threads".

A thread is just another executing context for an already existing process. Rust provides ways to start threads, ways to manipualte them, and ways to make them communicate safely.

## General Rules

Any program you turn in should compile using the `cargo` package manager, either with `cargo run` if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified in the `allowed dependencies` section are allowed.

Any program you turn in should compile *without warnings* using the `rustc` compiler available on the school's machines without additional options. You are allowed to use attributes to modify lint levels, but you must be able to explain why you did so. You are *not* allowed to use `unsafe` code anywere in your code.

## Exercise 00: Follow The Threads

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit
```

Create a **program** that starts 5 threads. Each thread must wait half a second, generate a random number between 0 and 9, print it, and then return that number.

The program must wait until all threads have successfully returned before summing the numbers and printing the result.

Example:

```txt
>_ cargo run
thread 2 has generated: 4
thread 5 has generated: 9
thread 1 has generated: 8
thread 4 has generated: 5
thread 3 has generated: 3
total: 29
```

## Exercise 01: Scope The Threads

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit
```

Create a **program** that takes a single string as an argument.

Spawn 5 threads. Each thread must randomly choose a substring of the argument, print it, then return it as a string reference (i.e. `&str`).

As for the last exercise, the program must wait until all threads have successfully completed. The selected substrings can then be concatenated and written to the standard output.

Example:

```txt
>_ cargo run -- "hello, world!"
thread 1 has choosen: "hel"
thread 4 has choosen: "hello, w"
thread 3 has choosen: "o, w"
thread 5 has choosen: "rld"
thread 2 has choosen: " world!"
"hel world!o, whello, wrld"
```

The program may panic when the user uses it incorrectly.

## Exercise 02: Mutual Exclusion

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

If a Rust program compiles, then it is *guarenteed* not to exhibit any memory unsafety (as long as `unsafe` code is not involved, of course, but that's a topic for later).

This memory unsafety can be a bit hard to uphold when multiple threads are executing and operating on the same data at the same time. Reading data at the same time is always fine (this is what the previous exercise what about). But writing... Oh boy, this is a whole computer science subject.

A way to safely access a given value while modifying it at multiple places simulatenously is to use a mechanism called "mutual exclusion". Mutex, for short. The idea is pretty simple: if you want to modify the protected value, you first check if someone is already accessing it. If so, you wait.

Create a **program** that starts 10 threads. Each thread is assigned a number. As soon as they start, each thread must append their number to a list (you can use a `Vec<T>` for that). When every thread has completed their task, the program must print the generated list.

Example:

```txt
>_ cargo run
[1, 2, 4, 0, 3, 5, 6, 7, 8, 9]
```

Rust provides other constructs which may suite your needs better, such as [`RwLock`](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html), [atomics](https://doc.rust-lang.org/stable/std/sync/atomic/index.html) or [channels](https://doc.rust-lang.org/stable/std/sync/mpsc/index.html), but those are outside the scope of this piscine.

## Exercise 03: `Arc` And Arrows

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit
```

[`Arc<T>`] stands for Atomically Reference Counted. This type manages an allocation that remains live until all [`Arc<T>`]s that reference it are dropped.

Without using [scoped threads](https://doc.rust-lang.org/std/thread/fn.scope.html), you must write a **program** that does the following:

1. First a thread is spawned. The thread increments a number by one.
2. After that, the thread has 1/2 chance to create two new threads doing the exact same thing.
3. Once *every* thread has returned, the final number is written to the standard output.

Example:

```txt
>_ cargo run
1 threads
>_ cargo run
121 threads
>_ cargo run
17 threads
>_ cargo run
21 threads
```

[`Arc<T>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html

## Exercise 04: Errno

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

The `errno` global variable available in `#include <errno.h>` is "thread-local; setting it in one thread does not affect its value in any other thread" - `man errno`.

Let's create our own `errno`! Rust has platform-independent mechanisms to create [thread-local variables](https://doc.rust-lang.org/stable/std/macro.thread_local.html).

First, create an `enum` named `Error`. This type can have the variants of your choice.

Example:

```Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Error {
    Success,
    AmbiguousRedirect,
    CommandNotFound,
    IsADirectory,
    FileNotFound,
    FileAlreadyExists,
    Interrupted,
}
```

Then, implement the following functions.

```Rust
impl Error {
    fn last_error() -> Self;
    fn make_last_error(self);
}
```

The `Error::make_last_error` function must set the calling thread's last `Error` instance. The `Error::last_error` function must return the calling thread's last `Error` instance.

Write tests to verify that the two function are indeed thread-local.

## Exercise 05: SOMEBODY TOUCHA MY SPAGHET??

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit
```

Philosophers are hungry creatures. In their life, there is only two things: spaghetti and sleep. Each philosopher has a specific number of spaghetti, patiently waiting for them in the fridge. They all share the same fridge.

The life a philosopher goes like this:

- First, the philosopher is hungry. They open the fridge. Only one philosopher can open the fridge at any given time.
- They look for their pack of spaghetti, and eat one.
- Once they finished eating, they close the fridge and go back to sleep for a some amount of time, letting other philosophers access the fridge.

There's a catch, though: if the spaghetti pack of a philosopher is empty, they will look for another's. When that other philosopher realises that one of their spaghetti is missing, they will `panic!` with the message `"SOMEBODY TOUCHA MY SPAGHET??"`.

The **program** you must write will simulate the life of some philosophers. Each argument passed to the program corresponds to the size of the spaghetti pack of those philosophers. When there is no spaghetti left in the fridge, the remaining philosophers cleanly return and the main thread displays which philosophers were alive by the end of the simulation.

```txt
>_ cargo run -- 5 5 1 1
1 has eaten the spaghetti of 1
0 has eaten the spaghetti of 0
3 has eaten the spaghetti of 3
2 has eaten the spaghetti of 2
2 has eaten the spaghetti of 0
1 has eaten the spaghetti of 1
3 has eaten the spaghetti of 0
thread '<unnamed>' panicked at 'SOMEBODY TOUCHA MY SPAGHET??', src/main.rs:24:29
thread '<unnamed>' panicked at 'SOMEBODY TOUCHA MY SPAGHET??', src/main.rs:24:29
1 has eaten the spaghetti of 1
3 has eaten the spaghetti of 0
1 has eaten the spaghetti of 1
1 has eaten the spaghetti of 1
3 has eaten the spaghetti of 0
still alive: [1, 3]
```

You can modify the [panic hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html) and/or the [thread's name](https://doc.rust-lang.org/std/thread/struct.Builder.html) to make panic messages display the name of the philosopher instead.

## Exercise 06: PI * Rayon * Rayon

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit  rayon
```

To finish with this module, let's look at a popular third-party crate!

First, let's create a single threaded **program** that uses [monte carlo's method](https://en.wikipedia.org/wiki/Monte_Carlo_method#Overview) to compute PI. The program takes a single argument: the number of sampled points.

Try to write this algorithm without a `for` loop. Instead, rely on chained iterators. This will make it easier for you in the second part of the exercise.

```txt
>_ RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo run -- 1000000
pi: 3.1413
duration: 722ms
```

Even for as little as a million points, the aglorithm is already pretty slow. Try to speed it up a little using the [`rayon`](https://crates.io/crates/rayon) crate.

```txt
>_ RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo run -- 1000000
pi: 3.144044
duration: 147ms
```
