# REPLisp

## Usage

  > You must have NodeJS installed.

  To start the REPL, run
  ```
  node REPLisp.js
  ```
  To execute a file (`.rep`), run
  ```
  node REPLisp.js <file.rep>
  ```

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
  (% 7 5) # modulo

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
  (for (i) in (range 5)
    (print i)
  )
  
  # Variable
  (var i 0)
  (set i (+ i 10))
  (print i) # 10

  # Function
  (function greet (name)
    (print (+ "Hello, " name "!"))
  )

  (greet "world") # Hello, world!

  # Comment
  # Hello world, I'm a comment!
  ```

## Testing

  ```
  node test.js
  ```

---

## Useful Resources

- https://ruslanspivak.com/lsbasi-part1/
- http://norvig.com/lispy.html
- http://norvig.com/lispy2.html
- http://www.craftinginterpreters.com
- http://lisperator.net/pltut/
- https://github.com/kanaka/mal/
- https://github.com/kanaka/miniMAL/
