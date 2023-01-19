# Module 06: Standard Monads

## Introduction

This module is the first that'll actually look at the Rust Standard Library's types. The standard
library is a separate crate automatically included in every Rust project. There is ways to disable
it, but this is beyond the scope of this piscine.

The standard library, while not as large as C++'s, exports lots of useful types and constructs to
help you create efficient software. This module will introduce you to some of them.

## General Rules

Any program you turn in should compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise you turn in that uses the `cargo` package manager must be part
of the `workspace.members` table declared for the whole module.

Any program you turn in should compile *without warnings* using the `rustc` compiler available on
the school's machines without additional options. You are allowed to use attributes to modify lint
levels, but you must be able to explain why you did so. You are *not* allowed to use `unsafe` code
anywere in your code.

## Exercise 00: Maybe

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

The Rust type system can be used to represent optional values. Create an `enum` that can either
contain `Something(T)`, or `Nothing`. That type should be named `Maybe<T>`.

You type should implement a method called `get_or_panic`. That function should either return the
value stored in the input `Maybe<T>` instance, or panic if it contains nothing.

You should also provide two methods to quickly test whether a `Maybe<T>` instance contains
something or not.

```Rust
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

allowed dependencies:

```

Because it is such a common pattern, the Rust Standard Library already defines the `Maybe<T>` type
for you. It is named [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html). That
type is used to encode the potential non-existence of a value.

Create a **function** that computes the square root of an integer. If the input of the function is
a perfect square, then its square root is returned. Otherwise, `None` is returned.

```Rust
fn int_sqrt(n: u32) -> Option<u32>;
```

Example:

```Rust
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

allowed dependencies:

```

Create a generic function `index_of` that returns the index of the first element of a slice that
matches another element. If no such element is found, `None` is returned. You are only allowed to
use the [`len`](https://doc.rust-lang.org/std/primitive.slice.html#method.len) function, as well as
the indexing operator `slice[...]`. The function should be prototyped like this:

```Rust
fn index_of<T: PartialEq>(slice: &[T], elem: &T) -> Option<usize>;
```

You must provide tests for this function.

## Exercise 03: Niche Optimization

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

Copy the following `main` function:

```Rust
fn main() {
    dbg!(std::mem::size_of::<usize>());
    dbg!(std::mem::size_of::<Option<usize>>());
    dbg!(std::mem::size_of::<&u8>());
    dbg!(std::mem::size_of::<Option<&u8>>());
}
```

Can you explain why `Option<usize>` takes more space than `usize` whereas `Option<&u8>` takes as
much memory as a regular `&u8`?

## Exercise 04: Binds & Maps

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.
Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.

```Rust
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

```Rust
fn my_function(input: &str) -> Result<u32, MyError>;
```

Example:

```Rust
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

allowed dependencies:

```

Create a **program** that takes exactly one argument. You can look at the [`std::env::args()`](https://doc.rust-lang.org/std/env/fn.args.html)
function to learn how to retrieve those.

If no arguments (or more than one) are passed, the program prints an error message.

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

## Exercise 06: Invalid Credentials

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit
```

Create a type named `LoginError` which will represent a connection error (to an admin page, for
example). It should represent any of the following errors.

* The server is not available. This will randomly happen once every five attempts on average.
* The passed credentials are invalid and do not match any existing user.
* The credentials were valid but the user do not have the required permissions to access the admin
  page.

Now that you are done with the error type, let's create the function that uses it. Create a
function named `admin_login`, taking a `login` and a `password`.

```Rust
type Secret = u32;

fn admin_login(login: &str, password: &str) -> Result<Secret, LoginError>;
```

If the function succeeds, a secret number is returned by the function. If an error occurs, the
right `LoginError` variant should be returned.

Now that you have built your awesome web server, you can create a `main` function!

* If the number of arguments passed to the function is not `2`, an error is displayed and the
  program stops.
* Otherwise, the first argument is the login, and the second one is the password. Those values are
  passed to the `admin_login` function.

Example:

```txt
>_ cargo run -- xXdarkXx chien123
internal server error
>_ cargo run -- xXdarkXx chien123
invalid credentials
>_ cargo run -- xXdarkXx chien23
not an admin
>_ cargo run -- marvin mynameismarvin
internal server error
>_ cargo run -- marvin mynameismarvin
secret code: 42
```
