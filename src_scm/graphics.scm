
(define (continuous num-times delay action)
  (if (= num-times 0)
    0
    (let ()
      (usleep delay)
      (action)
      (continuous (- num-times 1) delay action))))

(define (mv num-times x y z)
  (continuous num-times 10000 (lambda () (move 0 x y z))))
