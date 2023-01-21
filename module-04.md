# Module 04: Ownership

## Introduction

A program is said to be *memory safe* when it can be proven that it cannot perform any invalid
memory operation. In Rust it means that you cannot dereference dangling pointers, index into slice
past their length, etc. The compiler goes through lenghts to make sure any program you write is
memory safe.

In the second module, we've seen that every reference you create has a *lifetime* (may it be
implicit, or explicit). We'll see in this module how those lifetimes are defined, and the rules
they must follow.

* Every value has a single owner (which itself has an owner, etc). The owner of a value is
  responsible for running its destructor.
* A reference to a value cannot live past the lifetime of their owner.
* There can either be one exclusive (`&mut T`) reference to a value, or multiple shared (`&T`)
  references to one value at any given time.

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

## Exercise 00: Drop The Mic

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::ops::Drop
```

```rust
struct Mic;

fn main() {
    let _the_mic = Mic;
    println!("Blah!");
}
```

Without touching at the above function and type definition, add the necessary declaration to make
the code print the following lines (in the same order).

```txt
>_ cargo run
Blah!
The mic has been dropped.
```

## Exercise 01: The Case Of String

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::format  std::string::String
```

Your assignment is to create 100 strings. Each string should contain the following message:

> Hey! I'm string {}!

Where `{}` is replaced by the index of the string. Indexes go from 0 to 99.

Use whatever mean you want to build that string, but you should probably check Rust's formatting
system out ;).

Once your string is ready, you have to print it to the standard output. You can check with Valgrind
that no memory has leaked.

Example:

```txt
>_ cargo run
Hey! I'm string 0!
Hey! I'm string 1!
Hey! I'm string 2!
...
```

## Exercise 02: The No-Alias Rule

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml
```

Copy the following `main` function.

```rust
fn main() {
    let mut a = 15u32;
    let e = &mut a;
    let s = &a;
    *e = 13;
    println!("{s}");
}
```

Make this program correctly compile by swapping only two lines.

## Exercise 03: Transfering Ownership

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::println  std::string::String
```

Create a **function** that takes ownership of a `String` instance and prints it.

```rust
fn move_and_print(s: String);
```

Now that your function is working properly, add a `main` function to showcase it. Can you call the
function twice on the same string? Why? You will be asked in defense.

Create a function that does the exact same thing as the `move_and_print` function, but without
taking ownership of its input. Its name should be `borrow_and_print`. It must be possible to call
this new function multiple times with the same string.

Add some calls to `borrow_and_print` to the `main` function you previously wrote. Can you put those
calls *after* the call to `move_and_print`? Why?

## Exercise 04: Duplicating Ownership

```txt
turn-in directory:
    ex04/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::string::String  std::println
```

Create a function that takes ownership of a `String` and does nothing with it.

```
fn eat_string(s: String);
```

Copy the provided `main` function and makes it compile.

```rust
fn main() {
    let s = String::from("Well, I've known no other way.");

    eat_string(s);
    eat_string(s);
    eat_string(s);

    println!("{s}");
}
```

You have to create copies (or rather, `clone`s, in that case) of the string, not create new ones
with the same name/value. You can check with Valgrind that new memory allocations have been
created.

## Exercise 05: Clone Trooper

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::{assert*}  std::string::String
```

Cloning value is a common pattern in Rust. That behavior is therefor encoded by a trait. The
`Clone` trait.

Create a type named `Trooper`.

```rust
struct Trooper {
    name: String,
    serial: u64,
}
```

Implement the `Clone` trait for the `Trooper` type. The serial number of any cloned `Trooper` is
one more than that of its original.

You must provide tests.

## Exercise 06: Owner Not Found

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::marker::Copy  std::clone::Clone
```

Write the appropriate trait implementations to make this code compile. You can't touch any of the
provided code. You are not allowed to use the `#[derive(...)]` attribute.

```rust
struct Foo {
    bar: f32,
    baz: u32,
    xyzzy: &'static str,
}

fn eat_foo(_foo: Foo) {}

fn main() {
    let foo = Foo {
        bar: 1.0,
        baz: 2,
        xyzzy: "Hello",
    };

    eat_foo(foo);
    eat_foo(foo);
    eat_foo(foo);
    eat_foo(foo);
    eat_foo(foo);

    println!("{}", foo.xyzzy);
}
```
