# Module 03: Polymorphine

## Forword

```rust
// Bastion of the Turbofish
// ------------------------
// Beware travellers, lest you venture into waters callous and unforgiving,
// where hope must be abandoned, ere it is cruelly torn from you. For here
// stands the bastion of the Turbofish: an impenetrable fortress holding
// unshaking against those who would dare suggest the supererogation of the
// Turbofish.
//
// Once I was young and foolish and had the impudence to imagine that I could
// shake free from the coils by which that creature had us tightly bound. I
// dared to suggest that there was a better way: a brighter future, in which
// Rustaceans both new and old could be rid of that vile beast. But alas! In
// my foolhardiness my ignorance was unveiled and my dreams were dashed
// unforgivingly against the rock of syntactic ambiguity.
//
// This humble program, small and insignificant though it might seem,
// demonstrates that to which we had previously cast a blind eye: an ambiguity
// in permitting generic arguments to be provided without the consent of the
// Great Turbofish. Should you be so naïve as to try to revolt against its
// mighty clutches, here shall its wrath be indomitably displayed. This
// program must pass for all eternity, fundamentally at odds with an impetuous
// rebellion against the Turbofish.
//
// My heart aches in sorrow, for I know I am defeated. Let this be a warning
// to all those who come after. Here stands the bastion of the Turbofish.

fn main() {
    let (oh, woe, is, me) = ("the", "Turbofish", "remains", "undefeated");
    let _: (bool, bool) = (oh<woe, is>(me));
}
```

*Extracted from `rustc`'s [unit tests](https://github.com/rust-lang/rust/blob/79d8a0fcefa5134db2a94739b1d18daa01fc6e9f/src/test/ui/bastion-of-the-turbofish.rs),
in memory of Anna Harren.*

## General Rules

* Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed.

* Every exercise must be part of a virtual Cargo workspace, a single `workspace.members` table must
be declared for the whole module.

* Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options.  You are *not* allowed to use `unsafe` code anywere in your
code.

* You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You may optionally allow the `dead_code`
lint to silence warnings about unused variables, functions, etc.

* You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

## Exercise 00: `choose`

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    <[T]>::len  std::println  ftkit::random_number
```

Create a **function** that randomly chooses a value among an input slice. If the provided list is
empty, the function is allowed to panic.

```rust
fn choose<T>(values: &[T]) -> &T;
```

You can write a `main` function to show that the function works as expected.

## Exercise 01: Point Of No Return (v3)

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cmp::PartialOrd  std::{assert*}
    std::marker::Sized
```

Again? Yes. Another `min` function! But I promise, this one's the last one.

* Create a `min` function that takes *any* two values of a type that supports the `<` operator, and
returns the smaller one.

Example:

```rust
assert_eq!(min(12i32, -14i32), -14);
assert_eq!(min(12f32, 14f32), 12f32);
assert_eq!(min("abc", "def"), "abc");
assert_eq!(min(String::from("abc"), String::from("def")), "abc");
```

Still not allowed to use `return`!

## Exercise 02: Oooooh... So, that's how it works!

```txt
turn-in directory:
    ex02/

file to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::fmt::{Display, Debug, Binary, Formatter, Write}
    std::write
    std::result::Result
```

Create a type named `John` and implement the right such that executing the following code...

```rust
fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
```

...produces this output.

```txt
>_ cargo run
1. Hey! I'm John.
2. |Hey! I'm John.                |
3. |                Hey! I'm John.|
4. Hey! I
5. Don't try to silence me!
6. John, the man himself.
7. John, the man himself. He's handsome AND formidable.
8. Bip Boop?
```

