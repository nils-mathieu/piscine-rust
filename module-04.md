# Module 04: Higher Order

## Forword

Neophyte Col came to Master Tuor one day and said: "I have almost finished a program in which I see
repeated patterns of code. The patterns are not regular enough to be abstracted by functions or
traits. I have a mind to use macros, but am unsure of how to proceed."

Master Tuor considered this. "I believe I know who can help you find clarity. Come."

Together, they walked away from the main temple and past the gardens. On the periphery of the
grounds, they came across a strange building. Each plank of wood, each panel, tile, window; all
were unique in shape, size, colour, texture, material... "What is this place?" asked the neophyte.

"It is where the Clan of the Falling Snowflake practice the art of Syntactic Abstraction. They
abhor repetition above all else." Col nodded as they entered.

Inside, the building was just as relentlessly unique. No two doors were the same shape or size.
Each candle and its perch were designed and coloured a different way. Col began to feel dizzy.

Even the monks themselves exemplified this approach. Rather than the plain, dark brown robes the
other monks wore, each monk of the Falling Snowflake wore a haphazard collection of materials and
colours. Col wondered if perhaps a great storm had swept up a cloth merchant's wares and deposited
them on these unsuspecting monks.

His eyes were then caught by the walls. They were covered with programs that seemed to be in some
strange language as he might expect from the Temple of APL... but here and there he caught glimpses
of familiar syntax, though they made little sense.

The monks' speech was strange as well. Col observed several groups standing silently, staring at
each other. He began to wonder if perhaps they were forbidden to speak at all, when he heard an
outburst from one group:

"Sie durm! Wik puom, eeis."

With that, the monks standing in the group nodded at one another and walked away.

"Master..." Col began.

"As I said, they abhor repetition. As such, they abhor our language, filled as it is with
redundancy, not to mention inaccuracy. Instead, they use their own tongue which is both perfectly
precise, and perfectly optimal. They say what they mean and nothing else."

"But they are so quiet, otherwise."

Master Tuor nodded. "Do you think it easy to decide what one wishes to say, with utter and complete
precision, before speaking a word?"

As they passed through a tiny, five-sided doorway, they came upon a monk dressed in cloth dyed in
a thousand shades of dark grey. Wrapped around the robes was a mesmerising pattern in white thread
that seemed to spiral forever away into the cloth; an infinite, curving beast made of straight
lines and squares.

"Neophyte, this is Master Quoem, head of the Clan of the Falling Snowflake." The monk nodded in
greeting. "Tell him of your problem."

Col took a breath. "I have nearly completed a program in which I see much repetition.

"In some cases, the logic must call different functions, though all around it remains unchanged. In
others, different types implement similar methods, yet they cannot be unified by traits.

"I have been told that macros may be a solution to my problem. What would you advise?"

Master Quoem was silent. For many long minutes, he stared intently at Col. It went on for so long
that Col began to shuffle nervously under the master's gaze, uncertain if he had somehow offended
him.

The master, Col noticed, had slightly odd eyebrows...

Finally, Master Quoem spoke thus:

"Fheiq kah, puom."

Then he nodded at Col, turned, and walked away.

"Thank you for your counsel, Master Quoem," said Master Tuor as he guided Col back the way they
came.

As they walked, Col tried to understand what had happened. Stepping back out into the sunlight, he
came to a conclusion.

"Did speaking to Master Quoem help you find your answer?" Master Tuor asked him.

"Yes," replied Col. "I have finished my program."

Master Tuor nodded; the neophyte had been enlightened.

