## Table of contents
* [General Info](#general-info)
* [Description](#description)
* [Examples](#examples)
* [TODO List](#todo-list)

## General Info
This project is a brainfuck compiler entirely written in Rust. It aims to produce fast and secured brainfuck binaries.

## Decription
It has been written using the most recent stable Rust release.

This compiler compiles brainfuck source code to x86 binaries. 

Security of the binary :
* **compile time** : it detects at compile time mismatched `[` or `]`
* **runtime** : if the brainfuck stack pointer produces an overflow or underflow, then the programm stops immediatly. It displays the type of error and where it occured in the source code.

## Examples
In the `test` folder, there are some brainfuck code examples. In the `build` directory, there is a CLI Mandelbrot programm. The original brainfuck source code is not provided.

Example of error output, with the `overflow_error.bf` test code :
```
Runtime error : The stack pointer exceeded the stack size => |>|
Error occured at pos (row, column) : 1 2
```

## TODO List
- [x] Working and efficient compiler
- [x] A real error handling for runtime errors
- [ ] Add command line argument to the compiler
