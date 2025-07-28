(defn deep1 (x)
  (deep2 x))

(defn deep2 (x)
  (deep3 x))

(defn deep3 (x)
  (+ x undefined_var))

(deep1 10)