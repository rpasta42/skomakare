(sleep 3)
(load "utils.scm")
(define (o) (load "ogre.scm"))
 

;(define x1 (x '((0))))

;(define y (filter (lambda (n) (not (list? n)))
;		  (map (lambda (n) (cdr n)) x1)))

;(fold acc (lambda (acc x)) lst)

(define (replicate lst n)
  (define (rec curr ret)
    (if (= curr n)
	ret
	(rec (+ curr 1) (append ret lst))))
  (rec 0 '()))

;(define y (replicate y 20))

(define (enumerate lst)
  (fold '((0 1))
	(lambda (acc curr)
	  (cons (cons (+ (caar acc) 1.0) curr) acc))
	lst))

(define (go)
  (define n (create-child-scene-node (get-root-scene-node) "abc"))
  (define m (create-entity "s" "ogrehead.mesh"))
  (attach-object n m)
  (set-position n 0 0 100))

;(define (da)
;  (for-each (lambda (v)
;	      (begin (usleep 30000)
;		     (when (not (= (car v) 0))
;		       (set_position n (/ (car v) 10) 0 (* 10 (cdr v))))))
;	    x))


(define (step x) (cons (+ (car x) 5) (cos (/ (car x) 100))))
(define (iter curr) (if (> (caar curr) 630) curr (iter (cons (step (car curr)) curr))))
(define list (map cdr (iter (list (cons 0 1))))) ;[0.99 0.97 ... 0 -0.9 ... 0 1]
(define list (replicate list 10))
(define list (enumerate list)) ;[(128 0.99) .. (0 1)]

(define (da)
  (for-each (lambda (v)
	      (begin (usleep 30000)
		     (set-position n (/ (car v) 5) 0 (* 10 (cdr v)))))
	    list))

;(go)
