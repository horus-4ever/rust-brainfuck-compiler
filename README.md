## Table of contents
* [General Info](#general-info)
* [Description](#description)
* [Examples](#examples)
* [TODO List](#todo-list)

## General Info
This project is a brainfuck compiler entirely written in Rust. It aims to produce fast and secured brainfuck binaries.

## Decription
It has been written using the most recent stable Rust release.

This compiler compiles brainfuck source code to x86 binaries. It targets Linux specifically, since I use directly the Linux syscalls for input and output, and not the C library.

Security of the binary :
* **compile time** : it detects at compile time mismatched `[` or `]`
* **runtime** : if the brainfuck stack pointer produces an overflow or underflow, then the programm stops immediatly. For now, there is no description on the error.

## Examples
In the `test` folder, there are some brainfuck code examples. In the `build` directory, there is a CLI Mandelbrot programm. The original brainfuck source code is not provided.

## TODO List
- [x] Working and efficient compiler
- [x] A real error handling for runtime errors
- [ ] Add command line argument to the compiler
