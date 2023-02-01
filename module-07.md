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
unsafe fn ft_strcpy(dst: *mut u8, src: *const u8);
```

* `ft_swap` must swaps any two values of any type. Maybe `T` can be copied; maybe not. Maybe it has
a default value. Maybe not.
* `ft_strlen` must count the number of non-null bytes, starting at `s`. You must write an
appropriate "# Safety" section in the documentation of that function to educate about its users
about its correct usage.
* `ft_strcpy` must copy the null-terminated string at `src` into `dst`. Just like `ft_strlen`, you
must *precisely* describe the requirements of your function within a "# Safety" section in its
documentation.

Example:

```rust
let mut a = String::from("Hello, World!");
let mut b = String::from("Goodby, World!");
ft_swap(&mut a, &mut b);
assert_eq!(a, "Goodby, World!");
assert_eq!(b, "Hello, World!");

let s = b"Hello, World!\0";
// SAFETY:
//  /* ... */
let len = unsafe { ft_strlen(s.as_ptr()) };
assert_eq!(len, 13);

let mut dst = [0u8; 14];
// SAFETY:
//  /* ... */
unsafe { ft_strcpy(dst.as_mut_ptr(), s.as_ptr()) };
assert_eq!(s, b"Hello, World\0");
```

## Exercise 01: Philospher's Stone

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::slice::from_raw_parts
    std::mem::transmute
```

Let's play the


```rust
type GoldNugget = u16;

type Iron = u32;
type Mercure = u64;

struct PhilosopherStone;

impl PhilosopherStone {
    fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2];
    fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4];
}
```

* The `transmute_iron` function must convert the given `Iron` into a bunch of `GoldNugget`s. The
bit-pattern of the original iron *must be preserved*; ignoring byte-order.
* The `transmute_mercure` function must convert the given `Mercure` into a bunch of `GoldNugget`s.
The bit-pattern of the original mercure *must be preserved*; ignoring byte-order.

Example:

```rust
// On a LITTLE-ENDIAN machine! On big-endian machines, the result will be different.
let iron = 0x12345678;
assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
let mercure = 0x123456780ABCDEF;
assert_eq1(PhilosopherStone.transmute_mercure(mercure), [0xCDEF, 0x80AB, 0x5678, 0x1234]);
```

Let's generalize a bit.

```rust
type Gold = [u16];

unsafe trait Metal {}
```

* A `Metal` is a type that may be turned into gold by the `PhilosopherStone`.
* Do not forget the `# Safety` comment in the documentation for `Metal`!

```rust
impl PhilosopherStone {
    fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold;
}
```

* The `transmute_metal` function must convert the given `metal` into `&Gold`.

Example:

```rust
unsafe impl Metal for GoldNugget {}
let nugget = 0x1233;
assert_eq!(PhilosopherStone.transmute_metal(&nugget), &[0x3312]);
```

## Exercise 02:

TODO:

## Exercise 03: Carton

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::alloc::{alloc, dealloc, Layout}
    std::ops::{Deref, DerefMut}
    std::clone::Clone
```

Create a type named `Carton<T>`, which must manage an allocation of a single `T` on the heap.

```rust
impl<T> Carton<T> {
    fn new(value: T) -> Self;
    fn into_inner(this: Self) -> T;
}
```

* You must make sure that `Carton<T>` has the correct *variance* over `T`.
* You muts make sure that the *drop checker* makes the correct assumptions about the lifetime of
a `T` owned by a `Carton<T>`.
* You must make sure that `Carton<T>` properly manages the memory it owns. When memory is allocated,
it must be deallocated layer!
* Be careful! *Careful*! Any code that you didn't write can panic. Cloning a value can panic.
Dropping a value can panic. Make sure that your type do *not* leak memory; even when cloning or
dropping.

Example:

```rust
#[derive(Clone, Copy)]
struct Point { x: u32, y: u32 }
let point_in_carton = Carton::new(Point { x: 1, y: 2 });
assert_eq!(point_in_carton.x, 1);
assert_eq!(point_in_carton.y, 2);

