; Basic arithmetic operations in REPLisp
; This example demonstrates basic mathematical operations

(print "=== Basic Arithmetic ===")

; Simple addition
(print "(+ 5 3) =" (+ 5 3))

; Multiple operands
(print "(+ 1 2 3 4 5) =" (+ 1 2 3 4 5))

; Mixed operations
(print "(- 10 3) =" (- 10 3))
(print "(* 4 7) =" (* 4 7))
(print "(/ 20 4) =" (/ 20 4))

; Nested expressions
(print "(+ (* 2 3) (- 8 2)) =" (+ (* 2 3) (- 8 2)))

; Working with variables
(def x 10)
(def y 5)
(print "x =" x)
(print "y =" y)
(print "(+ x y) =" (+ x y))
(print "(* x y) =" (* x y))

; More complex expressions
(def result (+ (* x 2) (/ y 5)))
(print "(+ (* x 2) (/ y 5)) =" result)

; Comparison operations
(print "(> x y) =" (> x y))
(print "(< x y) =" (< x y))
(print "(= x 10) =" (= x 10))

; Min and max
(print "(min 3 7 2 9 1) =" (min 3 7 2 9 1))
(print "(max 3 7 2 9 1) =" (max 3 7 2 9 1))

; Absolute value and modulo
(print "(abs -5) =" (abs -5))
(print "(mod 17 5) =" (mod 17 5))