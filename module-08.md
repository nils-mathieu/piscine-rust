# Module 08: Functions Are Values

## Introduction

Rust provides multiple kinds of functions. Well, in the end, a function is a function. But Rust
allows you to do some cool things with functions, bringing it a bit closer to purely functional
languages.

Specifically, in Rust, functions are just like any regular values. You can pass them to other
functions to call them later, you can return them from other functions. A function that does that
is said to be of "higher order".

But in the end, a function is just a bunch of machine instructions that our beloved CPU can jump to
in order to do things. This module will teach you how Rust uses this fairly simple concept to turn
it into something very powerful (and pretty complicated).

## General Rules

Any exercise you turn in should compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise must be part of a virtual Cargo workspace, a single
`workspace.members` table must be declared for the whole module.

Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options. You are allowed to use attributes to modify lint levels, but
you must be able to explain why you did so. You are *not* allowed to use `unsafe` code anywere in
your code.

## Exercise 00: Function Pointers (`call_twice` v1)

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::println
```

Create a **function** that takes another function as its input and calls it twice.

```rust
fn call_twice(f: fn());
```

Write a **main** function that proves the passed function is indeed called twice.

## Exercise 01: Associated Functions (`call_twice` v2)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

Create a trait named `Action` with a single associated method.

```rust
fn perform_action(&mut self);
```

Create a type to implement your trait, and re-write the `call_twice` method to make use of your
trait.

```rust
fn call_twice<A: Action>(action: A);
```

Write tests to show that the `perform_action` method has been properly called twice.

## Exercise 02: Closures (`call_twice` v3)

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

```rust
fn call_twice<F: FnMut()>(f: F);
```

Notice how the type parameter implements the `FnMut()` trait. How would using `Fn()` or `FnOnce()`
change the semantics of the function?

You must provide tests to show that the function works.

## Exercise 03: Gotta Catch 'em All

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml
```

As you should have noticed in the previous exercise, closures are capable of capturing data present
in their scope.

```rust
#[derive(Clone)]
struct Pikachu;

fn one<F: FnOnce() -> Pikachu>(f: F) {
    let _ = f();
}

fn two<F: FnMut() -> Pikachu>(mut f: F) {
    let _ = f();
    let _ = f();
}

fn three<F: Fn() -> Pikachu>(f: F) {
    let _ = f();
    let _ = f();
    let _ = f();
}

fn main() {
    let pika1 = Pikachu;
    let pika2 = Pikachu;
    let pika3 = [Pikachu, Pikachu, Pikachu].into_iter();

    /* ??? */ (move || pika1.clone());
    /* ??? */ (move || pika2);
    /* ??? */ (move || pika3.next().unwrap());
}
```

Copy the above file into your `main.rs` file and replace the `/* ??? */` by any of the three
functions above. You are only allowed to use each function once.

How did you choose? What does `move` means? Would those calls work without it? Why? You will be
asked during defense.

## Exercise 04: Once No More

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::ops::{FnMut, FnOnce}
```

Create a function that turns a `FnOnce` function into an `Fn` function.

```rust
fn once_no_more(f: impl Clone + FnOnce()) -> impl Fn();
```

Example:

```rust
let f2 = once_no_more(|| ());
f2();
f2();
f2();
```

You must write tests!

## Exercise 05: GREP

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::ops::Fn  str::{len, split_at}  std::env::args
```

Let's create a Generic Regular Expression Parser. A regular expression is basically a function that
takes a string, and returns how many characters matched the string.

In other words, a regular expression is basically anythuing that matches this signature:

```rust
impl Fn(&str) -> bool
```

Let's start small. The most basic regular expression simply matches anything.

```rust
fn match_anything() -> impl Fn(&str) -> bool;
```

The `match_anything` function returns a function that, when given a string, always returns `true`.

Another basic regular expression is simply a string to match exactly.

```rust
fn match_exactly(to_match: &str) -> impl Fn(&str) -> bool;
```

The `match_exactly` function must return a function that returns whether it was given `to_match`.

Let's complicate things a bit: let's create a combinantor.

```rust
fn match_any(
    first: impl Fn(&str) -> bool,
    second: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool;
```

The `match_any` returns a function that, when given a string, returns whether that string matches
either the `first` or `second` regular expression.

Finally, let's create another combinator to chain two regular expressions together.

```rust
fn match_chain(
    first: impl Fn(&str) -> bool,
    second: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool;
```

The `match_chain` function returns a function that, when given a string, returns whether it can be
split in two, such that the first part matches with `first` and the second part matches with
`second`.

With those tools, create a **program** that takes a single parameter as input (the program can
panic in case an invalid input is given) and verifies whether it is a valid e-mail address of the
following form:

```txt
*@*[.com,.fr]
```

In other words: anything, followed by an `@`, followed by anything, followed by either `.com` or
`.fr`.

Example:

```txt
>_ cargo run -- ""
no
>_ cargo run -- a
no
>_ cargo run -- @.
no
>_ cargo run -- @.fr
yes
>_ cargo run -- test@example.com
yes
>_ cargo run -- @com.com
yes
>_ cargo run -- a@.fr
yes
>_ cargo run -- a.fr
no
```
