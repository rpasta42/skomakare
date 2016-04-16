

(define (pointer ptr size)
  (define curr-read 0)

  ;type   = \char|(((u?int)|float)(8|16|32|64))\
  (define (read-array type n)
    (cond ((eq? type 'char) 

;format = [(type,num)|type]
(define (read arg)
  (if (symbol? arg)
      (car (read-array arg 1))
      (read-array (car arg) (cdr arg)))) ;assume list
  (class
    'set  (lambda (ptr* . size*)
	    (set! ptr ptr*)
	    (set! size size*))
    'read (lambda (format)
	    (if (
