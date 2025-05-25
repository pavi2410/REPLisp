(do
  (print "Circle area calculator")
  
  (print "Defining the areacircle function...")
  (def {areacircle} (fun {rad} {do
    (print "Calculating area for radius:" rad)
    (* 314 rad rad 1 100)
  }))
  
  (print "Calculating area for radius 10:")
  (print (areacircle 10))
  
  (print "Calculating area for radius 5:")
  (print (areacircle 5))
)
