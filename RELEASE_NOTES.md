### Version 0.1

- Supports only addition and subtraction with only 2 operands in one statement.

### Version 0.2

- Replaced add & sub operators with `+` & `-`
- More new operators available
  - `*` (Multiplication)
  - `/` (Division)
  - `%` (Remainder)
- Allow integers of any number of digits

### Version 0.3

- Refactored code (Moved functions into separate files)

### Version 0.4

- Added an interactive CLI

### Version 0.5

- Changed Division operator to `/`

### Version 0.6

- Added `^` (power) operator

### Version 0.7

- Redesigned CLI
- Renamed project name to 'REPLisp'

### Version 0.8

- Source code moved to Github
- Using ES6 imports
- Organised files into folders (See README.md)
- Added 'REPL.js' API front-end
- Allow variable arguments
  - Additition `+` & Multiplication `*` can now use more than two arguments
- Added README.md

### Version 0.9

- Added a JS transpiler
- Improved CLI UX: Use one of the following options
  `REPLisp [option]`
  - no option: Run in REPL mode
  - `--debug`: Show tokens & AST in REPL mode
  - `--transpile`: Show transpiled JS in REPL mode
  - `--test`: Run tests (can be coupled with `--debug` & `--transpile`)
  
### Version 1.0

- Added strings, decimal, boolean
- Improved Lexer
