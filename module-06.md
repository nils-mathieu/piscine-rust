# Module 06: Standard Monads

## Introduction

This module is the first that'll actually look at the Rust Standard Library's types. The standard
library is a separate crate automatically included in every Rust project. There is ways to disable
it, but this is beyond the scope of this piscine.

The Rust standard library, while not as large as C++'s, exports lots of useful types and constructs
to help you create efficient software. This module will introduce you to some of them.

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

## Exercise 00: Maybe

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*, matches, panic}
```

The Rust type system can be used to represent optional values. Create an `enum` that can either
contain `Something(T)`, or `Nothing`. That type should be named `Maybe<T>`.

You type should implement a method called `get_or_panic`. That function should either return the
value stored in the input `Maybe<T>` instance, or panic if it contains nothing.

You should also provide two methods to quickly test whether a `Maybe<T>` instance contains
something or not.

```rust
impl<T> Maybe<T> {
    fn get_or_panic(self) -> T;

    fn contains_something(&self) -> bool;
    fn contains_nothing(&self) -> bool;
}
```

You must add tests to prove your function works as expected. You *have to* add a test that shows
that the `get_or_panic` function panics when the instance it is given contains no value.

## Exercise 01: Integer Square Root

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::option::Option  std::{assert*}
```

Because it is such a common pattern, the Rust Standard Library already defines the `Maybe<T>` type
for you. It is named `Option<T>`. That type is used to encode the potential non-existence of a
value.

Create a **function** that computes the square root of an integer. If the input of the function is
a perfect square, then its square root is returned. Otherwise, `None` is returned.

```rust
fn int_sqrt(n: u32) -> Option<u32>;
```

Example:

```rust
assert_eq!(int_sqrt(16), Some(4));
assert_eq!(int_sqrt(15), None);
```

You must provide tests to prove the function you wrote is indeed correct.

## Exercise 02: Index Of

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    <[T]>::len  std::option::Option
```

Create a generic function `index_of` that returns the index of the first element of a slice that
matches another element. If no such element is found, `None` is returned. The function should be
prototyped as follows:

```rust
fn index_of<T: PartialEq>(slice: &[T], elem: &T) -> Option<usize>;
```

You must provide tests for this function.

## Exercise 03: Niche Optimization

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml
```

Copy the following `main` function:

```rust
fn main() {
    dbg!(std::mem::size_of::<usize>());
    dbg!(std::mem::size_of::<Option<usize>>());
    dbg!(std::mem::size_of::<&u8>());
    dbg!(std::mem::size_of::<Option<&u8>>());
}
```

Can you explain why `Option<usize>` takes more space than `usize` whereas `Option<&u8>` takes as
much memory as a regular `&u8`? You will be asked during defense.

## Exercise 04: Binds & Maps

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    u32::{checked_*}  str::parse  std::result::Result  std::{assert*}
```

Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.
Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.

```rust
#[derive(Debug, PartialEq)]
enum MyError {
    CantParse,
    CantMultiply,
    CantAdd,
}
```

Write a **function** that:

* Takes a string as an input.
* Parses the string into an `u32`.
* Multiplies the result by `42`.
* Adds `100`.
* Returns the result.

The function is *never* allowed to panic, nor is it allowed to return an incorrect value. You must
write write this whole function without using a single semicolon ";".

The function should be prototyped like so:

```rust
fn my_function(input: &str) -> Result<u32, MyError>;
```

Example:

```rust
assert_eq!(my_function("12"), Ok(12 * 42 + 100));
assert_eq!(my_function("1000000000"), Err(MyError::CantMultiply));
```

*You have you write tests. ~*

## Exercise 05: Handling Errors: The Normal Way

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::println  std::eprintln  str::parse  std::result::Result
    std::num::{ParseIntError, IntErrorKind}  std::env::args
```

Create a **program** that takes exactly one argument. If no arguments (or more than one) are
passed, the program prints an error message but *does not panic*.

The single argument is parsed into an `i32` instance. If the convertion is a success, the function
prints a message indicating that the convertion is a success. If an error occurs, a message
describing the error is written.

Example:

```
>_ cargo run -- a b c
too many arguments: exactly one argument is expected
>_ cargo run -- '1234'
success
>_ cargo run -- 'a'
invalid digit: a non numerical character has been found
>_ cargo run -- '1111111111111111111111111111'
positive overflow: the provided value overflows the type `i32`
```

You must not create your own "atoi" function, use what Rust gives you!

## Exercise 06: What Time Is It?

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::str::FromStr  std::fmt::*  str::*  std::result::Result
```

Create a type named `Time` responsible for storing, well, a time.

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

Implement the right traits such that the above code produces the following output.

```txt
>_ cargo run
12 hours, 20 minutes
15 hours, 14 minutes
error: missing ':'
error: invalid length
error: invalid number
```

## Exercise 07: Comma-Separated Values

```txt
turn-in directory:
    ex07/

files to turn in:
    src/lib.rs  src/**/*.rs  Cargo.toml

allowed symbols:
    std::str::FromStr  str::*  std::result::Result
    std::vec::Vec
```

Let's create a generic CSV Encoder & Decoder.

A CSV file is defined like this:

```txt
value1,value1,value1,value1
value2,value2,value2,value2
value3,value3,value3,value3
...
```

Each line corresponds to a *record*, and each column corresponds to a *field*.

First, let's create a trait for types which may be encoded and decoded into a field value. This
trait should define a way to write an ASCII representation of the value, as well as a way to parse
a string into a concrete instance of the type. Error type may be as simple as unit structs.

Example:

```rust
struct EncodingError;
struct DecodingError;

trait Field {
    /* ... */
}
```

You should implement the `Field` type for common types, such as `&str`, `u32` or `char`. Errors
(such as invalid characters in a `&str`, or a numeric literal being too large) should return an
error instead of panicking.

With that out of the way, let's create a `Record` trait, which provides a way to access all of its
`Field`s, as well as a way to construct an instance of itself from a list of strings.

**Hint:** you might want to use dynamic dispatch (`dyn Field`) for that.

```rust
trait Record {
    /* ... */
}
```

Now, you have everything you need to create `decode_csv` and `encode_csv` functions.

```rust
fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError>;
fn decode_csv<R: Record>(contents: &str) -> Result<Vec<R>, DecodingError>;
```

Optionally, you can try to create a macro to implement the `Record` trait automatically:

```rust
struct MyType<'a> {
    id: u32,
    name: &'a str,
}

// Example:
impl_record!(MyType<'a>(u32, &'a str));
```

Write extensive tests for the two functions.
