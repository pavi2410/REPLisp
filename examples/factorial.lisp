; Factorial function implementation
; This example demonstrates recursion and conditionals

(print "=== Factorial Examples ===")

; Recursive factorial function
(defn factorial (n)
  (if (<= n 1)
      1
      (* n (factorial (- n 1)))))

; Test factorial function
(print "Factorial of 5:")
(print (factorial 5))              ; 120

(print "Factorial of 0:")
(print (factorial 0))              ; 1

(print "Factorial of 7:")
(print (factorial 7))              ; 5040

; Iterative factorial using a helper function
(defn factorial-iter (n acc)
  (if (<= n 1)
      acc
      (factorial-iter (- n 1) (* n acc))))

(defn factorial-iterative (n)
  (factorial-iter n 1))

(print "Iterative factorial of 6:")
(print (factorial-iterative 6))   ; 720

; Factorial table
(print "Factorial table:")
(defn print-factorial (n)
  (if (<= n 10)
      (do
        (print (list n "! =" (factorial n)))
        (print-factorial (+ n 1)))))

(print-factorial 1)

; Double factorial (n!!)
(defn double-factorial (n)
  (if (<= n 2)
      n
      (* n (double-factorial (- n 2)))))

(print "Double factorial of 8:")
(print (double-factorial 8))      ; 384 (8 * 6 * 4 * 2)