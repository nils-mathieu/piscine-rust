# Module 05: Generics

## Introduction

If the C programming language had only one flaw, it would be its poorly reusable code. This
language makes it very difficult to write code that may be used in multiple similar situations,
especially when the only thing that changes is a bunch of types. It wasn't an issue at first
because everything was basically an `int`. Things have changed a bit.

Dynamic dispatch can be very useful to avoid repeating yourself. In fact, this is what
inheritance-driven languages usually use (think Java, C# or C++ without templates).

In Rust, we tend to use static dispatch by default. The idea is pretty simple: re-compile the
function for every possible type that needs it. This allows more optimized code, but longer compile
times and potentially larger binary sizes. In practice, the binary size can be easily managed. As
for compilation times, this is still [a work in progress issue](https://perf.rust-lang.org/).

## General Rules

Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise must be part of a virtual Cargo workspace, a single
`workspace.members` table must be declared for the whole module.

Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options. You are *not* allowed to use `unsafe` code anywere in your
code.

You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You must use the `#![forbid(unsafe_code)]`
attribute in every project you turn in. You may optionally allow the `dead_code` lint to silence
warnings about unused variables, functions, etc.

You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

## Exercise 00: `choose`

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    <[T]>::len  std::println  ftkit::random_number
```

Create a **function** that randomly chooses a value among an input slice. If the provided list is
empty, the function is allowed to panic.

```rust
fn choose<T>(values: &[T]) -> &T;
```

You can write a `main` function to show that the function works as expected.

## Exercise 01: Point Of No Return (v3)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cmp::PartialOrd  std::{assert*}
    std::marker::Sized
```

Again? Yes. Another `min` function! But I promise, this one's the last one.

* Create a `min` function that takes *any* two values of a type that supports the `<` operator, and
returns the smaller one.

Example:

```rust
assert_eq!(min(12i32, -14i32), -14);
assert_eq!(min(12u32, 14u32), 12);
assert_eq!(min("abc", "def"), "abc");
assert_eq!(min(String::from("abc"), String::from("def")), "abc");
```

Still not allowed to use `return`!

## Exercise 02: Grettings

```txt
turn-in directory:
    ex03/

file to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::fmt::{Formatter, Display}
```

TODO: impl Display greet in function

## Exercise 03: A Generic Vector

```txt
turn-in directory:
    ex03/

file to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::fmt::Debug  std::cmp::{Eq, PartialEq} std::marker::Copy
    std::{assert*}  std::clone::Clone
```

Create a `Vector` type.

```rust
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self;
}
```

* The `new` function must create a new instance of `Vector<T>`, using the provided `x` and `y`
arguments to construct the vector.

For this exercise, you must write three different tests:

* The first one must be named `copy_vector` and must show the vector being *copied* multiple times.
* The second one must be named `clone_vector` and must show the vector being *cloned* multiple
times. It must not be possible to *copy* this vector.
* The thrid one must be named `compare_vector` and must verify that two `Vec<T>` can be compared
using the `==` operator as long as `T` can.

## Exercise 04: A Useful Generic Vector

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cmp::{PartialEq, Eq}  std::fmt::Debug
    std::ops::{Add, Sub, AddAssign, SubAssign}
    std::ops::{Mul, MullAssign, Div, DivAssign}
    std::{assert*} std::clone::Clone
    std::marker::Copy  f32::sqrt f64::sqrt
```

* Copy the previous exercise here (the `Vector<T>` type). This simple vector type, by itself, isn't
very useful: you cannot do anything with it.

* Overload the `+`, `-`, `+=` and `-=` operators for `Vector<T>`, for any `T` that itself has
support for those operators.

* Overload the `*`, `*=`, `/` and `/=` operators for `Vector<T>`, for any `T` that itself has support
for those operators. The second operand of those operations *must not* be `Vector<T>`, but `T`
itself, meaning that you must be able to compute `Vector::new(1, 2) * 3` but not
`Vector::new(1, 2) * Vector::new(2, 3)`.

* Implement specifically for both `Vector<f32>` and `Vector<f64>` a `length` function that computes
its length. The length of a vector can be computed using this formula: `‖(x, y)‖ = sqrt(x² + y²)`.

## Exercise 05: What Time Is It?

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::str::FromStr  std::fmt::{Display, Debug, Formatter}
    str::*  std::result::Result  std::{write, println}
```

Create a type named `Time` responsible for storing, well, a time.

```rust
struct Time {
    hours: u32,
    minutes: u32,
}

enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}
```

Implement the right traits such that the provided `main` function compiles and produces the given
output.

```rust
fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
```

Output:

```txt
>_ cargo run
12 hours, 20 minutes
15 hours, 14 minutes
error: missing ':'
error: invalid length
error: invalid number
```

