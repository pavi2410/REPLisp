# REPLisp Java Bytecode Compiler

This document describes the Java bytecode compiler for REPLisp, which compiles REPLisp source code to JVM bytecode targeting **Java 8**.

## Usage

Compile a REPLisp file to Java bytecode:

```bash
cargo build --release
./target/release/replisp --compile input.lisp
java Main
```

Specify a custom output file:

```bash
./target/release/replisp --compile input.lisp --output MyProgram.class
java MyProgram
```

## Supported Features ✅

- **Numbers, Strings, Booleans, Nil, Lists**
- **Arithmetic**: `+`, `-`, `*`, `/`, `mod`
- **Comparisons**: `=`, `<`, `>`, `<=`, `>=`
- **Variables**: `def`
- **Functions**: `defn`, `lambda` (as static methods)
- **Function calls** with recursion support
- **Lists**: `list`, `car`, `length`, `cons`, `null?`
- **Control flow**: `if`, `do`
- **I/O**: `print`

## Example

```lisp
(defn square (x)
  (* x x))

(print (square 5))     ; 25.0

(def mylist (list 1 2 3))
(print (car mylist))   ; 1.0
(print (length mylist)); 3.0
```

## Running

Standard execution:
```bash
java Main
```

With verification disabled (recommended for now):
```bash
java -noverify Main
```

Note: Stackmap frame generation not yet implemented. Use `-noverify` for complex control flow.

## Details

- **Class version**: 52.0 (Java 8)
- **Values**: Boxed as Java objects (Double, String, Boolean, ArrayList)
- **Functions**: Compiled to `public static` methods
- **Two-pass compilation**: Register functions, then compile bodies

See full documentation in the source code.
