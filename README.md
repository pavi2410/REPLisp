# REPLisp

REPLisp is a modern Lisp-inspired programming language implemented in Rust with no external dependencies. It features a clean syntax, powerful metaprogramming capabilities, and a focus on simplicity and expressiveness.

## Features

- Pure Rust implementation with zero external dependencies
- Interactive REPL (Read-Eval-Print Loop)
- Modern Lisp syntax with S-expressions
- First-class functions and closures
- Lexical scoping
- Homoiconicity (code as data)
- Tail call optimization
- Comprehensive error reporting

## Installation

To build REPLisp, you'll need Rust and Cargo installed. Clone this repository and run:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/replisp`.

## Usage

### REPL Mode

To start the interactive REPL:

```bash
cargo run
```

### File Mode

To execute a REPLisp script file:

```bash
cargo run -- path/to/script.lisp
```

## Syntax Guide

### Basic Expressions

REPLisp uses S-expressions (symbolic expressions) where operations are written in prefix notation:

```lisp
; Numbers
42
3.14159

; Strings
"Hello, World!"

; Symbols
foo
+
my-variable

; Comments start with semicolon
; This is a comment
```

### Function Calls

Function calls use parentheses with the function name first:

```lisp
(+ 1 2 3)           ; Addition: 6
(* 4 5)             ; Multiplication: 20
(print "Hello")     ; Print to stdout
```

### Lists

Lists are fundamental data structures:

```lisp
'(1 2 3 4)          ; Quoted list
(list 1 2 3 4)      ; Constructed list
(cons 1 '(2 3))     ; Prepend element: (1 2 3)
(car '(1 2 3))      ; First element: 1
(cdr '(1 2 3))      ; Rest of list: (2 3)
```

### Variable Definition

Define variables with `def`:

```lisp
(def x 10)
(def name "Alice")
(def my-list '(1 2 3))
```

### Functions

Define functions with `defn`:

```lisp
; Named function
(defn square (x)
  (* x x))

; Anonymous function (lambda)
(lambda (x y) (+ x y))

; Function with multiple expressions
(defn greet (name)
  (print "Hello,")
  (print name))
```

### Conditionals

Use `if` for conditional expressions:

```lisp
(if (> x 0) "positive" "non-positive")

; Multi-way conditionals with cond
(cond
  ((< x 0) "negative")
  ((> x 0) "positive")
  (else "zero"))
```

### Loops and Iteration

```lisp
; Map function over list
(map square '(1 2 3 4))     ; (1 4 9 16)

; Filter list elements
(filter (lambda (x) (> x 0)) '(-1 2 -3 4))  ; (2 4)

; Reduce/fold
(reduce + 0 '(1 2 3 4))     ; 10
```

### Let Bindings

Create local bindings with `let`:

```lisp
(let ((x 10)
      (y 20))
  (+ x y))                  ; 30
```

### Macros

Define macros for code transformation:

```lisp
(defmacro when (condition . body)
  `(if ,condition (do ,@body)))

(when (> x 0)
  (print "positive")
  (print "number"))
```

### Built-in Functions

#### Arithmetic
- `+`, `-`, `*`, `/` - Basic arithmetic
- `mod` - Modulo operation
- `abs` - Absolute value
- `min`, `max` - Minimum/maximum

#### Comparison
- `=`, `<`, `>`, `<=`, `>=` - Comparison operators
- `eq?` - Object equality

#### List Operations
- `list` - Create list
- `cons` - Prepend element
- `car`, `cdr` - First element, rest of list
- `length` - List length
- `append` - Join lists
- `reverse` - Reverse list

#### Type Predicates
- `number?`, `string?`, `symbol?`, `list?` - Type checking
- `null?` - Check for empty list

#### I/O
- `print` - Print to stdout
- `read` - Read from stdin

### Special Forms

Special forms are evaluated differently from regular functions:

- `quote` or `'` - Prevent evaluation
- `def` - Define variable
- `defn` - Define function
- `if` - Conditional
- `cond` - Multi-way conditional
- `let` - Local bindings
- `lambda` - Anonymous function
- `defmacro` - Define macro

### Example Programs

#### Factorial Function
```lisp
(defn factorial (n)
  (if (<= n 1)
      1
      (* n (factorial (- n 1)))))

(factorial 5)  ; 120
```

#### List Processing
```lisp
(defn sum-list (lst)
  (if (null? lst)
      0
      (+ (car lst) (sum-list (cdr lst)))))

(sum-list '(1 2 3 4 5))  ; 15
```

#### Higher-Order Functions
```lisp
(defn apply-twice (f x)
  (f (f x)))

(apply-twice (lambda (x) (* x 2)) 5)  ; 20
```

## Development

REPLisp is under active development. Current focus areas:

- Core language features
- Standard library expansion
- Performance optimizations
- Error handling improvements
- Documentation and examples
