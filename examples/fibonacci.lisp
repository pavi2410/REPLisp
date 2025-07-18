; Fibonacci sequence implementations
; This example shows different approaches to computing Fibonacci numbers

(print "=== Fibonacci Sequence ===")

; Basic recursive implementation
(defn fib-recursive (n)
  (if (<= n 1)
      n
      (+ (fib-recursive (- n 1)) (fib-recursive (- n 2)))))

(print "Recursive Fibonacci:")
(print (fib-recursive 0))          ; 0
(print (fib-recursive 1))          ; 1
(print (fib-recursive 5))          ; 5
(print (fib-recursive 10))         ; 55

; Iterative implementation
(defn fib-iter (n a b count)
  (if (= count 0)
      a
      (fib-iter n b (+ a b) (- count 1))))

(defn fib-iterative (n)
  (fib-iter n 0 1 n))

(print "Iterative Fibonacci:")
(print (fib-iterative 10))         ; 55
(print (fib-iterative 15))         ; 610

; Generate Fibonacci sequence as a list
(defn fib-sequence (n)
  (defn fib-helper (count a b acc)
    (if (= count 0)
        acc
        (fib-helper (- count 1) b (+ a b) (cons a acc))))
  (reverse (fib-helper n 0 1 '())))

(print "First 10 Fibonacci numbers:")
(print (fib-sequence 10))          ; (0 1 1 2 3 5 8 13 21 34)

; Tail-recursive Fibonacci
(defn fib-tail (n)
  (defn fib-tail-helper (n a b)
    (if (= n 0)
        a
        (fib-tail-helper (- n 1) b (+ a b))))
  (fib-tail-helper n 0 1))

(print "Tail-recursive Fibonacci:")
(print (fib-tail 12))              ; 144

; Check if a number is a Fibonacci number
(defn is-fibonacci? (num)
  (defn check-fib (n a b)
    (cond
      ((= a num) true)
      ((> a num) false)
      (else (check-fib (+ n 1) b (+ a b)))))
  (if (= num 0)
      true
      (check-fib 1 0 1)))

(print "Is 21 a Fibonacci number?")
(print (is-fibonacci? 21))         ; true

(print "Is 20 a Fibonacci number?")
(print (is-fibonacci? 20))         ; false

; Sum of first n Fibonacci numbers
(defn sum-fib (n)
  (defn sum-helper (count a b sum)
    (if (= count 0)
        sum
        (sum-helper (- count 1) b (+ a b) (+ sum a))))
  (sum-helper n 0 1 0))

(print "Sum of first 10 Fibonacci numbers:")
(print (sum-fib 10))               ; 88