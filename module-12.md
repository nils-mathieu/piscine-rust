# Module 12: Feeling Unsafe

## Introduction

We've come a long way, huh?

Rust is a powerful programing language, and you can be proud of yourself for coming this far. However, I want you to think twice before continuing to read the present work.

Even if this Piscine won't go as far as the [Rustnomicon](https://doc.rust-lang.org/nomicon/), one should be aware that the dark arts documented here are not to be taken lightly.

```txt
THE KNOWLEDGE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF UNLEASHING INDESCRIBABLE HORRORS THAT SHATTER YOUR PSYCHE AND SET YOUR MIND ADRIFT IN THE UNKNOWABLY INFINITE COSMOS.
```

## General Rules

Any program you turn in should compile using the `cargo` package manager, either with `cargo run` if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified in the `allowed dependencies` section are allowed.

Any program you turn in should compile *without warnings* using the `rustc` compiler available on the school's machines without additional options. You are allowed to use attributes to modify lint levels, but you must be able to explain why you did so.

You *are* allowed to use `unsafe` code in this module! However, some rules must be followed.

1. You must use the `#![forbid(unsafe_op_in_unsafe_fn)]` global attribute.

2. When an `unsafe fn` function is defined, its documentation must contain a `# Safety` section describing how to use it correctly.

```Rust
/// Returns a shared reference to one of the elements of `slice`
/// without checking whether `index` is actually a valid index.
///
/// # Safety
///
/// The provided `index` must be in bounds (i.e. `index` must be
/// strictly less than `slice.len()`).
unsafe fn get_unchecked(slice: &[u32], index: usize) -> &u32 {
    // SAFETY:
    //  - We have been given a regular `&[u32]` slice, which
    //    ensures that the pointer is valid for reads and is
    //    properly aligned. We can turn it back into a regular
    //    reference.
    //  - The caller must ensure that the `index` is in bounds,
    //    ensuring that `add()` won't overflow the memory block
    //    referenced by `slice`.
    unsafe { &*slice.as_ptr().add(index) }
}
```

3. When an `unsafe trait` trait is defined, its documentation must contain a `# Safety` section
describing how to implement it correctly.

```Rust
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

```Rust
let slice: &[u32] = &[1, 2, 3];

// SAFETY:
//  We know that `slice` has a length of 3, ensuring that accessing
//  the element at index 2 is always valid.
let val = unsafe { get_unchecked(slice, 2) };
```

5. Every time an `unsafe impl` is declared, it must be annotated with a `SAFETY:` directive.

```Rust
// SAFETY:
//  The `u64` type allows the "all-zeros" bit pattern - it corresponds
//  to the value `0u64`.
unsafe impl Zeroable for u64 {}
```

To summerize:

* `unsafe fn` means "know what you are doing before calling this function".
* `unsafe trait` means "know what you are doing before implementing this trait".
* `unsafe {}` and `unsafe impl` both mean "I know what I am doing".

## Exercise 00: `ft_swap`

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Sometimes, you need to perform an operation that the Rust compiler can't prove to be valid. When that happens, you need to use a backdoor programmed into the compiler to gently ask it to trust
us.

That is what the `unsafe` keyword is. When you write an `unsafe` block, you tell the compiler "do not worry, I know what I'm doing".

For example, in unsafe code, it's possible to create two mutable references `&mut T` to the same value. It *is* possible, but this is *invalid*. When you do that, you are triggering *undefined behavior*. The compiler always assumes that no two `&mut T` can point to the same value, so if you force that to happen, the compiler will simply behave incorrectly. In the best case, you will crash. In the worst case, the program will behave weirdly. This why we call this code "unsafe". It means "potentially incorrect".

Let's start simple: create a `ft_swap` generic function that swaps any two values of any type. This wasn't previously possible because you cannot make any assumption about `T`. Maybe it can be copied, maybe not. Maybe it has a default value, maybe not.

```Rust
fn ft_swap<T>(a: &mut T, b: &mut T);
```

You are only allowed to use [`std::ptr::read`](https://doc.rust-lang.org/std/ptr/fn.read.html) and [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).

Note that the function itself isn't unsafe. That means that you can call it safely with any parameter. However, it *uses* unsafe code to operate.

Write tests to prove the function works as expected.

## Exercise 01: `ft_strlen`

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Let's re-create the popular C function `strlen` in Rust.

```Rust
unsafe fn ft_strlen(s: *const u8) -> usize;
```

Don't forget that you have to write a `# Safety` section in the documentation for that function (as specified in general rules).

