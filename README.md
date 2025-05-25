# REPLisp

REPLisp is a Lisp-inspired programming language implemented in Rust. It features a REPL (Read-Eval-Print Loop) interface, a parser built with Chumsky, and a growing set of language features.

## Features

- Interactive REPL with command history and line editing (using rustyline)
- Chumsky-based parser for clean and maintainable parsing code
- Support for basic data types: numbers, symbols, and strings
- S-expressions and Q-expressions
- Built-in functions for arithmetic operations
- List manipulation functions (head, tail, join, etc.)
- Control flow with the `do` function
- Lambda functions for defining anonymous functions
- Detailed error reporting

## Installation

To build REPLisp, you'll need Rust and Cargo installed. Clone this repository and run:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/replisp`.

## Usage

### REPL Mode

To start the interactive REPL, simply run the program without arguments:

```bash
cargo run
```

In the REPL, you can type expressions to evaluate them. Type `:q` to exit.

### File Mode

To execute a REPLisp script file:

```bash
cargo run -- path/to/script.lisp
```

## Examples

The `examples` directory contains sample REPLisp programs:

- `simple.lisp` - Basic string printing
- `test.lisp` - Demonstrates the `do` function and arithmetic
- `calc.lisp` - A simple calculator

To run an example:

```bash
cargo run -- examples/simple.lisp
```

## Language Guide

### Basic Syntax

REPLisp uses S-expressions for function calls:

```lisp
(+ 1 2)  ; Returns 3
```

### Built-in Functions

- Arithmetic: `+`, `-`, `*`, `/`, `^` (power), `%` (remainder), `min`, `max`
- List operations: `list`, `head`, `tail`, `join`, `cons`, `len`, `init`
- Control flow: `do` (execute multiple expressions in sequence)
- Other: `print`, `exit`

### Lambda Functions

Define anonymous functions using the `\` or `fun` syntax:

```lisp
(\ {x y} {+ x y})  ; A function that adds two numbers
```

## Development

REPLisp is under active development. Future plans include:

- Variable definition and scoping
- Conditionals (if/else)
- Standard library
- File I/O
- More data types

## Credits

This project draws inspiration from:

- [Build Your Own Lisp](http://buildyourownlisp.com)
- [blispr](https://github.com/deciduously/blispr)
- [Make A Lisp](https://github.com/kanaka/mal)
