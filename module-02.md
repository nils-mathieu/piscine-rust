# Module 02: Structs n' Co

## Introduction

Using primitive types will only get you so far. Composing those types into larger types is the way
to go! Rust makes good use of encapsulation, product and sum types, and more generally, algebraic
data types to enpower developpers into constructing powerful abstractions without losing any bit of
performance.

This module will show you how to create new types to better represent the data you manipulate. By
the end, you should have a general understanding of how types are created in Rust, and how to use
them.

## General Rules

Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise must be part of a virtual Cargo workspace, a single
`workspace.members` table must be declared for the whole module.

Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options. You are *not* allowed to use `unsafe` code anywere in your
code.

You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You must use the `#![forbid(unsafe_code)]`
attribute in every project you turn in. You may optionally allow the `dead_code` lint to silence
warnings about unused variables, functions, etc.

You are *strongly* encouraged to write extensive tests for the functions and systems you turn in.
Correcting an already well-tested exercise is easier and faster than having to write them during
defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
they are not specified in the `allowed symbols` section.

## Exercise 00: A Point In Space

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    f32::sqrt
```

Defines the following type:

```rust
struct Point {
    x: f32,
    y: f32,
}
```

Implement the following inherent functions:

* `new`, which creates a new `Point` instance with specific coordinates.
* `zero`, which creates a new `Point` at coordinates `(0, 0)`.
* `distance`, which computes the distance between two existing points.
* `translate`, which adds the vector `(dx, dy)` to the coordinates of the point.

```rust
impl Point {
    fn new(x: f32, y: f32) -> Self;
    fn zero() -> Self;
    fn distance(&self, other: &Self) -> f32;
    fn translate(&mut self, dx: f32, dy: f32);
}
```

## Exercise 01: Dimensional Analysis

```txt
turn-in directory:
    ex01/

files to turn in:
    src/main.rs  Cargo.toml
```

Copy/Past the following code and make it compile by adding type alias definitions.

```rust
fn seconds_to_minutes(seconds: Seconds) -> Minutes {
    seconds / 60.0
}

fn main() {
    let s: Seconds = 120.0;
    let m: Minutes = seconds_to_minutes(s);

    println!("{s} seconds is {m} minutes");
}
```

```txt
>_ cargo run
120 seconds is 2 minutes
```

## Exercise 02: Dry Boilerplates

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::cmp::{PartialOrd, PartialEq}
    std::default::Default  std::fmt::Debug
```

Create a `struct`. You simply have to name it `MyType`.

```rust
fn main() {
    let instance = MyType::default();

    let other_instance = instance.clone();

    println!("the default value of MyType is {instance:?}");
    println!("the clone of `instance` is {instance:#?}");
    assert_eq!(
        instance,
        MyType::default(),
        "the default value isn't always the same :/"
    );
    assert_eq!(
        instance,
        other_instance,
        "the clone isn't the same :/"
    );
    assert!(
        instance >= other_instance && instance <= other_instance,
        "why would the clone be less or greater than the original?",
    );
}
```

Copy the above `main` function and make it compile and run. You are not allowed to use the `impl`
keyword!

## Exercise 03: All Over Me

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::maker::Copy  std::cmp::PartialEq
```

```rust
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self; // optional
    fn mix(self, other: Self, opacity: u8) -> Self;
}
```

Implement the mix **function**, which must mix `self` and `other` assuming that `self` is completly
opaque and that `other` has an opacity of `(100*opacity/255)%`. We're computing "other over self".

The formula to apply on every color component `C` is the following:

```
C = Cother * opacity + Cself * (1 - opacity)
```

Kudos if you manage to perform the operation without using `f32` convertions.

It must be possible to call this function in this way:

```rust
let color = Color::new(255, 0, 0);
assert_eq!(color, color.mix(color));
```

## Exercise 04: TODO

TODO: C-like Enums

## Exercise 05: TODO

TODO: Enum with fields

## Exercise 06: Todo List

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    std::{print, println}  std::vec::Vec  std::string::String
    ftkit::{read_line, read_number}
```

Create a simple TODO-List application.

When the user starts the program, the program asks them what to do. Available commands are the
following:

* **TODO <text>** adds a task to the TODO-List.
* **DONE <index>** marks a task as "done".
* **PURGE** removes all "done" tasks from the application.
* **QUIT** or end-of-file both stop the program.

Before prompting the user for a command, registered tasks are displayed to the user.

You may design the interface you want to this exercise. Here is an example.

```txt
>_ cargo run

TODO go shopping

    0 [ ] go shopping

TODO do my homeworks

    0 [ ] go shopping
    1 [ ] do my homeworks

DONE 0

    0 [ ] do my homeworks
      [x] go shopping

PURGE

    0 [ ] do my homework

QUIT
```

## Exercise 07: Nano Cpu

```txt
turn-in directory:
    ex07/

files to turn in:
    src/main.rs src/**/*.rs  Cargo.toml

allowed symbols:
    std::{assert*, println}   <[T]>::len
```

Let's simulate a simple computer using the Rust programming language. A computer is basically made
of two things: a CPU (Central Processing Unit), and some kind of memory storage.

Create a `NanoCpu` type, which stores the state of a the simulated computer's CPU as well as its
internal RAM (it can be implemented as an array of 256 bytes). The CPU has four 8-bit registers,
**A**, **B**, **C** and **D**. A register is a value directly baked into the CPU; registers do not
have a memory address, but they can be used to reference memory stored in the RAM.

It is able to execute the following operations.

* **Copy r1 into r2:** copies the content of register **r1** into register **r2**.
* **Read r1 into r2:** reads the memory pointed by register **r1** into register **r2**.
* **Write r1 at r2:** writes the content of register **r1** at the memory pointed by register
  **r2**.
* **Set r1 to v:** sets register **r1** to the value **v**.
* **Increment r:** increments the value of register **r** by 1 (the operation must wrap on
  overflow).
* **Decrement r:** decrements the value of register **r** by 1 (the operation must wrap on
  overflow).
* **Jump to i:** makes the CPU jump to the specific instruction index **i**.
* **JumpIfZero to i:** if the register **A** has the value `0`, then makes the CPU jump to the
  specific instruction index **i**.
* **Print r** must print the content of a register to the standard output.

For example, if the register **B** contains the value `124`, using the **Read B into C**
instruction will read the value in the RAM at address `124` and put that value into register **C**.

The operations supported by the CPU must be implemented as an `enum` named `Instruction`.

A program is simply an ordered list of instructions. It executes those instructions in order,
starting from the first one in the list. The `NanoCpu` type have to implement an associated method
to execute a specific program. The function ends as soon as the instruction index reaches the end
of the instruction list. You may also add a `new` function to easily create new instances of the
type.

```rust
impl NanoCpu {
    fn new() -> Self; // optional
    fn execute(&mut self, program: &[Instruction]);
}
```

As always, you must include tests to show that the different operations do behave as they should.

Here is a program that you should be able to execute on the created computer, as long as a
null-terminated string exists at address 0x00 in the RAM of the emulated computer. You can put it
in your `main` function as an example.

```txt
00  Set B to 0       ; address of the string
01  Read B into A    ; check for the '\0' - if yes, go to 05
02  JumpIfZero to 05 ;
03  Increment B      ; '\0' not found, add 1 to B and retry
04  Jump to 01       ;
06  Print B          ; print the result
```
