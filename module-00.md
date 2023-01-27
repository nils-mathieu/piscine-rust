# Module 00: Hello, World!

## Introduction

The Rust programming language is still fairly new: its `1.0` release is younger than 42 itself! For
this reason, please understand that the knowledge acquired here may become obsolete at some point.
Though this is true for most programming languages, Rust is still updated quite regularly (almost
on a monthly basis) and you should be aware of that. Never stop learning.

At times, Rust might feel a bit hard to get into, may it be its syntax, the borrow checker, or
more generally, the number of details it requires the developer to think about when writing code.
Trust the compiler. It is always (in 99% of cases) right - even when it complains for apparently
nothing. Plus it is pretty well known to provide really good error messages. Read the errors,
correct your code, and get over it!

Internet can provide lots of resources to learn the Rust programming language, but I must advice
you to primarily focus on the official documentation. It contains the most up-to-date information
about this rapidly evolving language.

## General Rules

Any program you turn in must compile *without warnings* using the `rustc` compiler available on
the school's machines without additional options. You are *not* allowed to use `unsafe` code
anywhere in your code (not until the last module :p).

For exercises using the `cargo` package manager, the same rule applies. In that case, only the
crates specified in the `allowed dependencies` section are allowed. Any other dependency is
forbidden. More generally, only the symbols specified in `allowed symbols` are authorized within an
exercise.

You are generally *not* authorized to modify lint levels - either using `#\[attributes\]`,
`#!\[global_attributes\]` or with command-line arguments. You must use the `#![forbid(unsafe_code)]`
attribute in every project you turn in. You may optionally allow the `dead_code` lint to silence
warnings about unused variables, functions, etc.

```rust
// Either globally:
#![allow(dead_code)] 

// Or locally, for a simple item:
#[allow(dead_code)]
fn my_unused_function() {}
```

## Exercise 00: Hello, World!

```txt
turn-in directory:
    ex00/

files to turn in:
    hello.rs

allowed symbols:
    std::println
```

What's a program without side effects?

Create a **program** that prints the string `Hello, World!`, followed by a line feed.

```txt
>_ ./hello
Hello, World!
```

## Exercise 01: Point Of No Return

```txt
turn-in directory:
    ex01/

files to turn in:
    min.rs

allowed symbols:
    std::println
```

Create a `min` **function** that takes two integers, and returns the smaller one. To make the file
compile and for it to be testable, you must add a  `main` function to showing that your function is
indeed correct.

The function must be prototyped like this:

```rust
fn min(a: i32, b: i32) -> i32;
```

Oh, I almost forgot. The `return` keyword is forbidden! Good luck with that ~

## Exercise 02: yyyyyyyyyyyyyy

```txt
turn-in directory:
    ex02/

files to turn in:
    yes.rs  collatz.rs  print_bytes.rs

allowed symbols:
    std::println  str::bytes
```

Create three **functions**. Each function must use one kind of loop supported by Rust, and you
cannot use the same loop kind twice.

The functions must be prototyped as follows:

```rust
fn yes() -> !;
fn collatz(start: u32);
fn print_bytes(s: &str);
```

The `yes` function must print the message `y`, followed by a line feed. It must do it
indefinitely.

```txt
y
y
y
y
y
y
y
...
```

The `collatz` function must execute the following algorithm...

* Let *n* be any natural number.
* If *n* is even, then *n* becomes *n*/2
* If *n* is odd, then *n* becomes 3*n* + 1

...until *n* equals 1. On each iteration, *n* must be displayed on the standard output, followed
by a line feed.

```txt
Input:
3

Output:
3
10
5
16
8
4
2
1
```

The `print_bytes` function must print every byte of the provided string.

```txt
Input:
"Déjà Vu\n"

Output:
68
195
169
106
195
160
32
86
117
10
```

Once again, you must add `main` functions to prove that your functions are correct.

## Exercise 03: FizzBuzz

```txt
turn-in directory:
    ex03/

files to turn in:
    fizzbuzz.rs

allowed symbols:
    std::println
```

Create a **program** that plays the popular (and loved!) game "fizz buzz" from 1 to 100.

The rules have changed a bit, however. They must be followed in order.

* When the number is both a multiple of 3 and 5, "fizzbuzz" must be displayed.
* When the number is a multiple of 3, "fizz" must be displayed.
* When the number is a multiple of 5, "buzz" must be displayed.
* When the number is congruent to 3 modulo 11 "FIZZ" is displayed.
* When the number is congruent to 5 modulo 11 "BUZZ" is displayed.
* Otherwise, the number itself is written.