_The [fourth Rust Koan](https://users.rust-lang.org/t/rust-koans/2408/4)._

## General Rules

- Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
  if the subject requires a _program_, or with `cargo test` otherwise. Only dependencies specified
  in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
  section are allowed.

- Every exercise must be part of a virtual Cargo workspace, a single `workspace.members` table must
  be declared for the whole module.

- Everything must compile _without warnings_ with the `rustc` compiler available on the school's
  machines without additional options. You are _not_ allowed to use `unsafe` code anywere in your
  code.

- You are generally _not_ authorized to modify lint levels - either using `#[attributes]`,
  `#![global_attributes]` or with command-line arguments. You may optionally allow the `dead_code`
  lint to silence warnings about unused variables, functions, etc.

- You are _strongly_ encouraged to write extensive tests for the functions and systems you turn in.
  Correcting an already well-tested exercise is easier and faster than having to write them during
  defense. Tests (when not specifically required by the subject) can use the symbols you want, even if
  they are not specified in the `allowed symbols` section.

## Exercise 00: Print All Things

```txt
turn-in directory:
    ex00/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::ops::Range
    std::iter::{IntoIterator, Iterator}
    std::fmt::Debug
    std::{print, println}
```

Create a **function** with the following signature.

```rust
fn print_all_things<I>(i: I)
where
    /* ... */;
```

- The `print_all_things` function must print the values it is given.
- You have to add appropriate trait bounds to `I`.
- Yo must use exactly _one_ for loop.

Example:

```rust
fn main() {
    print_all_things(0..=5);
    print_all_things("Hello".chars());
    print_all_things(vec![1, 3, 4, 2]);
    print_all_things([1, 2, 5, 4]);
}
```

```txt
>_ cargo run
[ 0 1 2 3 4 5 ]
[ 'H' 'e' 'l' 'l' 'o' ]
[ 1 3 4 2 ]
[ 1 2 5 4 ]
```

## Exercise 01: YYYYYYYYYYYYYY

```txt
turn-in directory:
    ex01/

files to turn in:
    src/yes.rs  src/collatz.rs  src/print_bytes.rs  Cargo.toml

allowed symbols:
    std::ops::{Fn, FnMut, FnOnce}
    str::chars
```

Create tree **functions**. Each function must be prototyped as follows.

```rust
fn collayz<F: /* ... */>(start: u32, f: F);
fn yes<F: /* ... */>(f: F) -> !;
fn print_byes<F: /* ... */>(f: F);
```

- The `collayz` function must call the `f` function on every new odd value in the [collatz sequence](https://en.wikipedia.org/wiki/Collatz_conjecture).
- You must add an appropriate bound for the `F` generic type so that you can call it with a single
  `u32` parameter (i.e. `f(n: u32);`).
- Create a `main` that calls `collatz` with `start = 11` and produces the following output:

```txt
>_ cargo run --bin collatz
YYYYYYYYYYY
YYYYYYYYYYYYYYYYY
YYYYYYYYYYYYY
YYYYY
Y
```

- The `print_byes` function must call `f` in repeat until it returns `None` (rather
  than a `Some(u8)`). Each time, `print_bytes` prints the returned byte in binary, but zeros are
  replaced by 'Y's, and ones with `y`.
- You must add an appropriate bound to the `F` generic type, such that it returns an `Option<u8>`
  and takes no parameters (i.e. `let ret: Option<u8> = f();`).
- Create a `main` that calls the `print_bytes` function in a way that it produces the following
  output (the bytes are those of `"Hello, World!"`).

Example:

```txt
>_ cargo run --bin yes
YyYYyYYY
YyyYYyYy
YyyYyyYY
YyyYyyYY
YyyYyyyy
YYyYyyYY
YYyYYYYY
YyYyYyyy
YyyYyyyy
YyyyYYyY
YyyYyyYY
YyyYYyYY
YYyYYYYy
```

- The `yes` function must call the `f` function once, and print its return value in an infinite
  loop.
- You must add an appropriate bound to the `F` generic type, such that it returns a `String` and
  that it can be called with no parameters (i.e. `let s: String = f();`).
- Create a `main` function that calls `yes` and produces the following output.

```txt
>_ cargo run --bin yes
YyY
YyY
YyY
YyY
YyY
YyY
YyY
YyY
...
```

You cannot use twice the same bound type on `F`!

## Exercise 02: One Liners

```txt
turn-in directory:
    ex02/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::iter::Iterator::{filter_map, map, filter, sum, collect}
    str::{split_whitespaces, trim, split_once}
    std::str::FromStr
```

Create a **function** that does the following:

1. Take a string as input.
2. Split the string on whitespaces.
3. Parse each word into an `u32`. If it is not possible, the word is ignored.
4. Only keep odd numbers.
5. Returns the sum of the remaining numbers.

```rust
fn sum_of_odds(s: &str) -> u32;
```

Example:

```rust
let sum = sum_of_odds("hey 20 test 3\t9 4 5, 1 hello");
assert_eq!(sum, 13);
```

Let's create a second one. It must:

1. Take a string as input.
2. For each line of the input, split the line into two fields, separated by a colon `':'`. If the
   line contains no `:`, it is ignored.
3. The first part is kept as a string reference, but starting and ending whitespaces are trimmed.
   The second part is turned into any `T`. Once again, errors are ignored.
4. Finally, everything is turned into a vector.

```rust
fn create_pairs<T: FromStr>(s: &str) -> Vec<(&str, T)>;
```

Example:

```rust
let input = "\
first: 1
second 2
   \t third   : 3
fourth
fifth  : 43\t
\tsixth
";

let v: Vec<(&str, u32)> = create_pairs(input);

assert_eq!(
    v,
    [
        ("first", 1),
        ("third", 3),
        ("fifth", 43),
    ]
);
```

Oh! You can use the `return` keyword in neither of those functions. And also, no `;`. I don't like
the name of this character.

## Exercise 03: The Great Fibonacci

```txt
turn-in directory:
    ex03/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::option::Option
    std::u32::{checked_add, checked_mul}
    std::iter::Iterator
    std::iter::Iterator::{take_while, filter, sum}
```

Create an **Iterator** named `Fibs` that yields _every_ fibonacci number that fits in a `u32`.

```rust
impl Fibs {
    fn new(first: u32, second: u32) -> Self;
}
```

- The `new` function must allow the user of the type to choose the first and second term of the
  sequence.
- Ensure that your iterator does _not_ panic when reaching the greatest fibonacci `u32`.

Example:

```rust
let mut count = 0;
for fib in Fibs::new(0, 1) {
    if fib >= 1000 {
        break;
    }
    if fib % 2 == 0 {
        count += fib;
    }
}

assert_eq!(count, 798);
```

Write the `even_fibs_bellow_1000` function.

```rust
fn even_fibs_bellow_1000() -> u32;
```

- This function must return the sum of even fibonacci numbers bellow `1000` using the `Fibs`
  iterator you previously wrote.
- You still can't use `return` nor can you use `;`s characters!

Example:

```rust
assert_eq!(even_fibs_bellow_1000(), 798);
```

## Exercuse 04: Monotically Increasing

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::iter::Iterator  std::option::Option
    std::cmp::{PartialOrd, Ordering}
    std::clone::Clone
```

Create a type named `Increasing<I>`.

```rust
struct Increasing<I: Iterator> {
    inner: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Increasing<I> {
    pub fn new<J>(iter: J) -> Self
    where
        J: IntoIterator<IntoIter = I>;
}
```

- The `new` function must create an instance of `Increasing` using the `IntoIterator`
  implementation of `J`.
- Implement the `Iterator` trait for `Increasing<I>`. You may add the bounds you wish to both `I`
  and `I::Item` (as long as the subject allows them) in order to make the assignmenet possible.
- The created iterator must filter any non-strictly-increasing number of the original iterator.

For example:

```rust
let mut iter = Increasing::new([0.4, 0.2, 0.1, 0.2, 0.4, 0.5, 0.4, 0.6]);
assert_eq!(iter.next(), Some(0.4));
assert_eq!(iter.next(), Some(0.5));
assert_eq!(iter.next(), Some(0.6));
assert_eq!(iter.next(), None);
```

## Exercise 05: From Side To Side

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::iter::{Iterator, DoubleEndedIterator}
    std::str::*  std::char::*  std::option::Option
```

Create a `Groups` iterator.

```rust
struct Groups<'a, F> {
    s: &'a str,
    f: F,
}

impl<'a, F> Groups<'a, F> {
    pub fn new(s: &'a str, f: F) -> Self
    where
        F: FnMut(char) -> bool;
}
```

- The `new` inherent method creates a new `Groups` instance.
- `Groups` must implement the `Iterator<Item = &str>` trait.
- The `f` function is called on every character of the string. As long as the function returns
  `true`. Characters for which the function returns `false` are ignored.

Example:

```rust
let mut groups = Groups::new("  hello,\tworld ", char::is_alphabetic);

assert_eq!(groups.next(), Some("hello"));
assert_eq!(groups.next(), Some("world"));
assert_eq!(groups.next(), None);
```

It must be possible to call `Iterator::rev` on your iterator to reverse it.

Example:

```rust
let mut groups = Groups::new("  abc\t def,test");

assert_eq!(groups.next(), Some("abc"));
assert_eq!(groups.next_back(), Some("test"));
assert_eq!(groups.next_back(), Some("def"));
assert_eq!(groups.next(), None);
```

## Exercise 06: More Format

```txt
turn-in directory:
    ex06/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::str::from_utf8
```

Let's create our own formatting system. Let's begin with a `Print` trait.

```rust
#[derive(Debug)]
struct FormatError;

type WriteFn<'a> = dyn 'a + FnMut(&str) -> Result<(), FormatError>;

trait Print {
    fn print(&self, write: &mut WriteFn) -> Result<(), FormatError>;
}
```

Calling `print` should use the `write` function to write an instance of the implementator.

Example, with an example implementation of `Print` for `u32`:

```rust
42u32.print(&mut |s| {
    assert_eq!(s, "42");
});
```

Let's create a `format_with` function to make use of that awesome trait.

```rust
fn format_with(s: &str, values: &[&dyn Print], write: &mut WriteFn) -> Result<(), FormatError>;

fn format_string(s: &str, values: &[&dyn Print]) -> Result<String, FormatError>;
fn format_print(s: &str, values: &[&dyn Print]) -> Result<(), FormatError>;
```

- The `format_with` function parses the input string `s`, and replaces `%` characters with the
  elements of `values`, calling `print` on them. If the length of `values` is not coherent with the
  input string, the function may panic.
- The `format_string` function uses `format_with` to create a `String`. If any formating function
  does not produce valid UTF-8, the function returns an error.
- The `format_print` function uses `format_with` to write to the standard output.

Example (assuming `Print` is implemented for `u32` and `&str`):

```rust
let s: String = format_string("salut % les % gens!", &[&14u32, &"Hello!"]).unwrap();
assert_eq!(s, "salut 14 les Hello! gens!");
```

## Exercise 07: Going Higher Order

```txt
turn-in directory:
    ex05/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::ops::Fn  str::{len, split_at, chars}
    char::{is_alphabetic, is_ascii_digit}
    std::iter::Iterator::{count, all, any}
```

A "pattern matcher" is basically a function that takes a string and returns whether that string
matches the pattern.

```rust
fn(input: &str) -> bool
```

Let's begin simple, let's create some "pattern matcher generators". Each must return a "pattern
matcher".

```rust
fn anything() -> impl Fn(&str) -> bool;
fn exactly(to_match: &str) -> impl Fn(&str) -> bool;
fn alphabetic() -> impl Fn(&str) -> bool;
fn ascii_digits() -> impl Fn(&str) -> bool;
fn min_size(min: usize) -> impl Fn(&str) -> bool;
```

- The `anything` function returns a "pattern matcher" that matches all strings. In other words, it
  always returns `true`.
- The `exactly` function returns a "pattern matcher" that matches strings exactly equal to
  `to_match`.
- The `alphabetic` function returns a "pattern matcher" that matches strings which characters are
  all alphabetic letters.
- The `ascii_digits` function returns a "pattern matcher" that matches strings which characters are
  ascii digits.
- The `min_size` function returns a "pattern matcher" that matches strings with a length greater or
  equal to `min` _characters_ (not _bytes_; _characters_).

Let's compilcate things a bit.

```rust
fn maybe(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool;
fn not(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool;
fn or(
    first: impl Fn(&str) -> bool,
    second: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool;
fn and(
    first: impl Fn(&str) -> bool,
    second: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool;
fn chain(
    first: impl Fn(&str) -> bool,
    second: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool;
```

- The `maybe` function returns a "pattern matcher" that matches strings that are either empty,
  or match the `pattern` "pattern matcher".
- The `not` function returns a "pattern matcher" that matches any string thats not matched by the
  `pattern` "pattern matcher".
- The `or` function returns a "pattern matcher" that matches strings that match either the `first`
  or `second` "pattern matcher" (both is also valid!).
- The `and` function returns a "pattern matcher" that matches strings that match both the `first`
  and `second` "pattern matcher".
- The `chain` function returns a "patterm matcher" that matches any string which can be split in
  two, such that the first part matches with `first` and the second part matches with `second`.

With those awesome tools, create three pattern matchers.

- The first one must match strings with this form:
  - at least one alphabetic character
  - the character '@'
  - at least one alphabetic character
  - the character '.'
  - either "fr" or "com"

```rust
let pattern = /* ... */;

assert!(pattern("my@address.com"));
assert!(pattern("a@b.fr"));
assert!(!pattern("@example.com"));
assert!(!pattern("abc@.com"));
assert!(!pattern("my@address.net"));
assert!(!pattern("myaddress.fr"));
assert!(!pattern("my-address@domain.fr"));
assert!(!pattern("address@my-domain.fr"));
```

- The second one must match strings with this form:
  - The character '('
  - zero or more characters that are neither '(' nor ')'.
  - The character ')'

```rust
let pattern = /* ... */;

assert!(pattern("()"));
assert!(pattern("(hello)"));
assert!(pattern("(hello 123)"));
assert!(!pattern("(hello () 123)"));
assert!(!pattern("(hello "));
assert!(!pattern(")"));
assert!(!pattern(" (test) "));
```

- The third one must match strings with this form:
  - An optional '+' or '-'.
  - one or more ascii digits
  - the following group is optional:
    - the character '.'.
    - at least one ascii digit.
  - the following group is optional
    - the character 'e' or 'E'
    - an optional '+' or '-'
    - at least one ascii digit

```rust
let pattern = /* ... */;

assert!(pattern("12"));
assert!(pattern("+12"));
assert!(pattern("-12"));
assert!(pattern("12.5"));
assert!(pattern("12.5e20"));
assert!(pattern("12E10"));
assert!(pattern("12E+9"));
assert!(pattern("12E-9"));
assert!(!pattern(""));
assert!(!pattern("+"));
assert!(!pattern("+12."));
assert!(!pattern("+12e"));
assert!(!pattern("+12.e10"));
assert!(!pattern("12e10.10"));
```
