; Basic arithmetic operations in REPLisp
; This example demonstrates basic mathematical operations

(print "=== Basic Arithmetic ===")

; Simple addition
(print (+ 5 3))                    ; 8

; Multiple operands
(print (+ 1 2 3 4 5))              ; 15

; Mixed operations
(print (- 10 3))                   ; 7
(print (* 4 7))                    ; 28
(print (/ 20 4))                   ; 5

; Nested expressions
(print (+ (* 2 3) (- 8 2)))        ; 12

; Working with variables
(def x 10)
(def y 5)
(print (+ x y))                    ; 15
(print (* x y))                    ; 50

; More complex expressions
(def result (+ (* x 2) (/ y 5)))
(print result)                     ; 21

; Comparison operations
(print (> x y))                    ; true
(print (< x y))                    ; false
(print (= x 10))                   ; true

; Min and max
(print (min 3 7 2 9 1))            ; 1
(print (max 3 7 2 9 1))            ; 9

; Absolute value and modulo
(print (abs -5))                   ; 5
(print (mod 17 5))                 ; 2