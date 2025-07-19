; Simple calculator with basic operations
; This example demonstrates a practical use case

(print "=== Calculator Example ===")

; Basic calculator operations
(defn add (x y) (+ x y))
(defn subtract (x y) (- x y))
(defn multiply (x y) (* x y))
(defn divide (x y) 
  (if (= y 0)
      "Error: Division by zero"
      (/ x y)))

; Power function
(defn power (base exponent)
  (if (= exponent 0)
      1
      (if (= exponent 1)
          base
          (* base (power base (- exponent 1))))))

; Square root approximation using Newton's method
(defn sqrt-approx (x guess)
  (def new-guess (/ (+ guess (/ x guess)) 2))
  (if (< (abs (- guess new-guess)) 0.001)
      new-guess
      (sqrt-approx x new-guess)))

(defn sqrt (x)
  (if (< x 0)
      "Error: Cannot take square root of negative number"
      (sqrt-approx x 1.0)))

; Percentage calculation
(defn percentage (part whole)
  (* (/ part whole) 100))

; Simple interest calculation
(defn simple-interest (principal rate time)
  (* principal (/ rate 100) time))

; Compound interest calculation
(defn compound-interest (principal rate time)
  (* principal (power (+ 1 (/ rate 100)) time)))

; Calculator demonstrations
(print "Basic operations:")
(print (add 15 25))                ; 40
(print (subtract 100 37))          ; 63
(print (multiply 8 9))             ; 72
(print (divide 144 12))            ; 12
(print (divide 10 0))              ; Error: Division by zero

(print "Power calculations:")
(print (power 2 8))                ; 256
(print (power 5 3))                ; 125

(print "Square root approximations:")
(print (sqrt 16))                  ; ~4.0
(print (sqrt 25))                  ; ~5.0
(print (sqrt 2))                   ; ~1.414

(print "Percentage calculations:")
(print (percentage 25 100))        ; 25
(print (percentage 15 60))         ; 25

(print "Financial calculations:")
(print "Simple interest (P=1000, R=5%, T=2 years):")
(print (simple-interest 1000 5 2)) ; 100

(print "Compound interest (P=1000, R=5%, T=2 years):")
(print (compound-interest 1000 5 2)) ; 1102.5

; Area calculations
(defn circle-area (radius)
  (* 3.14159 radius radius))

(defn rectangle-area (length width)
  (* length width))

(defn triangle-area (base height)
  (/ (* base height) 2))

(print "Area calculations:")
(print (circle-area 5))            ; ~78.54
(print (rectangle-area 10 6))      ; 60
(print (triangle-area 8 5))        ; 20

; Unit conversions
(defn celsius-to-fahrenheit (celsius)
  (+ (* celsius (/ 9 5)) 32))

(defn fahrenheit-to-celsius (fahrenheit)
  (* (- fahrenheit 32) (/ 5 9)))

(defn miles-to-kilometers (miles)
  (* miles 1.60934))

(defn kilometers-to-miles (km)
  (/ km 1.60934))

(print "Unit conversions:")
(print (celsius-to-fahrenheit 25))     ; 77
(print (fahrenheit-to-celsius 77))     ; 25
(print (miles-to-kilometers 10))       ; 16.0934
(print (kilometers-to-miles 16.0934))  ; 10