let mut another_point = point_in_carton.clone();
another_point.x = 2;
another_point.y = 3;
assert_eq!(another_point.x, 2);
assert_eq!(another_point.y, 3);
assert_eq!(point_in_carton.x, 1);
assert_eq!(point_in_carton.y, 2);
```

## Exercise 04:

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml  build.rs  awesome.c

allowed symbols:
```

However sad may it be, Rust is not the only programming language in existence.

TODO: FFI and build.rs
structs #[repr(C)]
union

## Exercise 05: RAII

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    libc  cstr

allowed symbols:
    std::copy::Copy  std::clone::Clone
    std::str::from_utf8_unchecked
    libc::__errno_location
    libc::strerror
    libc::{write, read, open, close}
    cstr::cstr
```

Create an `Errno` type, respondible for managing errors coming from C code.

```rust
struct Errno(libc::c_int);

impl Errno {
    fn last() -> Self;
    fn make_last(self);
    fn description(self) -> &'static str;
}
```

* `last` must return the calling thread's last "errno".
* `make_last` must make an `Errno` the calling thread's last "errno".
* `description` must return a textual description of the error. Don't try to enumate *every*
possible error! Use a function of `libc` to do it for you.

TODO: add an example that show that `Debug` and `Display` must be implemented.

With a robust way to handle errors, we can no start for real.

```rust
struct Fd(libc::c_int);

impl Fd {
    const STDIN: Self = /* ... */;
    const STDOUT: Self = /* ... */;
    const STDERR: Self = /* ... */;

    fn open(file: &CStr) -> Result<Self, Errno>;
    fn create(file: &CStr) -> Result<Self, Errno>;
    fn write(self, data: &[u8]) -> Result<usize, Errno>;
    fn read(self, buffer: &mut [u8]) -> Result<usize, Errno>;
    fn close(self) -> Result<(), Errno>;
}
```

* `open` must open a new file descriptor for reading (only).
* `create` must open a new file descriptor for writing (only). If the file already exists, it must
  be truncated.
* `write` must write some of the data referenced by `data` to the file descriptor.
* `read` must read some data from the file descriptor into `buffer`.
* `close` must attempt to close the file descriptor.
* In any case, errors must be handled properly.

That's cool, and all. But we can do better!

```rust
struct File(Fd);

impl File {
    fn open(file: &CStr) -> Result<Self, Errno>;
    fn create(file: &CStr) -> Result<Self, Errno>;
    fn write(&self, data: &[u8]) -> Result<usize, Errno>;
    fn read(&self, buffer: &mut [u8]) -> Result<usize, Errno>;
    fn leak(self) -> Fd;
}
```

* `open` and `create` work exactly the same as `Fd::open` and `Fd::create`.
* `write` and `read` work the same way as `Fd::write` and `Fd::read`. Note, however, that they only
  *borrow* the `File`.
* `leak` must "leak" the file descriptor of the file; returning it and "forgetting" that it had to
  be closed layer.

When a `File` is dropped, it must automatically close its file descriptor.

## Exercise 06: NonoGrind

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml
```

Create a **program** that spawns another program and verifies that any memory it allocates using
`mmap` is properly freed with an associated `munmap` later down the line. Memory errors are
displayed neatly when the "slave" (real GDB terminology) exits or crashes.

Example:

```txt
TODO: add an example
```

TODO: determine whether this is actually doable or not.

## Exercise 07: Never Needed Rust

```txt
turn-in directory:
    ex07/

files to turn in:
    ft_putchar.rs

allowed symbols:
    core::arch::asm
```

What better way to finish this great journey but by writing your very first `C` function?

```rust
fn ft_putchar(c: u8);
fn ft_exit(code: u8) -> !;
```

* You must use the `#![no_std]` and `#![no_main]` global attributes.
* If you want to make sure nothing gets into your sacred namespace, you can optioanlly add the
  `#![no_implicit_prelude]` global attribute.
* `ft_putchar` must print the specified character to the standard output of the program.
* `ft_exit` must exit the process with the specified exit code.
* Create a **program** that calls `ft_putchar` trice. Once with '4', once with '2', and once with
  `\n`.

Example:

```txt
>_ ./ft_putchar
42
>_ echo $?
42
```
