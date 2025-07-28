(defn test-lambda ()
  ((lambda (x) (+ x undefined_var)) 42))

(test-lambda)