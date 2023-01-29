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

## Exercise 02: Where's My Pizza?


```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml
```

* Once a pizza has been ordered, it takes two days before the cook start working on it.
* Making a pizza takes roughly 5 days.
* Once the pizza is ready, the only delivery man must pick it up. It takes 3 days on average.
* Delivering the pizza always take a whole week.

Define the following type:

```rust
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}
```

It must have the following inherent methods.

```rust
impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self;
    fn get_delivery_time_in_days(&self) -> u32;
}
```

* `from_delivery_time` predicts the status of a pizza that was ordered `ordered_days_ago` days ago.
* `get_delivery_time_in_days` returns the estimated time before the pizza is delivered, in days.

## Exercise 03: Dry Boilerplates

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::clone::Clone  std::cmp::{PartialOrd, PartialEq}
    std::default::Default  std::fmt::Debug
```

Create a type, may it be a `struct` or an `enum`. You simply have to name it `MyType`.

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

## Exercise 04: All Over Me

```txt
turn-in directory:
    ex04/

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

## Exercise 05: Todo List

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    std::{print, println}
    std::vec::Vec::{new, push, remove, clear}
    std::string::String::as_str
    str::{to_string, parse, len, is_empty}
    ftkit::{read_line, read_number}
    std::result::Result
```

Create a simple TODO-List application.

When the user starts the program, the program asks them what to do. Available commands are the
following:

```rust
enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

impl Command {
    fn prompt() -> Self;
}
```

* The `prompt` function must ask the user to write a command. End-Of-File generates the `Quit`
command. On error, the function must simply ask the user again. See the final example for the
required format.

```rust
struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn display(&self);
    fn add(&mut self, todo: String);
    fn done(&mut self, index: usize);
    fn purge(&mut self);
}
```

* `display` must print the content of the todolist to the user.
* `add` must add an item to be done.
* `done` must mark an item as being done. Invalid indices should do nothing.
* `purge` must purge any "done" task.

Write a `main` function, respondible for using both `TodoList` and `Command`. The content of the
todolist must be displayed to the user before each prompt.

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

## Exercise 06: Lexial Analysis

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    ftkit::ARGS  std::option::Option  str::*  <[u8]>::*
```

Create a simple token parser. It must be able to take an input string, and turn it into a list
of tokens.

Each token must be represented using the following `enum`:

```rust
enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}
```

* The character `>` produces a `RedirectStdout` token.
* The character `<` produces a `RedirectStdin` token.
* The character `|` produces a `Pipe` token.
* Any other character is part of a `Word`, unless it's a whitespace.
* Whitespaces are ignored.

```rust
fn next_token(s: &mut &str) -> Option<Token>;
```

* You may need to add some *lifetime annotations* to make the function compile properly.
* The `next_token` function either produce *Some(_)* value if a token is available, or nothing when
the input string contains no tokens.

**Note:** You do not have to handle single or double quotes, nor do you have to care about escaping with `\\`!

It must be possible to use the `next_token` function and the `Token` type in this way:

```rust
fn print_all_tokens(mut s: &str) {
    while let Some(token) = next_token(&mut s) {
        println!("{token:?}");
    }
}
```

Finish your program to make it behave in this way:

```txt
>_ cargo run -- a b
error: exacltly one argument expected
>_ cargo run -- "echo hello | cat -e > file.txt"
Word("echo")
Word("hello")
Pipe
Word("cat")
Word("-e")
RedirectStdout
Word("file.txt")
```

## Exercise 07: The Game Of Life

```txt
turn-in directory:
    ex07/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    ftkit::ARGS ftkit::random_number
    std::{println, print}
    std::thread::sleep  std::time::Duration
    std::vec::Vec  std::result::Result
    std::marker::Copy  std::clone::Clone
    std::cmp::PartialEq
```

Create a **program** that plays [Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

* The board must be represented using a `struct`, and each cell with an `enum`.

```rust
enum ParseError {
    InvalidWidth(&'static str),
    InvalidHeight(&'static str'),
    InvalidPercentage(&'static str),
    TooManyArguments,
    NotEnoughArguments,
}

enum Cell {
    Dead,
    Alive,
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self;
    fn from_args() -> Result<Self, ParseError>;
    fn step(&mut self);
    fn print(&self, clear: bool);
}
```

* `new` must generate a random board of size (`width` by `height`), with approximatly
`percentage`% live cells in it.
* `from_args` must parse the command-line arguments passed to the application and use them to
create a `Board` instance. Errors are communicated through the `ParseError` enumeration.
* `step` must simulate an entier step of the simulation. We will assume that the board repeats
itself infinitely in both directions. The cell at coordinate `width + 1` is the cell at coordinate
`1`. Similarly, the cell at coordinate `-1` is the cell at coordinate `width - 1`.
* `print` must print the board to the terminal. When `clear` is `true`, the function must also
clear a previously displayed board.

**Hint:** you might want to look at *ANSI Escape Codes* if you don't know where to start!

* Finally, write a **main** function that uses above function to simulate the game of line. At each
simulation step, the previous board must be replaced by the one in the terminal.

Example:

```txt
>_ cargo run -- 20 10 40
. . . . # . . . . . . . . . . # . . . .
. . . # # . . . . . . . . . . . . . . .
. . # . . # . . # . . . . . # . . . # .
. . . . . . . . . . . . . # . . . . . #
. . . # # . . # # # # . . . . . . . . .
# . . . # . . # . . # . . . . # . . # #
. . # # # # . # # . # # . . . . . . . .
. . # . . # . . # . . . . # # . . . . .
. # # . . # # . . . . . # # . . . . . .
. # . . . # . . . . # . . . . . # . . .
^C
>_ cargo run -- 
error: not enough arguments
>_ cargo run -- a b c
error: `a` is not a valid width
```

Keep in mind that this is only an example. You may use any characters and messages you wish.
