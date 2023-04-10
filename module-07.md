# Module 12: Feeling Unsafe

## Forword

For the eighth day in a row, Neophyte Col found himself standing before the Two Great Guards of the
temple. They stood before the large entrance of the temple, clad in simple robes. Nevertheless, they
were imposing, and feared. He strode toward the first guard, confident, and handed the parchment
bearing his program.

The First Guard read through it carefully. This step was but a formality; yesterday, he had only
failed to gain the assent of the Second Guard. He was certain he had addressed all outstanding
complaints.

The First Guard handed the parchment back to Col. Then, in a blinding motion, slapped Col across the
face with his bare hand. In a measured voice, the First Guard spoke to him: "mismatched types:
expected &Target, found &<T as Deref>::Target", then fell silent.

Col took his parchment and retreated back to a nearby bench, close to tears. Eight days. It was not
as though his program was particularly complicated, and yet he could not convince the Two Great
Guards to permit him entry to the temple. He had not had this much difficulty at other temples!

At another bench, he saw a fellow neophyte. They had spoken two days previous, when he had learned
his compatriot had been toiling outside the temple for close to two weeks to get his program
accepted.

It was the guard's fault. Col knew his program would work as intended. All they ever seemed to do
was pick on minor errors, denying him for the most petty of reasons. They were looking for reasons
to deny him entry; that was it!

He was beginning to seethe with resentment.

Thereupon, he noticed a monk speaking to the other neophyte. The conversation seemed quite animated
when, all of a sudden, the neophyte whooped, leapt up, and rushed toward the temple. As he ran, he
seemed to be frantically modifying his program.

However, rather than face the Two Great Guards, he instead walked over to a small, dingy part of the
wall. To Col's surprise, the wall opened into what appeared to be a secret entrance. The neophyte
passed through, and was gone from sight.

Col sat stunned. A secret entrance? Then... then the Two Great Guards might merely be a prank!
Something the other monks put neophyte through to teach them humility. Or... resilience. Or... or...
perhaps it was just to laugh at them secretly.

"Do you wish to know what I said to him?" a voice asked. Col turned to see the monk standing beside
his bench. "You told him there was another way in, did you not?"

"Yes," she replied. "I told him of the unsafe door."

"unsafe?" Col asked.

