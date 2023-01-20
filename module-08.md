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

Any program you turn in should compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise that uses the `cargo` package manager must be part of a single
virtual Cargo workspace, a single `workspace.members` table must be declared for the whole module.

Any program you turn in should compile *without warnings* using the `rustc` compiler available on
the school's machines without additional options. You are allowed to use attributes to modify lint
levels, but you must be able to explain why you did so. You are *not* allowed to use `unsafe` code
anywere in your code.

## Exercise 00: Function Pointers (`call_twice` v1)

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml
```

Create a **function** that takes another function as its input and calls it twice.

```Rust
fn call_twice(f: fn());
```

Write a **main** function that proves the passed function is indeed called twice.

## Exercise 01: Generic Function Pointers (`call_twice` v2)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Function pointers are cool, but they are pretty limited. Specifically, it is not possible to make
them to reference any data.

One way to work around this problem - and this is the only way to do it in C - is to pass a custom
parameter along with the function pointer.

```Rust
fn call_twice<T>(data: &mut T, f: fn(&mut T));
```

This time, the `call_twice` function must pass the `data` parameter to the function pointer when
calling it. This allow the passed function to access some custom state.

You must write test for this new functions.

## Exercise 02: Closures (`call_twice` v3)

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

The pattern presented in the previous exercise is so common, in fact, that the Rust language
provides a way to do it more easily: [closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html).

```Rust
fn call_twice<F: FnMut()>(f: F);
```

Notice how the type parameter implements the [`FnMut()`](https://doc.rust-lang.org/std/ops/trait.FnMut.html)
trait. This indicates that the "data" parameter is taken by mutable reference (i.e. `&mut Data`).
How would using [`Fn()`](https://doc.rust-lang.org/std/ops/trait.Fn.html) or [`FnOnce()`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)
change the semantics of the function?

You must provide tests to show that the function works.

## Exercise 03: Gotta Catch 'em All

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

As you should have noticed in the previous exercise, closures are capable of capturing data present
in their scope.

```Rust
#[derive(Clone)]
struct Pikachu;

fn one(f: impl FnOnce() -> Pikachu) {
    let _ = f();
}

fn two(mut f: impl FnMut() -> Pikachu) {
    let _ = f();
    let _ = f();
}

fn three(f: impl Fn() -> Pikachu) {
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
