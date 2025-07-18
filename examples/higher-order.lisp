; Higher-order functions and functional programming
; This example demonstrates functions as first-class values

(print "=== Higher-Order Functions ===")

; Functions that take other functions as arguments
(defn apply-twice (f x)
  (f (f x)))

(defn double (x) (* x 2))
(defn increment (x) (+ x 1))

(print "Apply double twice to 3:")
(print (apply-twice double 3))     ; 12

(print "Apply increment twice to 5:")
(print (apply-twice increment 5))  ; 7

; Function composition
(defn compose (f g)
  (lambda (x) (f (g x))))

(def increment-then-double (compose double increment))
(print "Increment then double 4:")
(print (increment-then-double 4))  ; 10

; Partial application
(defn partial (f a)
  (lambda (b) (f a b)))

(defn add (x y) (+ x y))
(def add-five (partial add 5))

(print "Add 5 to 3:")
(print (add-five 3))               ; 8

; Curry function
(defn curry (f)
  (lambda (x) (lambda (y) (f x y))))

(def curried-add (curry add))
(def add-ten ((curried-add 10)))

(print "Curried add 10 to 7:")
(print (add-ten 7))                ; 17

; Map, filter, and reduce implementations
(defn map (f lst)
  (if (null? lst)
      '()
      (cons (f (car lst)) (map f (cdr lst)))))

(defn filter (pred lst)
  (cond
    ((null? lst) '())
    ((pred (car lst)) (cons (car lst) (filter pred (cdr lst))))
    (else (filter pred (cdr lst)))))

(defn reduce (f init lst)
  (if (null? lst)
      init
      (reduce f (f init (car lst)) (cdr lst))))

; Using higher-order functions
(def numbers '(1 2 3 4 5))

(print "Original numbers:")
(print numbers)

(print "Doubled numbers:")
(print (map double numbers))       ; (2 4 6 8 10)

(defn positive? (x) (> x 0))
(defn negative? (x) (< x 0))

(def mixed-numbers '(-2 -1 0 1 2 3))
(print "Positive numbers from mixed:")
(print (filter positive? mixed-numbers)) ; (1 2 3)

(print "Sum using reduce:")
(print (reduce + 0 numbers))       ; 15

(print "Product using reduce:")
(print (reduce * 1 numbers))       ; 120

; Function factories
(defn make-adder (n)
  (lambda (x) (+ x n)))

(def add-three (make-adder 3))
(def add-seven (make-adder 7))

(print "Add 3 to 10:")
(print (add-three 10))             ; 13

(print "Add 7 to 10:")
(print (add-seven 10))             ; 17

; Predicate combinators
(defn not-pred (pred)
  (lambda (x) (not (pred x))))

(defn and-pred (pred1 pred2)
  (lambda (x) (and (pred1 x) (pred2 x))))

(defn or-pred (pred1 pred2)
  (lambda (x) (or (pred1 x) (pred2 x))))

(defn even? (x) (= (mod x 2) 0))
(defn greater-than-two? (x) (> x 2))

(def even-and-gt-two (and-pred even? greater-than-two?))

(print "Numbers that are even and > 2:")
(print (filter even-and-gt-two numbers)) ; (4)