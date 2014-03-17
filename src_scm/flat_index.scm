#lang racket

;(flatten '(1 2 (3 4 (5 6) 7 8) 9))
(define (flatten x)
  (if (pair? x)
      (apply append (map flatten x))
      (list x)))

(define (car-or-self x)
  (if (pair? x)
      (car x)
      x))

(define (cdr-or-self x)
  (if (pair? x)
      (cdr x)
      x))


(define (car-or-singleton x)
  (if (pair? x)
      (car x)
      (list x)))

(define (cdr-or-singleton x)
  (if (pair? x)
      (cdr x)
      (list x)))

(define (self-or-singleton x)
  (if (list? x) x (list x)))

(define (len lst)
  (length (flatten (self-or-singleton lst))))

(define (f1 d o l)
  (define (rec lst cd co cdo)
    (cond ((= co o)
           (cond ((= cd d) cdo)
                 ((< cd d) (format "HI") 0)
                 (else (error "1"))))
          ((< co o)
           (let* ((flen (len (car lst)))
                   (newo (+ co flen)))
              (cond ((< newo o)
                     (rec (cdr-or-self lst) cd newo (+ cdo 1)))
                    ((> newo o)
                     (cond ((= cd d) cdo)
                           ((< cd d) (rec (car-or-self lst) (+ cd 1) co 0))
                           ((> cd d) (error "2"))))
                    ((= newo o)
                     (cond ((= cd d) cdo)
                           ((< cd d) (rec (car-or-self lst) (+ cd 1) co 0))
                           ((> cd d) (error "2")))))))
          ((> co o)
           (error "3"))))
  (rec l 1 0 0))

(define (cdr-offset lst x)
  (if (> x 0)
      (cdr-offset (cdr lst) (- x 1))
      lst))

;(f-help 5 '((3 3) (4 43 2) (4 3 2)))
(define (f-help flat-len lst) ;get top-level num
  (define (rec top-len flat-len lst)
    (let* ((car-len      (len (car-or-singleton lst)))
           (new-flat-len (- flat-len car-len)))
      (if (<= new-flat-len 0)
          top-len
          (rec (+ top-len 1) new-flat-len (cdr-or-singleton lst)))))
  (rec 0 (+ flat-len 1) lst))

(define (f d o l)
  (define (rec lst cd co)
    (cond ((= cd d)
           (f-help (- o co) lst))
          ((< cd d)
           (let* ((flen (len (car lst)))
                   (newo (+ co flen)))
              (cond ((< newo o)
                     (rec (cdr lst) cd newo))
                    ((or (> newo o) (= newo o))
                     (rec (car lst) (+ cd 1) co)))))
          ((> cd d)
           (error "no"))))
  (rec l 1 0))
    
(define l1 '(1 5 x 6 (x (7 x)) 8 (x (x))))
(define l2 '(1 5 (x (2 (9 6) (3) 5)) x x x))

(define (range from to)
  (if (= from to) (list from) (cons from (range (+ from 1) to))))

;(map (lambda (x) (f 1 x l1)) (range 0 (- (len l1) 1)))
(f 2 4 l1) ;0
(f 2 5 l1) ;1
(f 2 6 l1) ;1
(f 2 8 l1) ;0
(f 2 9 l1) ;1

(f 3 6 l2) ;2


(define asdfs '(define (f1 d o l)
  (define (rec lst cd co cdo)
    (cond ((< co o)
           (let* ((flen (length (flatten (car-or-singleton lst))))
                  (newo (+ co flen)))
             (cond ((< newo o)
                    (rec (cdr-or-singleton lst) cd newo (+ cdo 1)))
                   ((> newo o)
                    (rec (car lst) (+ cd 1) co 0))
                   (else (cond ((< cd d) 0)
                               ((= cd d) cdo)
                               (else (error "5")))))))
                   ;(else (error "1")))))
          ((= co o)
           (cond ((< cd d) 0)
                 ((= cd d) cdo)
                 (else (error "2"))))
          (else (error "3"))))
  (rec l 1 0 0)))



           
                 