# Module 11: Interior Mutability

## Introduction

By now, you should've had your share of compiling errors and borrow checker complains. And while it
is true that most of the time, adhering to the no-alias rule is good for performance, code clarity
and code correctness, there is still some cases where accessing the same variable from two places
at the same time is *required*.

The Rust language provides some escape hatches to account for those cases: interior mutability.

You already have encountered a case for interior mutability: `Mutex<T>`. This module will explore
this topic more thoroughly.

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

## Exercise 00: Immutable Swap

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::cell::Cell::{get, set, replace, take}
```

The standard library provides the `Cell<T>` type. This type wraps a value of type `T` allows
mutation through simple shared references. In other words, if you have a `&Cell<T>`, you can modify
the `T`.

Note that this wasn't previously possible! You can't modify a `&T`.

Create a **function** that swaps the content two `Cell<i32>`s.

```rust
fn swap_cells(a: &Cell<i32>, b: &Cell<i32>);
```

Note that the `Cell<T>` type does not let you directly access its underlying value. This is
because you are *still* not allowed to have two `&mut T` pointing to the same value at the same
time, or a `&T` to a value that can be modified.

Let's complicate things a bit. Implement the following function, which also swaps the content of
the cells.

```rust
fn swap_cells_string(a: &Cell<String>, b: &Cell<String>);
```

You must write tests for those two functions!

## Exercise 01: Sharing Is Caring (v2)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::cell::Cell
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

## Exercise 02: Errno

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::thread_local  std::cell::Cell
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

## Exercise 03: Drop Detector

```txt
turn-in directory:
    ex04/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::ops::Drop  std::println  std::rc::Rc  std::cell::Cell  std::{assert*}
```

Create a type that detects when it is dropped. Specifically, every time an instance of this type
is dropped, it must update a value. That value must obiously be stored *outside* of itself so that
you can check it afterwards.

The type should roughly work like that:

```rust
let count = /* ... */;

let checked_drop = vec![
    DropDetector::new(count.clone()),
    DropDetector::new(count.clone()),
    DropDetector::new(count.clone()),
    DropDetector::new(count.clone()),
    DropDetector::new(count.clone()),
];

assert_eq!(count.get(), 0);
drop(DropDetector::new(count));
assert_eq!(count.get(), 1);
drop(checked_drop);
assert_eq!(count.get(), 6);
```

Write more tests for your type. Try multiple containers, and ways to store the `DropDetector`s.

## Exercise 04: Internet

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cell::RefCell  std::iter::*  std::rc::Rc  std::{assert*}
    std::string::String
```

Create a type named `Internet`. This type must have an associated `send_message` which stores a
message inside of internet. It must also have a `messages_of` method that returns an iterator over
someone's messages.

```rust
impl Internet {
    fn send_message(&self, author: &str, message: &str);
    fn messages_of(&self, author: &str) -> impl Iterator<Item = Ref<&str>>;
}
```

Next, let's create a `Phone` type. Each `Phone` is logged in as a specific user, and every message
sent by the `Phone` will use that name.

```rust
impl Phone {
    fn new(connection: &Internet, name: &str) -> Self;
    fn send(&self, message: &str);
}
```

Create tests to showcase multiple phones communicating through "internet", as well as messages
being retrieved using `messages_of`.

## Exercise 05: A Double-Linked List

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cell::{RefCell, Ref, Mut}  std::{assert*}  std::rc::{Rc, Weak}
```

You previously created a singly-linked list using `Box<T>`. Creating a doubly-linked list that way
is not possible in Rust.

Create a doubly-linked list.

```rust
struct NodeCursor<'list, T> { /* ... */ }
struct List<T> { /* ... */ }

impl<T> List<T> {
    /// Creates a new, empty, [`List<T>`].
    fn new() -> Self;

    /// Returns a cursor to the head of the list if it is not empty.
    fn cursor<'a>(&'a self) -> Option<NodeCursor<'a, T>>;

    /// Clears the list, removing any existing element.
    fn clear(&mut self);
}

impl<'a, T> NodeCursor<'a, T> {
    /// Tries to advance the cursor by one.
    ///
    /// If the function lands on a node, a reference to the value of that node
    /// is returned. Otherwise, `None` is returned.
    fn move_next(&mut self) -> Option<Mut<T>>;

    /// Tries to advance the cursor back by one.
    ///
    /// If the function lands on a node, a reference to the value of that node
    /// is returned. Otherwise, `None` is returned.
    fn move_prev(&mut self) -> Option<Mut<T>>;

    /// Tries to borrow the value beneath the cursor.
    fn value(&self) -> Ref<T>;
    /// Tries to mutably borrow the value beneath the cursor.
    fn value_mut(&self) -> Mut<T>

    /// Removes the current node, consuming the cursor.
    fn remove(self);

    /// Inserts a new node *after* the current one.
    fn insert_after(&self, value: T);
    /// Inserts a new node *before* the current one.
    fn insert_before(&self, value: T);
}
```

You must write extensive tests for your functions and types. Specifically, you must never leak
memory. You can check that with Valgrind.

