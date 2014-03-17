(define (l2) (load "utils.scm") (print-header))

(load "db.scm")

(use-modules (ice-9 format))
(use-modules (ice-9 regex))

;;;MISC
(define sm string-match)

(define (car-no-err x) (if (null? x) #f (car x)))
(define (cdr-no-err x) (if (null? x) #f (cdr x)))
(define (car-or-self x) (if (pair? x) (car x) x))
(define (cdr-or-self x) (if (pair? x) (cdr x) x))
(define (cdr-or-singleton x) (if (pair? x) (cdr x) x))
(define (car-or-singleton x) (if (pair? x) (car x) x))
(define (car-if-singleton x)
  (if (and (list? x) (= (length x) 1))
      (car x)
      x))

(define (self-or-singleton x) (if (list? x) x (list x)))

;last element in list
(define (last x) (if (null? (cdr x)) (car x) (last (cdr x))))

(define (about-eq a b)
  (< (abs (- a b)) 0.001))
;;;END MISC

;filter and return positions instead of elements 
(define (filter-pos pred? lst)
  (define (rec lst buff curr)
    (cond ((null? lst)       buff) 
	  ((pred? (car lst)) (rec (cdr lst) (cons curr buff) (+ curr 1)))
	  (else              (rec (cdr lst) buff             (+ curr 1)))))
  (rec lst '() 0))


;(comb acc (car lst))
(define (fold acc comb lst)
  (if (null? lst)
      acc 
      (fold (comb acc (car lst)) comb (cdr lst))))

(define (cdr-n lst x)
  (if (> x 0)
      (cdr-n (cdr lst) (- x 1))
      lst))

;returns offset of first (good? element)
(define (index lst good?)
  (define (rec lst i)
    (if (good? (car lst))
	i
	(rec (cdr lst) (+ i 1))))
  (rec lst 0))

(define (indexes lst good?)
  (define (rec lst acc i)
    (cond ((null? lst) acc)
	  ((good? (car lst))
	   (rec (cdr lst) (cons i acc) (+ i 1)))
	  (else
	   (rec (cdr lst) acc (+ i 1)))))
  (rec lst '() 0))


;faster than (car (filter good? lst))
(define (search lst good?)
  (cond ((null? lst) lst)
	((good? (car lst)) (car lst))
        (else (search (cdr lst) good?))))

(define (contains? elem lst eq-f)
  (cond ((null? lst) #f)
	((eq-f elem (car lst)) #t)
	(else (contains? elem (cdr lst) eq-f))))

;checks that every element if lst passes check
(define (every? lst check)
  (= (length lst) (length (filter check lst))))

;(a b . c) => (a b c)
(define (to-0-term-lst lst len)
  (if (= len 1)
      (if (pair? lst)
	  lst
	  (cons lst '()))
      (cons (car lst) (to-0-term-lst (cdr lst) (- len 1)))))

(define (sum lst)
  (if (pair? lst)
    (fold + 0 lst)
    lst))

;action takes current n
(define (repeat n action)
  (let ((x (action n)))
    (if (= n 1)
	x
	(repeat (- n 1) action))))

(define rand* 0)
(define (rand)
  (set! rand* (+ rand* 1))
  rand*)

(define (flatten x)
  (if (list? x)
      (apply append (map flatten x))
      (list x)))

(define (flat-length lst)
  (length (flatten (self-or-singleton lst))))


;depth offset list
(define (flat-offset-to-nested-offset d o l)
  (define (flat-offset-to-top-offset flat-len lst)
    (define (rec top-len flat-len lst)
      (let* ((car-len (flat-length (car-or-singleton lst)))
	     (new-flat-len (- flat-len car-len)))
	(if (<= new-flat-len 0)
	    top-len
	    (rec (+ top-len 1) new-flat-len (cdr-or-singleton lst)))))
    (rec 0 (+ flat-len 1) lst))
  (define (rec lst cd co)
    (cond ((= cd d)
           (flat-offset-to-top-offset (- o co) lst))
          ((< cd d)
           (let* ((flen (flat-length (car lst)))
                  (newo (+ co flen)))
             (if (<= newo o)
                 (rec (cdr lst) cd newo)
                 (rec (car lst) (+ cd 1) co))))
          ((> cd d) (error ".."))))
  (rec l 1 0))

(define (num-range from to)
  (define (rec lst curr)
    (if (= curr to)
	curr
	(rec (cons curr lst) (+ curr 1))))
  (rec '() from))

(define (list-range lst from to)
  (define (rec data from to)
    (if (= from to)
      data
      (rec (cons (list-ref lst from) data) (+ from 1) to)))
  (reverse (rec '() from to)))

;(map (lambda (x) (f 1 x l1)) (range 0 (- (len l1) 1)))


(define (print str)
  (cond ((string? str) (format #t str))
    ((number? str) (format #t (number->string str)))
    ((symbol? str) (format #t (symbol->string str)))))

(define (seps sep lst)
  (when (and (not (null? lst)) (null? (cdr lst)))
    (set! sep ""))
  (if (null? lst)
      '()
      (cons (car lst) (cons sep (seps sep (cdr lst))))))

(define (p . strs)
  (if (null? strs)
      'success
      (begin (if (pair? (car strs))
		 (set! strs (append '(0 "(") (seps " " (car strs)) '(")") (cdr strs)))
		 (print (car strs)))
	     (apply p (cdr strs)))))

(define (print-seps sep strs)
  (for-each (lambda (s) (p s) (p sep)) strs))


;(define (print . strs)
;  (define (print-convert str)
;    (cond ((string? str) (format #t str))
;          ((number? str) (format #t (number->string str)))
;          ((symbol? str) (format #t (symbol->string str)))))
;  (define (print-lst strs)
;    (for-each
;        (lambda (s) (print-convert s) (print-convert (car strs)))
;        (cdr strs)))
;  (if (not (pair? (car strs))) (print-convert (car strs))
;  (if (null? (cdr strs))
;    (print-lst (car strs))
;    (for-each (lambda (s) (print s) (print-convert (car strs))) (cdr strs)))))

;check that status != 'stat-bad
(define (assert status)
  (when (eq? status 'stat-bad) (error "failure"))
  status)

(define (stat-ok-f x) 'stat-ok)
(define (identity . x) (car x))

;;-------------------------------------

(define dd (database))

(define (put op type item)
  (when (eq? (dd type) 'not-found)
    (dd 'add type (database)))
  (dd type 'add op item))

(define (get op type)
  (dd type op))

;(define (arr))
;(arr)
(define (install-int)
  (put '+ 'int +)
  (put '- 'int -)
  (put '* 'int *))

(define (install-list)
  (define (list+ . nums)
    (apply append nums))
  (define (list- lst elem)
    (filter (lambda (x) (not (= x elem)))
	    lst))
  (define (list* lst scalar)
    (if (= scalar 1)
	lst
	(append lst (list* lst (- scalar 1)))))
  (put '+ 'lst list+)
  (put '- 'lst list-)
  (put '* 'lst list*))

(define (num)
  (install-int)
  (install-list))

(num)

(define (tag mark val) (cons mark val))
(define (get-val tagged) (cdr tagged))
(define (get-tag tagged) (car tagged))
