# REPLisp

## Usage

  ```
  REPLisp --help
  ```

## File extension

  This language uses `.rep` as its file extension

## Syntax

  ```
  # String
  "Hello World"
  
  # Numbers
  123
  -123
  12.3

  # Boolean
  true
  false

  null

  # Collections
  (list 1 2 3)
  (set 1 2 3)
  (map (a 1) (b 2) (c 3))

  # Operators
  # Math
  (+ 8 6) # addition
  (- 4 9) # subtraction
  (* 2 7) # multiplication
  (/ 8 3) # division
  (^ 2 3) # power

  # Comparion
  (= 5 5)  # equal to
  (!= 8 8) # not equal to
  (> 5 10) # greater than
  (>= 6 8) # greater than or equal to
  (< 9 3)  # less than
  (<= 2 9) # less than or equal to
  
  (not true)
  (or false true)
  (and true false)
  (xor true true)

  # Loop
  (while true (
    # Do some work
  ))

  (for (var i 0) (< i 10) (+ i 1) (
    # Do your work
  ))

  # Variable
  (var i 0)

  (set i (+ i 10))

  (print i) # 10

  # Function
  (fun greet (name) (
    (print (+ "Hello, " name "!"))
  ))

  (greet "world") # Hello, world!

  # Comment
  # Hello world, I'm a comment!
  ```

## Testing

  ```
  REPLisp --test [--debug]
  ```

---

## Useful Resources

- https://ruslanspivak.com/lsbasi-part1/
- http://norvig.com/lispy.html
- http://norvig.com/lispy2.html
- http://www.craftinginterpreters.com
- http://lisperator.net/pltut/
