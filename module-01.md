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
machines without additional options.  You are *not* allowed to use `unsafe` code anywere in your
code.

You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You must use the `#![forbid(unsafe_code)]`
attribute in every project you turn in. You may optionally allow the `dead_code` lint to silence
warnings about unused variables, functions, etc.

You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

## Exercise 00: Creating References

```txt
turn-in directory:
    ex00/

files to turn-in:
    src/lib.rs  Cargo.toml
```

Create two **functions**. Both must add two integers together.

```rust
fn add(a: &i32, b: i32) -> i32;
fn add_assign(a: &mut i32, b: i32);
```

* `add` must return the result of the operation.
* `add_assign` must store the result of the operation in `a`.

## Exercise 01: Point Of No Return (v2)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml
```

Write a **function** that returns the smallest value among two numbers.

```rust
fn min(a: &i32, b: &i32) -> &i32;
```

Note that you may have to add some *lifetime annotations* to the function in order to make it
compile.


## Exercise 02: The Name Of Colors

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml
```

Create a **function** that maps three color components to a name.

The name of a color is determined using the following rules, applied in order. The first rule that
`match`es the input color must be selected.

* The color `[0, 0, 0]` is "pure black".
* The color `[255, 255, 255]` is "pure white".
* The color `[255, 0, 0]` is "pure red".
* The color `[0, 255, 0]` is "pure green".
* The color `[0, 0, 255]` is "pure blue".
* The color `[128, 128, 128]` is "perfect grey".
* Any color whose components are all bellow 31 is "almost black".
* Any color whose red component is above 128, whose green and blue components are between 0 and 127
is "redish".
* Any color whose green component is above 128, whose red and blue components are between 0 and 127
is "greenish".
* Any color whose blue component is above 128, whose red and green components are between 0 and 127
is "blueish".
* Any other color is named "unknown".

```rust
const fn color_name(color: &[u8; 3]) -> &str;
```

You might need to add *lifetime* annotations to the function to make it compile. Specifially, the
following test must compile and run:

```rust
#[cfg(test)]
#[test]
fn test_lifetimes() {
    static NAME_OF_THE_BEST_COLOR: &str = color_name(&[42, 42, 42]);
    assert_eq!(NAME_OF_THE_BEST_COLOR, "unknown");
}
```

## Exercise 03: Living A Long Time

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml
```

Create a **function** with this signature. It must return a reference to the integer 42.

```rust
fn lives_a_long_time() -> &'static i32;
```

Example:

```rust
assert_eq!(lives_a_long_time(), &42);
```

## Exercise 04: Find Group

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    <[u32]>::len
```

Write a **function** that returns the largest subslice of `haystack` that contains all numbers in
`needle`.

```rust
fn find_group(haystack: &[u32], needle: &[u32]) -> &[u32];
```

* When multiple groups match the `needle`, the largest one is returned.
* When multiple largest groups are found, the first one is returned.

Example:

```rust
assert_eq!(find_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
assert_eq!(find_group(&[1, 3, 4, 3, 5, 5, 4], &[5], &[5, 5]));
assert_eq!(find_group(&[1, 3, 4, 3, 5, 5, 4], &[], &[]));
assert_eq!(find_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1], &[]));
```

Once again, you may need to specify some *lifetime annotations* for the function. To check whether
your annotations are correct for that case, you can use this pre-defined `test_lifetimes` test.
It must compile and run.

```rust
#[test]
#[cfg(test)]
fn test_lifetimes() {
    let array = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = smallest_subslice(&array, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}
```

## Exercise 05: Boxes Into Boxes

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    <[i32]>::{len, swap}
```

You are given a list of boxes (`[width, height]`). Sort that list of boxes in a way for every box
to be *contained* in the previous one. If the operation is not possible, the function must panic.

```rust
fn sort_boxes(boxes: &mut [[u32; 2]]);
```

Example:

```rust
let mut boxes = [
    [3, 3],
    [4, 3],  
    [1, 0],
    [5, 7],
    [3, 3],
];

sort_boxes(&mut boxes);
assert_eq!(
    boxes,
    &[
        [5, 7],
        [4, 3],
        [3, 3],
        [3, 3],
        [1, 0],
    ],
);
```

## Exercise 06: Deduplication

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::vec::Vec::{swap_remove}
```

Write a **function** that removes all repeated elements of a list, preserving its initial ordering.

```rust
fn deduplicate(list: &mut Vec<i32>);
```

Example:

```rust
let mut v = vec![1, 2, 2, 3, 2, 4, 3];
deduplicate(&mut v);
assert_eq!(v, [1, 2, 3, 4]);
```

## Exercise 07: LIS

```txt
turn-in directory:
    ex07/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    <[i32]>::len  std::vec::Vec
```

Write a **function** that finds the Longest Increasing Sequence in a given array.

```rust
fn lis(slice: &[i32]) -> Vec<i32>;
```

* The sequence itself is returned in a list.
* The returned sequence must be *strictly* increasing.
* When multiple largest sequence are found, any of those sequences can be selected.

Example:

```rust
assert_eq!(&[2, 1, 3], [2, 3]);
assert_eq!(&[2, 1, 4, 2, 4], [1, 2, 4]);
```

## Exercise 08: HTML Tag Validator

```txt
turn-in directory:
    ex08/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    ftkit::ARGS  ftkit::read_line
    std::process::ExitCode  std::eprintln
```

Create a **program** that reads the standard input to determine whether it contains valid HTML-like
tags. As soon as an error is found, the program stops with an appropriate error message.

* When the given HTML tags are valid, the program exists with the value 0. Otherwise, the error is
displayed to the standard error output.


```txt
>_ cargo run
<Hello>
    <a>Lorem</a>
</Hello>
>_ echo $?
0
```

* Tags can only contain ASCII letters:

```txt
>_ cargo run
<He-llo>
error: line 1: invalid tag character '-'
>_ cargo run
<He
error: line 1: invalid tag character '\n'
>_ echo $?
1
```

* Every tag must have a corresponding closing tag.

```txt
>_ cargo run
<MyTag>some text
error: line 1: missing closing tag for `<MyTag>`
```

* Tags cannot be interspersed.

```txt
>_ cargo run
<a>hello<b>world</a></b>
error: line 1: '<b>' is never closed.
```
