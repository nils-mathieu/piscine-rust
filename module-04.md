# Module 04: Side Effects

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

Create a **program** that reads the standard input of the program, and copies it to the standard
output, as well as every file specified in command-line arguments.

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

## Exercise 02: TODO

```txt
turn-in directory:
    ex02/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::fs::Metadata
```

TODO: Open a file/directory, and print information about it?

## Exercise 03: TODO

TODO:

## Exercise 04: Silence It!

```txt
turn-in directory:
    ex04/

files to turn in:
    std/main.rs  Cargo.toml
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
```

Create a **program** that reads an arbitrary binary file, and prints the UTF-8 strings it finds.

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

## Exercise 06: Reverse Shell

```txt
turn-in directory:
    ex06/

files to turn in:
    std/client.rs  std/server.rs  Cargo.toml
```

Create a **Not So Secure Shell** client/server that allows a user to use a shell from a remote
computer.

```txt
======= SERVER =======
>_ cargo run --bin server -- 127.0.0.1:49150 /bin/bash
remote-computer$ echo test
test
remote-computer$

======= CLIENT =======
>_ cargo run --bin client -- 127.0.0.1:49150
remote-computer$ echo test
test
remote-computer$
```

The server must:

1. Open socket connection and listen for incoming connections. When a connection is available, the
server stops accepting new connections.
2. Start the program specified in command-line arguments. It pipes the socket's output into the
shell's standard input, and the shell standard output into the socket's input.
3. Both the standard output of the child process, and the output of the socket must be copied to
its standard output.
4. When the remote client is disconnected, or when the child command exists, it stops.

The client must:

1. Connect to the specified address.
2. Pipe its standard input into the socket's input. Pipe the socket's output into its standard
output.
3. If the socket is disconnected, the program stops.

Errors must be handled properly in both the server and the client!

## Exercise 07: PBP

```txt
turn-in directory:
    ex07/

files to turn in:
    std/client.rs  std/server.rs  Cargo.toml
```

TODO: Write an asymetric encryptor and decryptor that uses private and public keys.
