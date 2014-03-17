;key-lookup table which can also be used for dispatch/OOP.

;(define db (database))

;(db 'add 'default "heyyo")
;(db 'add 'hey 69)
;(db 'add 'magic 42)
;(db 'add 'inc (lambda (x) (+ x 1)))

;(db)        -> "heyyo"
;(db 'magic) -> 42
;(db 'inc 4) -> 5


;------------------------------
;;;internal functions:
;dispatch*  - real dispatch-like search
;dispatch   - default return value from db. adds some default members (default, add, custom eq-func)
;
;;;default members:
;(index eq?* key) - (DELETEME) get element's place starting from 0
;(add key val)    - add a new pair
;(default)        - called if no args
;(list-mems)

;;;misc stuff
;class     - synonym for database
;db-add    - function
;db-lookup - function
;------------------------------

;(key1 val1 key2 val2)
(define (database . vals)

  (define (index eq?* key)
    (define (rec lst n)
      (cond ((null? lst) (error "not found"))
            ((eq?* (car lst) key) n)
            (else (rec (cddr lst) (+ n 1)))))
    (rec vals 0))

  (define (add key val)
    (set! vals (cons key (cons val vals))))

  (define (list-mems)
    (let rec ((curr '()) (left vals))
      (if (null? left)
          curr
          (rec (cons (car left) curr) (cddr left)))))

  (define (dispatch* vals eq?* key . args)
    (cond ((null? vals) (format #f "\ndb: member not found") 'not-found)
          ((eq?* key (car vals))
	   (if (null? args)
	       (cadr vals)
	       (apply (cadr vals) args)))
          (else (apply dispatch* (cddr vals) eq?* key args))))

  (define (dispatch . args)
    (cond ((null? args)
           (dispatch* vals eq? 'default))
          ((eq? (car args) 'add)
           (add (cadr args) (caddr args)))
          ((eq? (car args) 'eq-func)
           (apply dispatch* vals (cadr args) (cddr args)))
          ((eq? (car args) 'list)
           (list-mems))
	  ((eq? (car args) 'index)
	   (index))
          (else
           (apply dispatch* vals eq? args))))

  dispatch)

(define class database)

(define (db-add db key value)
  (db 'add key value))

(define (db-lookup db key . params)
  (if (null? params)
      (db key)
      (apply (db key) params)))

