; Conditional expressions and control flow
; This example demonstrates if, cond, and logical operations

(print "=== Conditionals and Control Flow ===")

; Basic if expressions
(def x 10)
(def y 5)

(print "x > y:")
(print (if (> x y) "x is greater" "y is greater")) ; x is greater

(print "x = y:")
(print (if (= x y) "equal" "not equal"))           ; not equal

; Nested conditionals
(def grade 85)
(def letter-grade
  (if (>= grade 90)
      "A"
      (if (>= grade 80)
          "B"
          (if (>= grade 70)
              "C"
              (if (>= grade 60)
                  "D"
                  "F")))))

(print "Grade 85 is:")
(print letter-grade)               ; B

; Multi-way conditionals with cond
(defn grade-to-letter (score)
  (cond
    ((>= score 90) "A")
    ((>= score 80) "B")
    ((>= score 70) "C")
    ((>= score 60) "D")
    (else "F")))

(print "Grade 92 is:")
(print (grade-to-letter 92))       ; A

(print "Grade 55 is:")
(print (grade-to-letter 55))       ; F

; Logical operations
(def a true)
(def b false)

(print "Logical AND:")
(print (and a b))                  ; false
(print (and a true))               ; true

(print "Logical OR:")
(print (or a b))                   ; true
(print (or b false))               ; false

(print "Logical NOT:")
(print (not a))                    ; false
(print (not b))                    ; true

; Complex conditions
(def age 25)
(def has-license true)
(def has-insurance true)

(defn can-drive? (age license insurance)
  (and (>= age 18) license insurance))

(print "Can drive?")
(print (can-drive? age has-license has-insurance)) ; true

; Using conditionals in functions
(defn abs-value (n)
  (if (< n 0) (- n) n))

(print "Absolute value of -7:")
(print (abs-value -7))             ; 7

(print "Absolute value of 3:")
(print (abs-value 3))              ; 3

; Sign function
(defn sign (n)
  (cond
    ((> n 0) 1)
    ((< n 0) -1)
    (else 0)))

(print "Sign of 15:")
(print (sign 15))                  ; 1

(print "Sign of -8:")
(print (sign -8))                  ; -1

(print "Sign of 0:")
(print (sign 0))                   ; 0

; Maximum of three numbers
(defn max3 (a b c)
  (if (> a b)
      (if (> a c) a c)
      (if (> b c) b c)))

(print "Max of 3, 7, 5:")
(print (max3 3 7 5))               ; 7

; Leap year checker
(defn leap-year? (year)
  (cond
    ((= (mod year 400) 0) true)
    ((= (mod year 100) 0) false)
    ((= (mod year 4) 0) true)
    (else false)))

(print "Is 2020 a leap year?")
(print (leap-year? 2020))          ; true

(print "Is 1900 a leap year?")
(print (leap-year? 1900))          ; false

; Guards in recursive functions
(defn factorial-safe (n)
  (cond
    ((< n 0) "Error: negative number")
    ((= n 0) 1)
    ((= n 1) 1)
    (else (* n (factorial-safe (- n 1))))))

(print "Safe factorial of 5:")
(print (factorial-safe 5))         ; 120

(print "Safe factorial of -3:")
(print (factorial-safe -3))        ; Error: negative number