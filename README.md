# REPLisp

## Usage

  ```
  REPLisp --help
  ```

## File extension

  This language uses `.rep` as its file extension

## Syntax

  - Atoms
    
    ```javascript
    "Hello World"
    123
    -123
    12.3
    true
    false
    null
    ```
    
  - Collections
    
    ```clojure
    (list 1 2 3)
    (set 1 2 3)
    (map (a 1) (b 2) (c 3))
    ```
    
  - Operators
    
    ```clojure
    # Math
    (+ 8 6) # addition
    (- 4 9) # subtraction
    (* 2 7) # multiplication
    (/ 8 3) # division
    (^ 2 3) # power
    
    # Comparion
    (= 5 5)  # equal to
    (> 5 10) # greater than
    (>= 6 8) # greater than or equal to
    (< 9 3)  # less than
    (<= 2 9) # less than or equal to
    (not true)
    (or false true)
    (and true false)
    (xor true true)
    ```
    
  - Loops
    
    ```clojure
    (while true (
      # Do some work
    ))
    
    (for (var i 0) (< i 10) (+ i 1) (
      # Do your work
    ))
    ```
    
  - Variable
    
    ```clojure
    (var i 0)
    
    (set i (+ i 10))
    
    (print i) # 10
    ```
    
  - Function
    
    ```clojure
    (fun greet (name) (
      (print (+ "Hello, " name "!"))
    ))
    
    (greet "world") # Hello, world!
    ```
    
  - Comment
    
    ```python
    # Hello, I'm a comment!
    ```

## Testing

  ```
  REPLisp --test [--debug]
  ```
