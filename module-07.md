# Module 07: Common Standard Collections

## Introduction

The Rust standard library may not be as large as C++'s, but it does contain some very useful
data-structures.

Specifically, manipulating memory directly is a very involved process and Rust cannot give you the
power to do it without, at the same time, giving you ways to shoot yourself in the foot. For
example, functions like `malloc` and `free` cannot be used directly, because the lifetime of the
memory they create does not behave like normal references created to values living on the stack.

This module will teach you how to use a few of those data-structures.

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

## Exercise 00: A Singly-Linked List

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

The simplest kind of container is the [`Box<T>`]. A [`Box<T>`] is just a pointer to a `malloc`ed
instance of `T`. It is called a "smart" pointer because it automatically `free`s its pointer when
it is [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)ed.

Let's create a simple linked-list using [`Box<T>`]s.

Create a `Node<T>` type, respondible for storing an instance of `T`, as well as an optional `next`
    
`Node<T>`.

With that, create a `List<T>` tpye which simply contains an optional `Node<T>`.

```Rust
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

    /// Removes the last element of the list.
    fn remove_back(&mut self) -> Option<T>;
    /// Removes the first element of the list.
    fn remove_front(&mut self) -> Option<T>;

    /// Removes all elements from the list.
    fn clear(&mut self);
}
```

In any case, you must write tests for mandatory functions.

[`Box<T>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html

## Exercise 01: List Of Indexes

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Storing values on the stack is cool and all. But sometimes, it's not possible to know in advance
the number of things you will have to store. Rust gives you several abstractions over collections
of elements, available under the [`collections`](https://doc.rust-lang.org/std/collections/index.html)
module of the Standard Library.

Create a **function** that takes a string as input, and returns the index of every occurence of a
character `c` in that string. The function should be prototyped as follows:

```Rust
fn indexes_of(s: &str, c: char) -> Vec<usize>;
```

Keep in mind that the returned vector must contains *indexes*, not count characters. The example
bellow shows that behaviour.

Example:

```Rust
let indexes = indexes_of("les élèves s'élèvent", 'e');
assert_eq!(indexes, [1, 10, 21]);
```

You must write tests!

## Exercise 02: `ft_split`

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Create a **function** that splits a string `s` using every character in `sep` as a separator. It
should be prototyped like so:

```Rust
fn ft_split(s: &str, sep: &str) -> Vec<&str>;
```

You may have to add *lifetime annotations* to the function ;). Try to make those as general as
possible: the `sep` reference shouldn't be bound to `s`'s lifetime.

Example:

```Rust
let substrings = ft_split("Hello\tWorld \t Test  ", "\t ");
assert_eq!(substrings, ["Hello", "World", "Test"]);
```

You must write tests for the function! More tests! More tests! More tests!

## Exercise 03: For Each Move

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs

allowed dependencies:

```

The elements stored in a `Vec<T>` are owned. Dropping the vector will automatically drop its
elements. One way to access the content of a vector is using `for` loops.

Create a function that looks for a `String` of length `len` in its provided vector. If found, the
`String` is returned. Otherwise, `None` is returned. If multiple strings of the same length are
found in the vector, only the first one is returned.

```Rust
fn find_str_of_len(v: Vec<String>, len: usize) -> Option<String>;
```

You must write tests for your function!

## Exercise 04: For Each Reference

```txt
turn-in directory:
    ex04/

files to turn in:
    sec/lib.rs

allowed dependencies:

```

Sometimes you might want to avoid losing a whole vector every time you're looking for something
inside of it. In other words, instead of *moving* it, you just want to *borrow* it.

Create a **function** that returns a reference to the first string of length `len`.

```Rust
fn find_str_of_len(v: &Vec<String>, len: usize) -> Option<&String>;
```

Note that, this time, the ownership of the vector is not transfered to the `find_str_of_len`
function.

## Exercise 05: `Vec<T>` And `[T]`

```txt
turn-in directory:
    ex05/

files to turn in:
    sec/lib.rs

allowed dependencies:

```

Taking a constant reference to a `Vec<T>` does not make much sense since you can't really do
anything more than what you'd be able to with a regular `&[String]` slice. However, mutable
reference are very much different.

```Rust
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

You are allowed to use the [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push),
[`Vec::pop`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop), and
[`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) functions.

Write tests for both functions.

## Exercise 06: Dictionary

```txt
turn-in directory:
    ex06/

files to turn in:
    sec/main.rs

allowed dependencies:

```

Create a dictionary program. It should include *some* definitions and can be used as specified in
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
>_ cargo run --
error: Yes? What do you want?
```

The defnitions can be modified, words can be added, error messages can be edited.

You must *not* use a forest of `if`/`if else` statements. The standard library contains
datastructures specifically designed to do this, and you should use them.