## Exercise 03: 42

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::fmt::Debug  std::println
```

Define the following trait:

```rust
trait FortyTwo {
    fn forty_two() -> Self;
}
```

* The `forty_two` associated function must return an instance of the implementator that represents
the number 42 in some way.

Implement this trait for some common types, at least `u32` and `String`.

```rust
fn print_forty_two<T: Debug + FortyTwo>();
```

* The `print_forty_two` function must create an instance of `T` using the `FortyTwo` trait, and then
print it to the standard output using its `Debug` implementation.

Create a `main` function that showcase this function being called for at least two distinct types.

## Exercise 04: What Time Is It?

```txt
turn-in directory:
    ex04/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::str::FromStr  std::fmt::{Display, Debug, Formatter}
    str::as_bytes  std::result::Result  std::{write, println}
    u8::is_ascii_digit
```

Create a type named `Time` responsible for storing, well, a time.

```rust
struct Time {
    hours: u32,
    minutes: u32,
}

enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}
```

Implement the right traits such that the provided `main` function compiles and produces the given
output.

```rust
fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
```

Output:

```txt
>_ cargo run
12 hours, 20 minutes
15 hours, 14 minutes
error: missing ':'
error: invalid length
error: invalid number
```

## Exercise 05: Quick Math

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::cmp::{PartialEq, Eq}  std::fmt::Debug
    std::ops::{Add, Sub, AddAssign, SubAssign}
    std::ops::{Mul, MullAssign, Div, DivAssign}
    std::{assert*} std::marker::Copy
    std::clone::Clone  f32::sqrt f64::sqrt
```

```rust
struct Vector<T> {
    x: T,
    y: Y,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self;
}
```

* The `new` function must create a new `Vector<T>` with the specified components.
* Overload the `+`, `-`, `+=` and `-=` operators for `Vector<T>`, for any `T` that itself has
support for those operators.
* Overload the `*`, `*=`, `/` and `/=` operators for `Vector<T>`, for any `T` that itself has support
for those operators. The second operand of those operations *must not* be `Vector<T>`, but `T`
itself, meaning that you must be able to compute `Vector::new(1, 2) * 3` but not
`Vector::new(1, 2) * Vector::new(2, 3)`. You can require `T: Copy` when needed.
* Overload the `==` and `!=` operators for any `T` that supports them.
* Implement specifically for both `Vector<f32>` and `Vector<f64>` a `length` function that computes
its length. The length of a vector can be computed using this formula: `‖(x, y)‖ = sqrt(x² + y²)`.

The following tests must compile and run properly:

```rust
#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}
```

## Exercise 06: A Singly-Linked List

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::boxed::Box  std::option::Option
    std::panic
```

* Create a linked list type named `List<T>` defined as follows.

```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>
}

impl<T> List<T> {
    fn new() -> Self;

    fn push_front(&mut self, value: T);
    fn push_back(&mut self, value: T);

    fn count(&self) -> usize;

    fn get(&self, i: usize) -> Option<&T>;
    fn get_mut(&mut self, i: usize) -> Option<&mut T>;

    fn remove_front(&mut self) -> Option<T>;
    fn remove_back(&mut self) -> Option<T>;
    fn clear(&mut self);
}
```

* `new` must create an empty list.
* `push_back` must append an element to the list.
* `push_front` must prepend an element to the list.
* `count` must return the number of elements present in the list.
* `get` must return a shared reference to the element at index `i`.
* `get_mut` must return an exclusive reference to the element at index `i`.
* `remove_back` must remove the last element of the list and return it.
* `remove_front` must remove the first element of the list and return it.
* `clear` must remove all elements of the list, leaving it empty.

The following tests must compile and pass.

```rust
#[cfg(test)]
#[test]
fn default_list_is_empty() {
    let list: List<i32> = Default::default();
    assert_eq!(list.count(), 0);
}

#[cfg(test)]
#[test]
fn cloned_list_are_equal() {
    let mut list = List::new();
    list.push_back(String::from("Hello"));
    list.push_back(String::from("World"));

    let cloned = list.clone();
    assert_eq!(cloned.count(), list.count());
    assert_eq!(&cloned[0], &cloned[0]);
    assert_eq!(&cloned[1], &cloned[1]);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "tried to access out of bound index 10")]