"Indeed. It is a secret known to those who have studied long and hard at the temple. In truth, one
can overcome many of the obstacles posed in writing one's program through the use of the unsafe
arts, as spoken of in the [Rustonomicon](http://doc.rust-lang.org/nightly/nomicon/)."

"Are they powerful?" Col asked in wonder.

"Immensely powerful. With transmute, one can simply re-assign the type of a value, or extend the
lifetime of a pointer. One can even summon pointers from the air itself, or data from nothingness."

Col felt he finally understood how the temple worked. It was this "unsafe" magic that the monks
used! However...

"Then, why are the Two Great Guards employed if one can simply walk through the unsafe door to reach
the temple? Why not..."

At that moment, a blood-curdling scream was heard from within the temple. It echoed across the
courtyard before ending suddenly.

Silence descended. No one moved. No one spoke. The wind stilled. Even the birds halted their
singing.

Col could feel his heart pounding in his ears.

"They are there," the monk said, breaking the spell, "to protect you from the temple, and what lies
within."

Col turned to gaze once more at the hidden doorway. "Then why does that door exist?"

"Because, even they are not infallible. Some times, one must face peril alone." She sighed. "But not
all are so brave or skilled."

With that, the monk took a sheet of parchment from her cloak and walked toward the Two Great Guards.
The First Guard read her program and nodded. The Second Guard read her program, handed it back, and
then struck her across the face.

"Borrowed value does not live long enough."

The monk, rubbing her face, walked back and sat down on one of the benches, muttering curses.

_The [first Rust Koan](https://users.rust-lang.org/t/rust-koans/2408)._

## General Rules

- Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
  if the subject requires a _program_, or with `cargo test` otherwise. Only dependencies specified
  in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
  section are allowed.

- Every exercise must be part of a virtual Cargo workspace, a single `workspace.members` table must
  be declared for the whole module.

- Everything must compile _without warnings_ with the `rustc` compiler available on the school's
  machines without additional options.

- You are generally _not_ authorized to modify lint levels - either using `#\[attributes\]`,
  `#!\[global_attributes\]` or with command-line arguments. You may optionally allow the `dead_code`
  lint to silence warnings about unused variables, functions, etc.

- You are _strongly_ encouraged to write extensive tests for the functions and systems you turn in.
  Correcting an already well-tested exercise is easier and faster than having to write them during
  defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
  they are not specified in the `allowed symbols` section.

You _are_ allowed to use `unsafe` code in this module! However, some rules must be followed.

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

- `unsafe fn` means "know what you are doing before calling this function".
- `unsafe trait` means "know what you are doing before implementing this trait".
- `unsafe {}` and `unsafe impl` both mean "I know what I am doing".

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

- `ft_swap` must swaps any two values of any type. Maybe `T` can be copied; maybe not. Maybe it has
  a default value. Maybe not.
- `ft_strlen` must count the number of non-null bytes, starting at `s`. You must write an
  appropriate "# Safety" section in the documentation of that function to educate about its users
  about its correct usage.
- `ft_strcpy` must copy the null-terminated string at `src` into `dst`. Just like `ft_strlen`, you
  must _precisely_ describe the requirements of your function within a "# Safety" section in its
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

- The `transmute_iron` function must convert the given `Iron` into a bunch of `GoldNugget`s. The
  bit-pattern of the original iron _must be preserved_; ignoring byte-order.
- The `transmute_mercure` function must convert the given `Mercure` into a bunch of `GoldNugget`s.
  The bit-pattern of the original mercure _must be preserved_; ignoring byte-order.

Example:

```rust
// On a LITTLE-ENDIAN machine! On big-endian machines, the result will be different.
let iron = 0x12345678;
assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
let mercure = 0x0123456789ABCDEF;
assert_eq!(
    PhilosopherStone.transmute_mercure(mercure),
    [0xCDEF, 0x89AB, 0x4567, 0x0123],
);
```

Let's generalize a bit.

```rust
type Gold = [GoldNugget];

unsafe trait Metal {}
```

- A `Metal` is a type that may be turned into gold by the `PhilosopherStone`.
- Do not forget the `# Safety` comment in the documentation for `Metal`!

```rust
impl PhilosopherStone {
    fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold;
}
```

- The `transmute_metal` function must convert the given `metal` into `&Gold`.

Example:

```rust
let mercure: Mercure = 0x0123456789ABCDEF;
assert_eq!(
    PhilosopherStone.transmute_metal(&mercure),
    &[0xCDEF, 0x89AB, 0x4567, 0x0123],
);
```

## Exercise 02: Carton

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::alloc::{alloc, dealloc, Layout, handle_alloc_error}
    std::ops::{Deref, DerefMut}
    std::clone::Clone
    std::marker::PhantomData
    std::ptr::{NonNull, drop_in_place}
```

Create a type named `Carton<T>`, which must manage an allocation of a single `T` on the heap.

```rust
impl<T> Carton<T> {
    fn new(value: T) -> Self;
    fn into_inner(this: Self) -> T;
}
```

- You must make sure that `Carton<T>` has the correct _variance_ over `T`.
- You must make sure that the _drop checker_ makes the correct assumptions about the lifetime of
  a `T` owned by a `Carton<T>`.
- You must make sure that `Carton<T>` properly manages the memory it owns. When memory is allocated,
  it must be deallocated later!

Example:

```rust
#[derive(Clone)]
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

## Exercise 03: `Cellule<T>`

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::marker::Copy
    std::cell::UnsafeCell
    std::ptr::*  std::mem::*
```

Let's re-create our own `Cell<T>` named `Cellule<T>`.

You must implement the following inherent methods, as specified in the official documentation of
`Cell<T>`.

```rust
impl<T> Cellule<T> {
    const fn new(value: T) -> Self;

    fn set(&self, value: T);
    fn replace(&self, value: T) -> T;

    fn get(&self, value: T) -> Self;
    fn get_mut(&mut self) -> &mut T;

    fn into_inner(self) -> T;
}
```

Note that you may need to add trait bounds to some of the above methods to ensure their safety,
and once again, be extra careful of the _variance_ of your type.

You must write tests for the functions you've written.

## Exercise 04: RAII

```txt
turn-in directory:
    ex04/

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
    std::cmp::{PartialEq, Eq, PartialOrd, Ord}
    std::fmt::{Debug, Display}
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

- `last` must return the calling thread's last "errno".
- `make_last` must make an `Errno` the calling thread's last "errno".
- `description` must return a textual description of the error. Don't try to enumate _every_
  possible error! Use a function of `libc` to do it for you.

Example:

```rust
Errno(12).make_last();
assert_eq!(Errno::last(), Errno(12));

let desc = format!("{}", Errno(1));
// TODO: find what is the description of `Errno(1)`.
assert_eq!(desc, "");
```

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

- `open` must open a new file descriptor for reading (only).
- `create` must open a new file descriptor for writing (only). If the file already exists, it must
  be truncated.
- `write` must write some of the data referenced by `data` to the file descriptor.
- `read` must read some data from the file descriptor into `buffer`.
- `close` must attempt to close the file descriptor.
- In any case, errors must be handled properly.

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

- `open` and `create` work exactly the same as `Fd::open` and `Fd::create`.
- `write` and `read` work the same way as `Fd::write` and `Fd::read`. Note, however, that they only
  _borrow_ the `File`.
- `leak` must "leak" the file descriptor of the file; returning it and "forgetting" that it had to
  be closed layer.

When a `File` is dropped, it must automatically close its file descriptor.

## Exercise 05: Tableau

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::alloc::{alloc, dealloc, Layout}
    std::marker::Copy  std::clone::Clone
    std::ops::{Deref, DerefMut}
    std::ptr::*  std::mem::*
```

It must implement the following inherent methods, as specified in the official documentation:

```rust
impl<T> Tableau<T> {
    const fn new() -> Self;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;

    fn clear(&mut self);
}
```

It must be possible to do the following:

```rust
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

**Note:** Be careful! _Careful_! Any code that you didn't write can panic. Cloning a value can
panic. Dropping a value can panic. Make sure that your type do _not_ leak memory; even when cloning
or dropping.

If you're feeling like taking a challenge, you can try to write a macro to construct a `Tableau<T>`
automatically:

```rust
let v: Tableau<i32> = tableau![1, 2, 4];
assert_eq!(v, [1, 2, 4]);
```

## Exercise 06: Foreign User

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml  build.rs  awesome.c

allowed symbols:
    std::mem::MaybeUninit
    std::ffi::CStr
    std::ffi::{c_int, c_char}
```

However sad may it be, Rust is not the only programming language in existence.

Let's create a simple C library.

```c
typedef unsigned int t_id;

typedef struct {
    t_id id;
    char const *name;
} t_user;

typedef struct {
    t_id next_user_id;
    t_user *users;
    size_t count;
    size_t allocated;
} t_database;

typedef enum {
    ERR_SUCCESS,
    ERR_MEMORY,
    ERR_NO_MORE_IDS,
    ERR_UNKNOWN_ID,
} e_result;

e_result create_database(t_database *database);
void delete_database(t_database *database);

e_result create_user(t_database *database, char const *name, t_id *result);
e_result delete_user(t_database *database, t_id id);
e_result get_user(t_database const *database, t_id id, t_user const **result);
```

- `create_database` must initialize the passed `t_database` instance.
- `delete_database` must destroy the passed `t_database` instance, freeing the memory that was
  allocated.
- `create_user` must insert a new `t_user` instance in the database.
- `delete_user` must remove a `t_user` from the database.
- `get_user` must write a pointer to the user with the provided ID, if any.

In any case, on success, `ERR_SUCCESS` is returned. When a memory error occurs, `ERR_MEMORY` is
returned. When no more IDs can be allocated, `ERR_NO_MORE_IDS` is returned. When a given ID is
invalid, `ERR_UNKNOWN_ID` is returned.

You now have an awesome C library, but it's sad that you cannot use it in Rust...

Setup your project such that this C library is automatically compiled into a `.a` static library
when to call `cargo build`. Your Rust library must link against that compiled C library.

```rust
enum Error { /* ... */ }

type Id = /* ... */;

struct User { /* ... */ }

struct Database { /* ... */ }

impl Database {
    fn new() -> Self;

    fn create_user(&mut self, name: &CStr) -> Result<Id, Error>;
    fn delete_user(&mut self, id: Id) -> Result<(), Error>;
    fn get_user(&self, id: Id) -> Result<&User, Error>;
}
```

- `new` must call the `create_database` function of your C library.
- `create_user` must call `create_user`.
- `delete_user` must call `delete_user`.
- `get_user` must call `get_user`.

When a `Database` goes out of scope, it must automatically call `delete_database`.

## Exercise 07: Bare Metal

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

- You must use the `#![no_std]` and `#![no_main]` global attributes.
- If you want to make sure nothing gets into your sacred namespace, you can optioanlly add the
  `#![no_implicit_prelude]` global attribute.
- `ft_putchar` must print the specified character to the standard output of the program.
- `ft_exit` must exit the process with the specified exit code.
- Create a **program** that calls `ft_putchar` trice. Once with '4', once with '2', and once with
  `\n`.

Example:

```txt
>_ ./ft_putchar
42
>_ echo $?
42
```
