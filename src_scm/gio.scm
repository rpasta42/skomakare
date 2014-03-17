;TODO: add regex
(load "db.scm")
(load "utils.scm")

;deleteme
;(define (g)
;  (load "gio.scm")
;  (blend-file 'arr 2 'str 10))
(define (get-read-proc type)
  (when (symbol? type) (set! type (symbol->string type)))

  (cond ((string=? type "int")      c/ptr->int)
	((string=? type "uint")     c/ptr->uint)
	((string=? type "float")    c/ptr->float)
	((string=? type "short")    (lambda (addr size) (c/ptr->uint addr 2)))
	((string=? type "uint64_t") (lambda (addr size) (c/ptr->uint addr 8)))
	((string=? type "str")      c/ptr->str)
	((string=? type "char")     c/ptr->char)
	((string=? type "ptr")     (lambda (ptr size) (cons ptr size)))
	(else (error (string-append type " is bad type")))))

(define (pointer-reader address)
  (define index 0)

  ;read array of num elements
  (define (next-arr num type type-size)
    (define read (get-read-proc type))

    (define (rec buff num-left)
      (if (= num-left 0)
	  buff
	  (let ((data (read (+ address index) type-size)))
	    (set! index (+ index
			    (if (and (eq? type 'str) (= type-size 0))
				(string-length data)
				type-size)))
	    (rec (cons data buff) (- num-left 1)))))

    (rec '() num))

  (define (next type . type-size)
    (if (null? type-size)
	(car (next-arr 1 type 0))
	(car (next-arr 1 type (car type-size)))))
  
  (define (skip n)
    ;(p "skipping")
    (set! index (+ index n)))

  ;arr=read array; skip=skip bytes; address=original; value=current
  (class 'next next 'arr next-arr 'skip skip
    'address (lambda () address)
    'value (lambda () (+ address index))
    'in (lambda () index)))


;(define (gio-open-file file-name size)
;  (define port (open-file file-name "rb"))
;  (define ptr (c/read-port-to-ptr port size))
;  ptr)

(define (filename->ptr path)
  (c/file path)
  ;(define s   (symbol->string (read (open-file path "r"))))
  ;(define len (string-length s))
  ;(c/str->ptr s len))
   ;(substring s 0 (- len 2))))
  )

;(define blend-file (gio-open "untitled.blend" 5000))
;(define vertex-shader (gio-open "shader.vert"))



