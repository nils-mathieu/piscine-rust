# Module 11: Interior Mutability And Smart Pointers

## Introduction

By now, you should've had your share of compiling errors and borrow checker complains. And while it
is true that most of the time, adhering to the no-alias rule is good for performance, code clarity
and code correctness, there is still some cases where accessing the same variable from two places
at the same time is *required*.

The Rust language provides some escape hatches to account for those cases: interior mutability.

You already have encountered a case for interior mutability: [`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html).
This module will explore this topic more thoroughly.

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

## Exercise 00: Immutable Swap

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

The standard library provides the [`Cell<T>`] type. This type wraps a value of type `T` allows
mutation through simple shared references. In other words, if you have a `&Cell<T>`, you can modify
the `T`.

Note that this wasn't previously possible! You can't modify a `&T`.

Create a **function** that swaps the content two `Cell<i32>`s.

```rust
fn swap_cells(a: &Cell<i32>, b: &Cell<i32>);
```

Note that the [`Cell<T>`] type does not let you directly access its underlying value. This is
because you are *still* not allowed to have two `&mut T` pointing to the same value at the same
time, or a `&T` to a value that can be modified.

Let's complicate things a bit. Implement the following function, which also swaps the content of
the cells.

```rust
fn swap_cells_string(a: &Cell<String>, b: &Cell<String>);
```

You can't use [`Cell::swap`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.swap) for
any of those two functions! That would obviously be too easy.

You must write tests for those two functions!

## Exercise 01: Sharing Is Caring (v2)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:

```

First, let's create a `Weapon` `enum`. This type may have the variants you wish, but at least two.

Example:

```rust
#[derive(Debug, Clone, Copy)]
enum Weapon {
    Club,
    Gun,
    Sword,
}
```

Then, create a `Warior` type, including a *reference* to a `Weapon`. That reference must support
interior mutability.

With that done, write a **program** that showcases two `Wariors` sharing the same weapon. Modify
that weapon and verify that the weapon of both wariors properly changed.

## Exercise 04: Errno

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::thread_local
```

The `errno` global variable available in `#include <errno.h>` is "thread-local; setting it in one
thread does not affect its value in any other thread" - `man errno`.

Let's create our own `errno`! First, create an `enum` named `Error`. This type can have the
variants of your choice.

Then, implement the following functions.

```rust
impl Error {
    fn last_error() -> Self;
    fn make_last_error(self);
}
```

The `Error::make_last_error` function must set the calling thread's last `Error` instance. The
`Error::last_error` function must return the calling thread's last `Error` instance.

Write tests to verify that the two function are indeed thread-local.

## Exercise 02: `Rc<T>`

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

You've already seen [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html), the Atomically
Reference Counter pointer. [`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html) is its
non-[`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) counterpart.

## Exercise 03: `RefCell<T>`

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

As you have may have seen in the previous exercise, `Cell<T>` can be a bit hard to use with types
that do not implement the [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait.
Sometimes, you *need* to have a mutable reference to the underlying value. For this reason, the
standard library has the [`RefCell<T>`] type.

// TODO

One downside of both [`RefCell<T>`] and [`Cell<T>`] is that neither of those types are
[`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html), meaning that they cannot be used
on multiple threads at the same time. This is because the mechanism they use to provide interior
mutable access assumes that no other thread can break any of their invariants. If you need an
thread-safe equivalent of [`RefCell<T>`], you are looking for a [`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
or a [`RwLock<T>`](https://doc.rust-lang.org/std/sync/struct.RwLock.html).

## Exercise 04: Traversing A Graph

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

// TODO

[`Cell<T>`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`RefCell<T>`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`Rc<T>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