fn out_of_bound_access_panics() {
    let mut list: List<u32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list[10], 42);
}
```

## Exercise 07: Comma-Separated Values

```txt
turn-in directory:
    ex07/

files to turn in:
    src/lib.rs  src/*.rs  Cargo.toml

allowed symbols:
    str::{split, to_string, lines}  std::result::Result
    std::vec::Vec  std::string::String  std::write
    std::fmt::{Debug, Display, Formatter, Write}
    std::cmp::PartialEq  std::marker::Sized
```

Let's create a generic CSV Encoder & Decoder. A CSV file is defined like this:

```txt
value1,value1,value1,value1
value2,value2,value2,value2
value3,value3,value3,value3
...
```

Each line corresponds to a *record*, and each column corresponds to a *field*.

* Create a `Field` trait, which describes how to encode or decode a value.

```rust
struct EncodingError;
struct DecodingError;

trait Field: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(field: &str) -> Result<Self, DecodingError>;
}
```

* Implement the `Field` trait for `String`. Keep in mind that finding a ',' or a '\n' in the string
is an `EncodingError`!
* Implement the `Field` trait for `Option<T>` as long as `T` implements `Field` too. The empty
string maps to `None`, while a non-empty string maps to the `Field` implementation of `T`.
* Implement the `Field` trait for *every possible integer type*. Because this is long, repetitive
and borring, write a *macro* to do it for you.

```rust
// ez
impl_field_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
```

* Create a `Record` trait, which describes how to encode or decode a collection of `Field`s.

```rust
trait Record: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(line: &str) -> Result<Self, DecodingError>; 
}
```

* Now, you have everything you need to create `decode_csv` and `encode_csv` functions.

```rust
fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError>;
fn decode_csv<R: Record>(contents: &str) -> Result<Vec<R>, DecodingError>;
```

* `encode_csv` takes a list of records and encode them into a `String`.
* `decode_csv` takes the content of a CSV file and decodes it into a list of records.

Example:

```rust
#[cfg(test)]
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[cfg(test)]
impl Record for User { /* ... */ }

#[cfg(test)]
#[test]
fn test_encode() {
    let database = [
        User { name: "aaa".into(), age : 23 },
        User { name: "bb".into(), age: 2 },
    ];

    let csv = encode_csv(&database).unwrap();

    assert_eq!(
        csv,
        "\
        aaa,23\n\
        bb,2\n\
        "
    );
}

#[cfg(test)]
#[test]
fn test_decode() {
    let csv = "\
        hello,2\n\
        yes,5\n\
        no,100\n\
    ";

    let database: Vec<User> = decode_csv(csv).unwrap();

    assert_eq!(
        database,
        [
            User { name: "hello".into(), age: 2 },
            User { name: "yes".into(), age: 5 },
            User { name: "no".into(), age: 100 },
        ]
    );
}

#[cfg(test)]
#[test]
fn decoding_error() {
    let csv = "\
        hello,2\n\
        yes,6\n\
        no,23,hello\n\
    ";

    decode_csv::<User>(csv).unwrap_err();
}

```

You might have noticed that implementing the `Record` trait is *very* repetitive. As a bonus (a
bonus to the bonus if you will), you can create an `impl_record!` macro to implement it in a single
line:

```rust
struct MyType {
    id: u32,
    name: String,
}

// ez
impl_record!(MyType(id, name));

#[cfg(test)]
#[test]
fn test_impl_record() {
    let records = [
        MyType { id: 10, name: "Marvin".into() },
        MyType { id: 11, name: "Marvin".into() },
        MyType { id: 12, name: "Marvin".into() },
    ];

    let csv = encode_csv(&records).unwrap();
    assert_eq!(
        csv,
        "\
        10,Marvin\n\
        11,Marvin\n\
        12,Marvin\n\
        "
    );
}
```
