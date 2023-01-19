# Module 04: Ownership

## Introduction

A program is said to be *memory safe* when it can be proven that it cannot perform any invalid
memory operation. In Rust it means that you cannot dereference dangling pointers, index into slice
past their length, etc. The compiler goes through lenghts to make sure any program you write is
memory safe.

In the second module, we've seen that every reference you create has a *lifetime* (may it be
implicit, or explicit). We'll see in this module how those lifetimes are defined, and the rules
they must follow.

The rules are simple, but can be a bit hard to use in practice:

* Every value has a single owner (which itself has an owner, etc).
* A reference to a value cannot live past the lifetime of their owner.
* There can either be one exclusive (`&mut T`) reference to value, or multiple shared (`&T`)
references to one value at any given time.

## General Rules

Any program you turn in should compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified in
the `allowed dependencies` section are allowed.

Any program you turn in should compile *without warnings* using the `rustc` compiler available on
the school's machines without additional options. You are allowed to use attributes to modify lint
levels, but you must be able to explain why you did so. You are *not* allowed to use `unsafe` code
anywere in your code.

## Exercise 00: Dropping The Mic

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

In Rust, the owner of a value is responsible for dropping it. The most basic value owner is
functions (more like scopes, specifically). When a function returns, the values that were created
inside of it are said to "go out of scope" and are no longer needed. When that happens, those
values are dropped. Dropping a value may run a bit of code (usually, this code frees the resources
owned by the value, such as `malloc`s, or so).

For example:

```
struct Mic;

fn main() {
    let _the_mic = Mic; // The `_` prefix is used to silence the `unused_variable` warning.
    println!("Blah!");
}
```

Here, the `main` function is responsible for dropping `_the_mic`.

Without touching at the above function and type definition, add the necessary declaration to make
the code print the following lines (in the same order).

```txt
>_ cargo run
Blah!
Drop The Mic!
```

**Hint:** The mic must be [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)ed.

## Exercise 01: The Case Of String

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

[`String`](https://doc.rust-lang.org/std/string/struct.String.html) is a `malloc`d `str`. We say
that `String` "owns" a `str`. Why does it makes sense for `String` to own it? Well, the owner of a
value is responsible for dropping it. Since `String` contains a `malloc`, it is responsible for
`free`ing it when it does not need it anymore.

However, when `String` is dropped, and that the `malloc` pointer is `free`d, any reference
currently pointing to the stored `str` becomes invalid! Fortunately, the Rust ownership model is
here to save us: it ensures that no reference can be created with a lifetime larger than that of
their owner. That ensures that when the `String` is dropped and that the `malloc` is `free`d, no
reference can exist to that block of memory, upholding memory safety.

For now, you assignment is to create 100 strings. Each string should contain the following message:

> Hey! I'm string {}!

Where `{}` is replaced by the index of the string. Indexes go from 0 to 99.

Use whatever mean you want to build that string, but you should probably check Rust's formatting
system out ;).

Once your string is ready, you have to print it to the standard output. You can check with valgrind
that no memory has leaked.

## Exercise 02: The No-Alias Rule

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

In module 01, you witnessed the first rule of Rust's ownership model: a reference cannot outlive
its owner. This exercise will showcase the second one: no the alias rule.

In Rust, you cannot have an exclusive reference (&mut T) alongside any other reference to the same
value.

Copy the following `main` function.

```Rust
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

allowed dependencies:

```

The ownership of a value can be transfered from one owner to another. This is called "moving". This
can only be done when no reference exist to said value.

Once ownership has been transfered, the previous owner is not allowed to access the value anymore:
it has been moved somewhere else.

Create a **function** that takes ownership of a `String` instance and prints it.

```Rust
fn move_and_print(s: String);
```

Now that your function is working properly, add a `main` function to showcase it. Can you call the
function twice on the same string? Why? You will be asked in defense.

Create a function that does the exact same thing as the `move_and_print` function, but without
taking ownership of its input. Its name should be `borrow_and_print`. It must be possible to call
this new function multiple times with the same string.

Add some calls to `borrow_and_print` to the `main` function you previously wrote. Can you put those
calls *after* the call to `move_and_print`? Why?

## Exercise 04: Cloning Values

```txt
turn-in directory:
    ex04/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

Using a function that takes ownership of the values you give it isn't very convenient.

Create a function that takes ownership of a `String` and does nothing with it.

```
fn eat_string(s: String);
```

Copy the provided `main` function and makes it compile.

```Rust
fn main() {
    let s = String::from("Hello!");

    eat_string(s);
    eat_string(s);
    eat_string(s);

    println!("{s}");
}
```

You have to create copies (or rather, `clone`s, in that case) of the string, not create new ones
with the same name/value. You can check with Valgrind that new memory allocations have been
created.

## Exercise 05: The `Clone` Trait

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Cloning value is a common pattern in Rust. That behavior is therefor encoded by a trait. The
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait.

Create a type named `BlogPost`.

```Rust
type Id = u64;

struct BlogPost {
    name: String,
    content: String,
    author: Id,
    id: Id,
}
```

Implement the `Clone` trait for the `BlogPost` type manually (you cannot use the `#[derive(...)]`
attribute).

You must provide tests.

## Exercise 06: The `Copy` Trait

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

You might've noticed that some values didn't need to be explicitely cloned. Types like `u32`, `i8`
or `u64` didn't require you to `.clone()` them! This is because those types implement the
[`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait. The ownership of a value
deriving the `Copy` trait cannot be transfered. Instead, the bits of the value are simply copied
and the value is duplicated.

Formally, a `Copy` type is a type whose bits can simply be copied around (using `memcpy`) without
extra care. This is the case for `u32`, but can you see how implementing `Copy` for `String` could
crash a program?

```Rust
struct Foo {
    bar: f32,
    baz: u32,
    xyzzy: &'static str,
}
```

Implement the `Copy` trait for the above type manually (you still cannot use the `#[derive(...)]`
attribute).

You have to write unit tests for the functions you wrote.
