# Module 04

## Introduction

TODO: Reading files is cool.

## Module 00: Tee-Hee

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

## Module 01: Duh

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

## Module 02: TODO

```txt
turn-in directory:
    ex02/

files to turn in:
    std/main.rs  Cargo.toml

allowed symbols:
    std::fs::Metadata
```

TODO: Open a file/directory, and print information about it?

## Module 03: TODO

TODO:

## Module 04: Silence It!

TODO: replace symbol of elf file by another because it's funny.
The modification must be done in-place. Remove `write` by `sleep`.

Maybe only edit the "strings" section of the elf? That uses a tiny bit of parsing.

## Module 05: String Finder

```txt
turn-in directory:
    ex05/

files to turn in:
    std/main.rs  Cargo.toml
```

TODO: Create a **program** that reads a file, and prints the UTF-8 strings it finds.

## Module 06: Remote Shell

```txt
turn-in directory:
    ex06/

files to turn in:
    std/client.rs  std/server.rs  Cargo.toml
```

TODO: Create a "Not Secure Shell" that allows connecting to a remote computer and use it. Nothing
is secured. The standard input of the client to sent to the standard input of the server, the
standard output of the server is sent to the standard output of the client.

## Module 07: PBP

TODO: Write an asymetric encryptor and decryptor that uses private and public keys.
