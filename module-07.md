# Module 12: Feeling Unsafe

## Introduction

We've come a long way, huh?

Rust is a powerful programing language, and you can be proud of yourself for coming this far.
However, I want you to think twice before continuing to read the present work. One should be aware
that the dark arts documented here are not to be taken lightly.

```txt
THE KNOWLEDGE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF UNLEASHING INDESCRIBABLE HORRORS THAT SHATTER YOUR PSYCHE AND
SET YOUR MIND ADRIFT IN THE UNKNOWABLY INFINITE COSMOS.
```

## General Rules

* Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed.

* Every exercise must be part of a virtual Cargo workspace, a single `workspace.members` table must
be declared for the whole module.

* Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options.

* You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You may optionally allow the `dead_code`
lint to silence warnings about unused variables, functions, etc.

* You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

You *are* allowed to use `unsafe` code in this module! However, some rules must be followed.

1. You must use the `#![forbid(unsafe_op_in_unsafe_fn)]` global attribute.

2. When an `unsafe fn` function is defined, its documentation must contain a `# Safety` section
describing how to use it correctly.

```rust
/// Returns one of the elements of `slice`, as specified by
/// `index`, but without checking whether `index` is actually
/// in bounds.
///
/// # Safety
///
/// The provided `index` must be in bounds (i.e. it must be
/// strictly less than `slice.len()`).
unsafe fn get_unchecked(slice: &[u32], index: usize) -> u32 {
    // SAFETY:
    //  - We have been given a regular `&[u32]` slice, which
    //    ensures that the pointer is valid for reads and is
    //    properly aligned. We can turn it back into a regular
    //    reference.
    //  - The caller must ensure that the `index` is in bounds,
    //    ensuring that `add()` won't overflow the memory block
    //    referenced by `slice`.
    unsafe { *slice.as_ptr().add(index) }
}
```

3. When an `unsafe trait` trait is defined, its documentation must contain a `# Safety` section
describing how to implement it correctly.

```rust
/// Types that can be initialized to zeros.
///
/// # Safety
///
/// Implementators of this trait must allow the "all-zeros" bit pattern.
unsafe trait Zeroable {
    fn zeroed() -> Self {
        // SAFETY:
        //  Implementators of the `Zeroable` trait can be initialized
        //  with the "all-zeros" bit pattern, ensuring that calling
        //  this function won't produce an invalid value.
        unsafe { std::mem::zeroed() }
    }
}
```

4. Every time an `unsafe` block is used, it must be annotated with a `SAFETY:` directive.

```rust
let slice: &[u32] = &[1, 2, 3];
// SAFETY:
//  We know that `slice` has a length of 3, ensuring that accessing
//  the element at index 2 is always valid.
let val = unsafe { get_unchecked(slice, 2) };
```

5. Every time an `unsafe impl` is declared, it must be annotated with a `SAFETY:` directive.

```rust
// SAFETY:
//  The `u64` type allows the "all-zeros" bit pattern - it corresponds
//  to the value `0u64`.
unsafe impl Zeroable for u64 {}
```

To summarise:

* `unsafe fn` means "know what you are doing before calling this function".
* `unsafe trait` means "know what you are doing before implementing this trait".
* `unsafe {}` and `unsafe impl` both mean "I know what I am doing".

## Exercise 00: Libft

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::ptr::{write, read, add}
```

Let's start simple.

```rust
fn ft_swap<T>(a: &mut T, b: &mut T);
unsafe fn ft_strlen(s: *const u8) -> usize;
```

* `ft_swap` must swaps any two values of any type. Maybe `T` can be copied; maybe not. Maybe it has
a default value. Maybe not.
* `ft_strlen` must count the number of non-null bytes, starting at `s`. You must write an
appropriate "# Safety" section in the documentation of that function to educate its users about its
correct usage.

Example:

```rust
let mut a = String::from("Hello, World!");
let mut b = String::from("Goodby, World!");
ft_swap(&mut a, &mut b);
assert_eq!(a, "Goodby, World!");
assert_eq!(b, "Hello, World!");

let s = b"Hello, World!\0;
// SAFETY:
//  /* ... */
let len = unsafe { ft_strlen(s.as_ptr()) };
assert_eq!(len, 13);
```

## Exercise 01:

TODO:

## Exercise 02: Carton

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::alloc::{alloc, dealloc, Layout}
    std::ops::{Deref, DerefMut}
```

Create a type named `Carton<T>`, which must manage an allocation of a single `T` on the heap.

```rust
impl<T> Carton<T> {
    fn new(value: T) -> Self;
    fn into_inner(this: Self) -> T;
}
```

The following code must work.

```rust
struct Point { x: u32, y: u32 }
let point_in_carton = Carton::new(Point { x: 1, y: 2 });
assert_eq!(point_in_carton.x, 1);
assert_eq!(point_in_carton.y, 2);
```

Because I'm feeling generous, I'll give some pointers. You need to make sure `Carton<T>` has the
correct *variance* over `T`. You also need to properly inform the *drop checker* that your type owns
a `T`.

## Exercise 03:

TODO: FFI and build.rs
structs #[repr(C)]

## Exercise 04:

TODO: libc (create safe wrapper for some functions here?) Oh! A wrapper for file descriptors would
be cool! Plus a bonus for errno turned into a result.

## Exercise 05:

TODO:

## Exercise 06:

TODO:

## Exercise 07: `ft_putchar`

```txt
turn-in directory:
    ex07/

files to turn in:
    ft_putchar.rs
```

What better way to finish this great journey but by writing your very first `C` function?

```rust
fn ft_putchar(c: u8);
```

You can't use any function or symbol from the standard library apart from the `std::arch::asm!`
macro. If you want to make sure nothing gets into your sacred namespace, you can use the
`#![no_implicit_prelude]` global attribute.

You can optionally provide a `main` function to showcase your work of art.

TODO: maybe this exercise shouldn't be the last one. It is not that complicated. Maybe? Not sure.
