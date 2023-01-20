# Module 05: Generics

## Introduction

Dynamic dispatch can be very useful to avoid repeating yourself. In fact, this is what
inheritance-driven languages usually use (think Java, C# or C++ without templates).

In Rust, we tend to use static dispatch by default. The idea is pretty simple: re-compile the
function for every possible type that needs it. This allows more optimized code, but longer compile
times and potentially larger binary sizes. In practice, the binary size can be easily managed. As
for compilation times, this is still [a work in progress issue](https://perf.rust-lang.org/).

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

## Exercise 00: `choose`

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    <[T]>::len  std::println
```

Create a **function** that randomly chooses a value among an input slice.

```Rust
fn choose<T>(values: &[T]) -> &T;
```

You can write a `main` function to show that the function works as expected.

## Exercise 01: Print Yourself, I'll Greet You

```txt
turn-in directory:
    ex01/

file to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::{print, println}
```

Copy the following trait into your `main.rs` file.

```Rust
impl PrintMyself {
    fn print_myself(&self);
}
```

And implement it for some basic types (such as `u32` or `i8`).

Create a `greet` function, which takes any value as an input, as long as the type of that value
derives the `PrintMyself` trait. When called, this function must print the following message:

> Hey, **name**! How are you?

Where **name** can be replaced by the text displayed by the `print_myself` function.

Create a `main` function that showcases this function being called with different types as input.

## Exercise 02: A Generic Vector

```txt
turn-in directory:
    ex02/

file to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::fmt::*  std::cmp::PartialEq  std::{assert*}
```

Structures too, can be generic over some other type.

Create a `Vector` type.

* It must be generic over some type `T`.
* It must have two fields `x` and `y`, both of type `T`.

The `Vector` type must have a `new` associated function to create an instance of `Vector`. The
prototype of that function should be:

```Rust
impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self;
}
```

Use the `#[derive(...)]` attribute to derive the `Debug` and `PartialEq` traits for `Vector<T>`
and write a simple tests to make sure the `new` function and those trait implementations do work
as expected.

## Exercise 03: A Useful Generic Vector

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::fmt::*  std::cmp::*  std::ops::*  std::{assert*}
    std::clone::Clone  std::marker::Copy  f32::sqrt
    f64::sqrt
```

Copy the previous exercise here (the `Vector<T>` type). This simple vector type, by itself, isn't
very useful: you cannot do anything with it.

Overload the `+`, `-`, `+=` and `-=` operators for `Vector<T>`, for any `T` that itself has support
for the `+`, `-`, `+=` and `-=` operators (respectively). You must provide additional tests for
those new functions.

Not every type has support for the square root operation. In fact, only `f32` and `f64` have an
associated `sqrt` function.

Implement specifically for both `Vector<f32>` and `Vector<f64>` a `length` function that computes
its length. The length of a vector can be computed using this formula: `‖(x, y)‖ = sqrt(x² + y²)`.

You have to provide more tests for that.

## Exercise 04: Point Of No Return (v3)

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cmp::PartialOrd  std::{assert*}
```

Again? Yes. Another `min` function! But I promise, this one's the last one.

Create a `min` function that takes *any* two values of a type that supports the `<` operator, and
returns the smaller one.

```Rust
assert_eq!(min(12i32, -14i32), -14);
assert_eq!(min(12u32, 14u32), 12);
assert_eq!(min("abc", "def"), "abc");
```

You must provide tests for your function.

## Exercise 05: Saturating Convertion

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

Create a generic trait named `StaturateInto` with one method. This trait should be generic
over a `T`. It should require one method named `saturate_into` whose prototype is:

```Rust
fn saturate_into(self) -> T;
```

Where `T` is the output of the convertion. If the convertion isn't possible (such as trying to
convert `260u32` into an `u8`), the function should use the maximum or minimum value of the target
type (depending on whether the input value is too large or too small).

Example:

```Rust
assert_eq!(10u32.saturate_into(), 10u8);
assert_eq!(-16i8.saturate_into(), -16i16);
assert_eq!(-200i32.saturate_into(), 0u32);
```

Implement the `SaturateInto` trait for some types. Don't bother implementing it for every possible
combinaison of primitive types (unless you want to use this exercise as a way to learn
[`macro_rules!`]). You'll simply have to provide *some* implementations to showcase how the trait
is used.

Anything can be converted into itself. Formally, for any given type `T`, it's possible to implement
the `SaturateInto<T>` trait. Create a *blacket implementation* of the `SaturateInto<T>` trait for
every `T`. In that case, the `saturate_into` method simply returns its input.

You must provide tests for the trait implementations.

## Exercise 06: Standard Convertion Traits

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

The Rust standard library already provides traits to convert values [`From`] and [`Into`] other
values. Those traits are already implemented for most of the Standard Library's types.

Copy the following type definition into your file.

```Rust
struct EvenValue(u32);
```

It should implement the `From<u32>` trait. When the value it is given is even, the function returns
an instance of `EvenValue` containing the provided `u32` instance. If the value is odd, the
function panics.

Create tests for your implementation! You must include the following tests.

```Rust
#[test]
fn even_into_even() {
    let v: EvenValue = 12u32.into();
    assert_eq!(v.0, 12);
}

#[test]
#[should_panic]
fn odd_into_even() {
    let _v: EvenValue = 11u32.into();
}
```

Why does the `u32` type suddenly has the `into()` method?? You only implemented the `From<u32>`
trait...

## Exercise 07: Pseudo-Random

```txt
turn-in directory:
    ex07/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    ftkit::random_number  std::println  std::maker::Sized
    {u8, u16, ..}::{from_ne_bytes, to_ne_bytes}
```

Create a `Generate` trait with a single associated method:

```rust
fn generate() -> Self;
```

The `generate` method should generate a random instance of the implementator.

Implement this trait for several types, such as `u8` or `u64` and showcase this trait being used
in a `main` function.
