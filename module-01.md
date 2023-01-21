# Module 01: References And Slices

## Introduction

Rust is basically operating on the same hardware abstraction level as C. As such, it does have a
way to create pointers to any existing value. In Rust, however, it is *impossible* to create
invalid pointers. When using that language, the compiler *ensures* statically that every pointer
you create won't ever be invalidated while you are using it. To provide this guarentee, Rust uses
a system known as the *Borrow Checker*.

Rust's Borrow Checker can be a bit hard to get used to, but remember that 99% of the program it
rules out are actually invalid and could potentially lead to memory unsafety and undefined
behavior. This module will introduce you to how it works, and what information it uses to
determine whether a program is valid or not.

## General Rules

Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise must be part of a virtual Cargo workspace, a single
`workspace.members` table must be declared for the whole module.

Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options. You are allowed to use attributes to modify lint levels, but
you must be able to explain why you did so. You are *not* allowed to use `unsafe` code anywere in
your code.

## Exercise 00: Creating References

```txt
turn-in directory:
    ex00/

files to turn-in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

Creating a reference isn't exactly an involved process. Using those references properly can be
quite a bit harder, however.

Create a **function** that adds two integers together. It must be prototyped as follows:

```rust
fn add(a: &i32, b: i32) -> i32;
```

Now, create another function, but this time, it must store the result of the operation in the
first number.

```rust
fn add_assign(a: &mut i32, b: i32);
```

How does using a `&mut` or a `&` change the semantics of the function? Would it be possible to
crates an `add_assign` function using a regular `&i32` reference (without the `mut`)? You should
be able to answer those questions during the defense.

You must provide some tests to prove every function behaves as expected.

## Exercise 01: Dangling References

```txt
turn-in directory:
    ex01/

files to turn-in:
    src/main.rs  Cargo.toml
```

Rust won't ever allow you to create a dangling reference (a reference whose pointed value has been
lost).

```rust
fn main() {
    let b;

    {
        let a: i32 = 0;
        b = &a;
    }

    println!("{b}");
}
```

This exercise simply requires you to understand why above code does not compile (and why it
*shouldn't* compile). Don't try to fix it, and just be prepared to being asked what happened here
during defense.

Copy this flawed `main` into your project, comment it so that the project still compiles and move
on to the next exercise.

## Exercise 02: Point Of No Return (v2)

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

Do you remember the point of the exercise 01 from the first module? You had to create a function
prototyped as so:

```rust
fn min(a: i32, b: i32) -> i32;
```

The assignment of this exercise is to write the same exact function, but this time, the inputs of
this function are references.

```rust
fn min(a: &i32, b: &i32) -> &i32;
```

The above function returns the reference to the smallest integer among `a` and `b`. Note that you
may have to add some *lifetime annotations* to the function in order to make it compile.

In addition to the usual tests you have to write to prove the function you wrote is actually valid,
you must create a `"spike"` test that showcases how *not* having those annotations could lead to
dangling references. You must comment that non-compiling test out before pushing. You will have to
explain that fairly difficult concept to your evaluators!

## Exericse 03: Array Addition

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}
```

Your assignment is to create a **function** that adds two instances of `[i32; 3]` index-wise.

```rust
fn add_vectors(a: [i32; 3], b: [i32; 3]) -> [i32; 3];
```

Example:

```rust
let a = [1, 2, 3];
let b = [2, 3, 4];
assert_eq!(add_vectors(a, b), [3, 5, 7]);
```

You must write tests to prove your function is behaving as expected.

## Exercise 04: Smallest Subslice

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}  <[u32]>::len
```

Create a **function** that computes the smallest subslice whose sum is above a given treshold. When
multiple sub-slices of the same length are above the treshold, the first one is returned. If no
such slice is found, the function panics.

```rust
fn smallest_subslice(slice: &[u32], threshold: &u32) -> &[u32];
```

Example:

```txt
smallest_subslice([10, 1, 11], 11) => [11]
smallest_subslice([10, 1, 11], 12) => [1, 11]
smallest_subslice([10, 1, 11], 13) => [10, 1, 11]
smallest_subslice([10, 1, 11], 23) => panic!("no subslice found...")
```

Once again, you may need to specify some *lifetime annotations* for the function. To check whether
your annotations are correct for that case, you can use this pre-defined `test_lifetimes` function.
It must compile.

```rust
#[test]
fn test_lifetimes() {
    let array = [3, 4, 1, 2, 12];
    let result;

    {
        let threshold = 1000;
        result = smallest_subslice(&array, &threshold);
    }

    assert_eq!(result, &[]);
}
```

Other than that, you must provide tests. One of those tests must show that the function panics
properly when no subslice is found.

## Exercise 05: Sorting A Slice

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}  <[i32]>::{len, swap}
```

Iterating over an array is fine, but doing that while modifying it is better!

Create a **function** that sorts a slice of `i32`s.

```rust
fn sort_slice(slice: &mut [i32]);
```

You must provide tests!

## Exercise 06: Is That A `\0` Is My String??

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*}  str::as_bytes
```

Create a **function** that finds the first `\0` character of a given string, and splits it into
two part. The first part must contain all the characters until the `\0`, and the second one must
contain all other characters (without the `\0`).

```rust
fn split_once_at_null(s: &str) -> (&str, &str);
```

If no `\0` is found in the string, the function panics with an appropriate message.

Example:

```rust
assert_eq!(split_once_at_null("Hello\0World"), ("Hello\0", "World"));
```

You must write tests for this function! Specifically, there must be a test that verifies the
function properly panics when given a string with no `\0`.

## Exercise 07: Static References

```txt
turn-in directory:
    ex07/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    std::{println, print, panic, assert*}  ftkit::random_number
```

Create a **function** that returns a string associated to a given key. If the key is invalid, the
function is allowed to panic. The valid keys are numbers between 0 and 4 (included).

```rust
fn get_string(key: &i32) -> &str;
```

You will have to add the correct *lifetime annotations* to ensure the provided `main` function
compiles.

```rust
fn main() {
    let result;

    {
        let key = ftkit::random_number(0..=4);
        result = get_string(&key);
    }

    println!("{result}");
}
```

Example:

```
>_ cargo run
Bonjour!
>_ cargo run
Bonjour!
>_ cargo run
Hello!
```

This is only an example, you are free to choose the values actually returned by the `get_string`
function.

## Exercise 08: The Size Of Slices

```txt
turn-in directory:
    ex08

files to turn in:
    src/main.rs  Cargo.toml
```

Let's finish this module with an easy one.

Copy this bit of code inside of the `main` function.

```rust
dbg!(std::mem::size_of::<i32>());
dbg!(std::mem::size_of::<&i32>());
dbg!(std::mem::size_of::<[i32; 6]>());
dbg!(std::mem::size_of::<&[i32; 6]>());
dbg!(std::mem::size_of::<&[i32]>());
```

Do you understand why it prints those values? You will be asked during defense.
