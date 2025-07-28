(print "Testing special forms:")

; Test lambda
((lambda (x y) (+ x y)) 5 3)

; Test do
(do 
  (print "first")
  (print "second")
  42)

; Test if
(if true "yes" "no")
(if false "yes" "no")
(if false "yes")

; Test cond
(cond 
  (false "nope")
  ((= 2 2) "found it")
  (else "default"))