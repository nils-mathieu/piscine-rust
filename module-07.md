# Module 07: Common Standard Collections

## Introduction

Manipulating memory directly is a very involved process and Rust cannot give you the power to do it
without, at the same time, giving you ways to shoot yourself in the foot. For example, in Rust,
functions like `malloc` and `free` cannot be used directly, because the lifetime of the memory they
create does not behave like normal references created to values living on the stack. Instead, such
reference lives "until it is `free`d", which can be hard to keep track for the Borrow Checker.

The Rust Standard Library hence provides some structures to safely access allocated memory
addresses. This module will teach you how to use a few of those data-structures.

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

## Exercise 00: A Singly-Linked List

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::boxed::Box  std::option::Option
```

Let's create a simple linked-list. As said in the introduction, you can't directly allocate memory
on the heap using something like `malloc`.

Create a `Node<T>` type, respondible for storing an instance of `T`, as well as an optional `next`
`Node<T>`.

With that, create a `List<T>` tpye which simply contains an optional first `Node<T>`.

```rust
struct List<T> {
    head: Option<Node<T>>
}

impl<T> List<T> {
    /// Creates a new, empty, list.
    fn new() -> Self;

    /// Adds an element at the end of the list.
    fn push_back(&mut self, elem: T);
    /// Adds an element at the front of the list.
    fn push_front(&mut self, elem: T);

    /// Removes the last element of the list and returns it.
    fn remove_back(&mut self) -> Option<T>;
    /// Removes the first element of the list and returns it.
    fn remove_front(&mut self) -> Option<T>;

    /// Removes all elements from the list.
    fn clear(&mut self);
}
```

In any case, you must write tests for mandatory functions.

## Exercise 01: List Of Indexes

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::vec::Vec  str::char_indices
```

Create a **function** that takes a string as input, and returns the index of every occurence of a
character `c` in that string. The function should be prototyped as follows:

```rust
fn indexes_of(s: &str, c: char) -> Vec<usize>;
```

Keep in mind that the returned vector must contains *indexes*, not count characters. The example
bellow shows that behaviour.

Example:

```rust
let indexes = indexes_of("les élèves s'élèvent", 'e');
assert_eq!(indexes, [1, 10, 21]);
```

You must write tests!

## Exercise 02: Cumulative Sum

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::vec::Vec
```

Create a **function** named `cumulative_sum` which takes a slice as an input, and computes its
cumulative sum.

```rust
fn cumulative_sum(series: &[u32]) -> Vec<u32>;
```

Example:

```rust
assert_eq!(cumulative_sum(&[1, 2, 2, 3]), [1, 3, 5, 8]);
```

You must write proper tests for your function.

## Exercise 03: For Each Move, For Each Reference

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs

alowed symbols:
    std::string::String::len
```

Create a **function** that looks for a `String` of length `len` in its provided vector. If found, the
`String` is returned. Otherwise, `None` is returned. If multiple strings of the same length are
found in the vector, only the first one is returned.

```rust
fn ind_str_of_len(v: Vec<String>, len: usize) -> Option<String>;
```

You must write tests for your function!

With that out of the way, create a new **function** that returns a reference to the first string of
length `len`.

```rust
fn find_str_of_len_ref(v: &Vec<String>, len: usize) -> Option<&String>;
```

You must write tests for those two functions.

## Exercise 04: `Vec<T>` And `[T]`

```txt
turn-in directory:
    ex04/

files to turn in:
    sec/lib.rs

allowed symbols:
    <[T]>::swap  std::vec::Vec::{push, pop}
```

Taking a constant reference to a `Vec<T>` does not make much sense since you can't really do
anything more than what you'd be able to with a regular `&[T]` slice. However, mutable
reference are very much different.

```rust
fn reverse<T>(list: /* ??? */);
fn resize<T: Default>(list: /* ??? */, new_size: usize);
```

* The `reverse` function reverses the values of the list it is given. For example, `[1, 2, 3]`
  becomes `[3, 2, 1]`.

* The `resize` function changes the size of the list it is given, ensuring that its length is
`new_size`. If the list is not large enough, the default value of the type are added at the end of
the list. Otherwise, the values simply removed.

Implement both of these functions, replacing the `/* ??? */` comment with an appropriate storage
type (either `&mut Vec<T>` or `&mut [T]`). You will have to explain your choice during defense.

Write tests for both functions.

## Exercise 06: Dictionary

```txt
turn-in directory:
    ex06/

files to turn in:
    sec/main.rs

allowed symbols:
    std::collection::HashMap  std::env::args
```

Create a dictionary program. It should include some definitions and can be used as specified in
this example:

```txt
>_ cargo run -- coder
synonyme de "stack overflow"
>_ cargo run -- 42
42 est le nombre de fois qu'il faut plier une feuille de papier de 0,1 mm pour
obtenir la distance approximative Terre-Lune (439 000 km)
>_ cargo run -- C++
mathématiquement inférieur à Rust
>_ cargo run -- "cache invalidation"
error: I do not know anything about that...
>_ cargo run -- a b c
error: Woah, I can't look up more than one word! Who do you take me for?
A computer?
>_ cargo run
error: Yes? What do you want? I think you forgot how to speak...
```

The defnitions can be modified, words can be added, error messages can be edited.

You must *not* use a forest of `if`/`if else` statements. The standard library contains
datastructures specifically designed to do this, and you should use them.
