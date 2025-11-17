; Demonstration of bytecode compilation features
; Compile with: replisp --compile examples/bytecode_demo.lisp
; Run with: java -noverify Main

(print "=== REPLisp Bytecode Compiler Demo ===")

; Variables
(def pi 3.14159)
(print "Pi:")
(print pi)

; Simple function
(defn square (x)
  (* x x))

(print "Square of 7:")
(print (square 7))

; Multiple parameters
(defn add-three (a b c)
  (+ a (+ b c)))

(print "Add 10 + 20 + 30:")
(print (add-three 10 20 30))

; Function calling function
(defn sum-of-squares (a b)
  (+ (square a) (square b)))

(print "Sum of squares of 3 and 4:")
(print (sum-of-squares 3 4))

; Conditional
(defn max (a b)
  (if (> a b)
      a
      b))

(print "Max of 15 and 23:")
(print (max 15 23))

; Lists
(def numbers (list 10 20 30 40 50))
(print "Numbers list:")
(print numbers)

(print "First number:")
(print (car numbers))

(print "List length:")
(print (length numbers))

(print "Is list empty?:")
(print (null? numbers))

(print "Is empty list empty?:")
(print (null? (list)))

; Cons
(def extended (cons 0 numbers))
(print "Cons 0 to numbers:")
(print extended)

(print "=== Demo Complete ===")
