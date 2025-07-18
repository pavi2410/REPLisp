; List processing examples
; This example demonstrates list operations and manipulation

(print "=== List Processing ===")

; Creating lists
(def numbers '(1 2 3 4 5))
(def colors (list "red" "green" "blue"))
(def mixed (list 1 "hello" 2 "world"))

(print "Numbers list:")
(print numbers)

(print "Colors list:")
(print colors)

; Basic list operations
(print "First element (car):")
(print (car numbers))              ; 1

(print "Rest of list (cdr):")
(print (cdr numbers))              ; (2 3 4 5)

(print "Length of numbers:")
(print (length numbers))           ; 5

; Building lists with cons
(print "Cons 0 to numbers:")
(print (cons 0 numbers))           ; (0 1 2 3 4 5)

; List manipulation functions
(defn sum-list (lst)
  (if (null? lst)
      0
      (+ (car lst) (sum-list (cdr lst)))))

(print "Sum of numbers:")
(print (sum-list numbers))         ; 15

(defn length-recursive (lst)
  (if (null? lst)
      0
      (+ 1 (length-recursive (cdr lst)))))

(print "Length (recursive):")
(print (length-recursive numbers)) ; 5

; Reverse a list
(defn reverse-list (lst)
  (defn reverse-helper (lst acc)
    (if (null? lst)
        acc
        (reverse-helper (cdr lst) (cons (car lst) acc))))
  (reverse-helper lst '()))

(print "Reversed numbers:")
(print (reverse-list numbers))     ; (5 4 3 2 1)

; Find element in list
(defn member? (item lst)
  (cond
    ((null? lst) false)
    ((= item (car lst)) true)
    (else (member? item (cdr lst)))))

(print "Is 3 in numbers?")
(print (member? 3 numbers))        ; true

(print "Is 7 in numbers?")
(print (member? 7 numbers))        ; false

; Map function over list
(defn map-list (func lst)
  (if (null? lst)
      '()
      (cons (func (car lst)) (map-list func (cdr lst)))))

(defn square (x) (* x x))

(print "Squared numbers:")
(print (map-list square numbers))  ; (1 4 9 16 25)

; Filter list
(defn filter-list (pred lst)
  (cond
    ((null? lst) '())
    ((pred (car lst)) (cons (car lst) (filter-list pred (cdr lst))))
    (else (filter-list pred (cdr lst)))))

(defn even? (x) (= (mod x 2) 0))

(print "Even numbers:")
(print (filter-list even? numbers)) ; (2 4)

; Append lists
(defn append-lists (lst1 lst2)
  (if (null? lst1)
      lst2
      (cons (car lst1) (append-lists (cdr lst1) lst2))))

(def more-numbers '(6 7 8))
(print "Appended lists:")
(print (append-lists numbers more-numbers)) ; (1 2 3 4 5 6 7 8)