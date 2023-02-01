# Module 05: Side Effects

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

## Exercise 00: Tee-Hee

```txt
turn-in directory:
    ex00/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::io::{Write, Read, stdin, stdout, stderr}
    std::fs::File  std::env::args
    std::vec::Vec  std::string::String
```

Create a **program** that reads the standard input, and copies it to the standard output, as well as
to every file specified in command-line arguments.

Example:

```txt
>_ echo "Hello, World!" | cargo run -- a b c
Hello, World!
>_ cat a b c
Hello, World!
Hello, World!
Hello, World!
```

You program must not panic when interacting with the file system. All errors must be handled
properly.

## Exercise 01: Duh

```txt
turn-in directory:
    ex01/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::fs::{metadata, Metadata}
    std::env::args
```

Create a **program** that computes the total size of a directory or file. The program must write the
aggregated size of directories *in real-time*. As more files are taken in account in the count,
the total size must be updated in the terminal.

```txt
>_ cargo run -- ~
1.2 gigabytes
```

* If a size is less than a kilobyte, it is written in bytes. (e.g. 245 bytes)
* If a size is more than a kilobyte, it is written in kilobytes, with one decimal (e.g. 12.2
kilobytes).
* If a size is more than a megabyte, it is written in megabytes, with one decimal (e.g. 100.4
megabytes).
* If a size is more than a gigabyte, it is written in gigabytes, with one decimal (e.g. 23.9
gigabytes).

Your program must not panic when interacting with the file system. Errors must be handled properly.

## Exercise 02: to_args

```txt
turn-in directory:
    ex02/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::env::args
    std::process::Command
    std::io::stdin
```

Create a **program** takes a path and some arguments as an input, and spawns that process with:

1. The arguments passed in command-line arguments.
2. Each line of its standard input.

Example:

```rust
>_ << EOF cargo run -- echo -n
hello
test
EOF
hello test>_
```

The program called the `echo -n hello test` command.

Your program must not panic when interacting with the system, you must handle errors properly.

## Exercise 03: Command Multiplexer

```txt
turn-in directory:
    ex03/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::env::args
    std::process::{Command, Stdio}
    std::vec::Vec
    std::io::{stdout, Write, Read}
```

Create a **program** that starts multiple commands, gather their output and then print it to its
standard output, in their original order without any of them mixing with any other. Standard error
output is ignored. 

Example:

```txt
>_ cargo run -- echo a b , sleep 1 , echo b , cat Cargo.toml
===== echo a b =====
a b

==== sleep 1 =====

===== echo b =====
b

==== cat Cargo.toml =====
[package]
name = "ex03"
version = "0.1.0"
...
```

Commands must be executed in parallel! Any error occuring when interacting with the system must be
handled properly.

## Exercise 04: Silence It!

```txt
turn-in directory:
    ex04/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::env::args
    std::fs::File  std::io::{Read, Write, Seek}
```

Create a **program** that replaces any call to the `write` function in an ELF file by the `sleep`
function.

Example:

```txt
>_ ./a.out
C is the best programming language! Fight me.
>_ cargo run -- a.out
>_ ./a.out
>_ cargo run -- Cargo.toml
error: not an ELF file
```

* The modification must be done in-place. Do not overwrite the file completely!
* Do not read to whole file. Skip to the parts that you actually need.
* Only the "string" section of the ELF file must be edited. Be careful when editing the text! You
must not edit the function when it does not refer to unistd's `write` function.

For example, the following program must *not* be patched:

```c
#include <stdio.h>

void write(char const *s)
{
    printf("%s", s);
}

int main(void)
{
    write("Hello, World!");    
    return 0;
}
```

Parsing errors and I/O errors must be handled properly.

## Exercise 05: String Finder

```txt
turn-in directory:
    ex05/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::env::args
    std::fs::File  std::io::{Write, Read, stdout, stdin}
```

Create a **program** that reads an arbitrary binary file, and prints the UTF-8 strings it finds.
When no file is provided, the standard input is used instead.

Example:

```txt
>_ cargo run -- ./a.out
TODO:
```

The program must have the following options available:

* `-z` filters out strings that are not null-terminated.
* `-m <min>` filters out strings that are strictly smaller than `min`.
* `-M <max>` filters out strings that are strictly larger than `max`.

Errors when interacting with the file system must be handled properly!

## Exercise 06: Dog

```txt
turn-in directory:
    ex07/

files to turn in:
    std/server.rs  std/client.rs  Cargo.toml

allowed symbols:
    std::env::args
    std::net::{TcpStream, TcpListener, SocketAddr}
    std::io::{Write, Read}
```

Create two **programs**.

* The first program is the server. Its role is to wait for TCP connections. When data is received,
it is retransmitted to all other connections.

* The second program is the client. Its role is to connect to the server, write the data it sends
to its standard output, and send its standard input to the server.

Example:

```txt
===== SERVER =====
>_ cargo run --bin server -- 127.0.0.1:49150
cedric: Hey!
kevin: How are you?

===== CLIENT 1 =====
>_ cargo run --bin client -- 127.0.0.1:49150 cedric
Hey!
cedric: Hey!
kevin: How are you?

===== CLIENT 2 =====
>_ cargo run --bin client -- 127.0.0.1:49150 kevin
cedric: Hey!
How are you?
kevin: How are you?
```

Note that this interface is only an example! You can experiment with this!

When the end-of-file is reached, the client closes its connection with the server. In both binaries,
errors must be handled properly!

## Exercise 07: Pretty Bad Privacy

```txt
turn-in directory:
    ex07/

files to turn in:
    std/main.rs src/*.rs  Cargo.toml

allowed dependencies:
    crypto_bigint(v0.4.9)  rand(v0.8.5)

allowed symbols:
    std::vec::Vec
    std::env::args
    std::io::{stdin, stdout, stderr, Write, Read}
    std::fs::File
    rand::*
    crypto_bigint::*
```

Write a **program** that behaves in the following way:

```txt
>_ cargo run -- gen-keys my-key.pub my-key.priv
>_ << EOF cargo run -- encrypt my-key.pub > encypted-message
This is a very secret message.
EOF
>_ cat encrypted-message | cargo run -- decrypt my-key.priv
This is a very secret message.
```

In order to generate keys, your program must perform the following steps:

1. Generate two random prime numbers (`p` and `q`).
2. Let `M` be their product.
3. Let `Phi` be the value `(p - 1) * (q - 1)`.
4. Pick a random `E`, such that:
    * `E < Phi`
    * `E` and `Phi` are coprime
    * `E` and `M` are coprime
5. Pick a random `D`, any multiplicative inverse of `E` modulo `Phi`.

Your private key is `(D, M)`, and your public key is `(E, M)`.

* With the public key, you can encrypt any number: `encrypt(m) { m^E % M }`.
* With the private key, you can decrypt the original message: `decrypt(m') { m'^D % M }`.
* Obviously, for any `m < M`, `decrypt(encrypt(m)) == m`.

Now that you have your private and public keys, you can already create the `gen-keys` subcommand,
which saves both keys to files specified as arguments to the command. You can choose the format that
you like, may it be binary or textual.

Let's define a new value: `B`, the "block size".

* Let `B` be the largest integer such that `255^B < M`. 

In order to encrypt or decrypt a block, take `B` bytes of your message, treat it as a
very big base-256 number, and put it through your encryption/decryption function. That's your
encrypted/decrypted block! Perform the operation for every block of the message, and voilà!