Example:

```txt
>_ ./fizzbuzz
  1: 1
  2: 2
  3: fizz
  4: 4
  5: buzz
  6: fizz
  7: 7
  8: 8
  9: fizz
 10: buzz
 11: 11
 12: fizz
 13: 13
 14: FIZZ
 15: fizzbuzz
 16: BUZZ
 17: 17
 18: fizz
 19: 19
 20: buzz
...
```

For this exercise, you can only use one `for` loop, and one `match` statement. Nothing more.

## Exercise 04: Shipping With Cargo

```txt
turn-in directory:
    ex04/

files to turn in:
    src/default.rs  str/overflow.rs  str/other.rs  Cargo.toml

allowed symbols:
    std::println
```

Create a Cargo project.

* Its name must be "module00-ex04"
* It must use Rust edition 2021
* Its author must be you.
* Its description must be "my answer to the fifth exercise of the first module of 42's Rust Piscine"
* It must not be possible to publish the package, even when using `cargo publish`.

* The following commands must give this output:

```txt
>_ cargo run
Hello, Cargo!
>_ cargo run --bin other
Hey! I'm the other bin target!
>_ cargo run --release --bin other
Hey! I'm the other bin target!
I'm in release mode!
```

* In `release` mode, the final binary must not have any visible symbols in its binary.

```txt
>_ cargo build
>_ nm <output> | head
000000000004d008 V DW.ref.rust_eh_personality
0000000000049acc r GCC_except_table0
0000000000049ad8 r GCC_except_table1
00000000000493e0 r GCC_except_table1049
00000000000493f8 r GCC_except_table1051
0000000000049410 r GCC_except_table1060
0000000000049430 r GCC_except_table1072
0000000000049440 r GCC_except_table1073
000000000004945c r GCC_except_table1075
0000000000049470 r GCC_except_table1076
>_ cargo build --release
>_ nm <output>
nm: <output>: no symbols
```

* It must have a custom profile inheritint from the "dev" profile, which disablse integer
overflow.

```txt
>_ cargo run --bin test-overflows
thread 'main' panicked at 'attempt to add with overflow', src/overflow.rs:3:5
>_ cargo run --profile no-overflows --bin test-overflows
255u8 + 1u8 == 0
```

## Exercise 05: Unit Tests

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert, assert_eq}
```

Testing a program is probably at least half the work of a developer. Every single function of any
digital system must be carefully tested to avoid as many crashes and unexpected behaviors as
possible.

```rust
fn fibs(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

fn is_prime(n: u32) -> bool {
    n >= 2 && !(2..n).any(|d| n % d == 0)
}
```

Copy the above functions, and write unit tests to determine whether those functions do work as
expected.

* The `fibs` function must compute the `n`-th term of the [fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_number).
The first two terms of the sequence are `F0 = 0` and `F1 = 1`.

* The `is_prime` function returns whether `n` is a prime number.

In case of an error, the test should panic with an appropriate error message, but you are *not*
allowed to use the `panic!` macro. Instead, you can `assert!` that a specific value has been
properly returned.

```txt
>_ cargo test
running 7 tests
test zero_is_not_prime ... ok
test one_is_not_prime ... ok
test tree_is_prime ... ok
test four_is_not_prime ... ok
test first_fib_is_0 ... ok
test fifth_fib_is_3 ... ok
test seventeenth_fib_is_987 ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

...
```

Note that the above is just an example. You are allowed to do the tests you want, but they must
properly check that those functions are actually correct.

## Exercise 06: The Price Is Right

```txt
turn-in directory:
    ex06/

files to turn in:
    src/main.rs  Cargo.toml

allowed dependencies:
    ftkit

allowed symbols:
    ftkit::*  std::println  std::cmp::Ordering  i32::cmp
```

To end this first module, why not try to create a simple game?

Create a **program** that allows its user to play the popular game show "The Price Is Right". The
game plays as follows:

1. The program randomly chooses an object whose price is unknown to the user. The name of the
object is displayed.
2. The user is prompted to bid the price of said object.
3. If the user is correct, the program ends with a message congratulating them.
4. Otherwise, a message indicates whether they overbid or underbid and the program is back to step 2.

Example:

```txt
>_ cargo run
Here is a fabulous 'cool ring'.
23
A 'cool ring' costs more than that!
57
A 'cool ring' isn't worth that much money!
34
Congrats! That 'cool ring' is worth $34.
```

To help you with that task, you are allowed to depend on the `ftkit` crate, which provides some
basic utility functions. Documentation for that library is available [here](https://docs.rs/ftkit).