Write tests for your function!

## Exercise 02: `Carton<T>`

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Let's re-create the [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) type ourselves. Your type will be named `Carton<T>` and must have the following inherent implementation.

```Rust
impl<T> Carton<T> {
    /// Creates a new [`Carton<T>`], effectively moving
    /// `value` on the heap.
    fn new(value: T) -> Self;
}
```

The following code must work.

```Rust
struct Point { x: u32, y: u32 }

let point_in_carton = Carton::new(Point { x: 1, y: 2 });

assert_eq!(point_in_carton.x, 1);
assert_eq!(point_in_carton.y, 2);
```

Because I'm feeling generous, I'll give some pointers. You need to make sure `Carton<T>` has the correct [variance](https://doc.rust-lang.org/nomicon/subtyping.html). You need to properly inform the [drop checker](https://doc.rust-lang.org/nomicon/dropck.html) that your type owns a `T`.

You must write tests!

## Exercise 03: `Cellule<T>`

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Let's re-create our own [`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html) named `Cellule<T>`. You will need to read about the [`UnsafeCell<T>`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html) primitive.

You must implement the following inherent methods, as specified in the official documentation for [`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html):

```Rust
impl<T> Cellule<T> {
    const fn new(value: T) -> Self;

    fn set(&self, value: T);
    fn replace(&self, value: T) -> T;

    fn get(&self, value: T) -> Self;

    fn get_mut(&mut self) -> &mut T;
    fn into_inner(self) -> T;
}
```

Note that you may need to add trait bounds to *some* of the above methods to ensure their safety.

Once again, be extra careful of the [variance](https://doc.rust-lang.org/nomicon/subtyping.html) of your type.

You must write tests for the functions you've written.

## Exercise 04: `Pointeur<T>`

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

Create a type named `Pointeur<T>` that recreates the functionalities of [`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html). You write the following inherent methods as specified in the official documentation, as well as implementions for traits that make sense for the type.

```Rust
impl<T> Pointeur<T> {
    fn new(value: T) -> Self;

    fn strong_count(this: &Self) -> usize;
    fn try_unwrap(this: Self) -> Result<T, Self>;
}
```

You must not implement both a "strong" count and a "weak" count. You will assume that every existing instance of `Pointeur<T>` is a "strong" reference.

It must be possible to use the type in this way:

```Rust
let p1 = Pointeur::new(Cellule::new(1));
let p2 = p1.clone();

assert_eq!(p1.get(), 1);
assert_eq!(p2.get(), 1);

p1.set(2);

assert_eq!(p1.get(), 2);
assert_eq!(p2.get(), 2);
```

Write tests!

## Exercise 05: `Tableau<T>`

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed dependencies:

```

To finish with this module, let's re-create our own [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html). Your type will be named `Tableau<T>`.

It must implement the following inherent methods, as specified in the official documentation:

```Rust
impl<T> Tableau<T> {
    const fn new() -> Self;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;

    fn clear(&mut self);

    fn into_iter() -> IntoIter<T>;
    // You will have to write our own `IntoIter` iterator.
}
```

It must be possible to do the following:

```Rust
let mut a = Vec::new();
a.push(1); a.push(2); a.push(4);
let b = a.clone();

for it in b {
    println!("{it}");
}
// This will print:
// 1
// 2
// 4

let c: &[i32] = &*a;
assert_eq!(c, [1, 2, 4]);
```

If you're feeling like taking a challenge, you can try to write a macro to construct a `Tableau<T>` automatically:

```Rust
let v: Tableau<i32> = tableau![1, 2, 4];
assert_eq!(v, [1, 2, 4]);
```

In any case, you must write extensive tests for your type. Valgrind may be used to detect invalid operations (which would means that you have used `unsafe` incorrectly .\\/.). Be careful of `panic!`s, they can happen anytime you call a function that you didn't write.
