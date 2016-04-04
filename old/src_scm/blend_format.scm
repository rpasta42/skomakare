(load "utils.scm")

;filt structs with at least 1 field with length >n+1
(define (filt n)
  (filter (lambda (strc)
	    (> (length
		(filter (lambda (field) (> (length field) (+ 1 n)))
			(cdr strc)))
	       0))
	  sdna))

(define (merge-strs x)
  ;check if every element of list is string
  (define (each-str? lst) (every? lst string?))
  (if (list? x)
      (if (each-str? x)
	  (fold "" string-append x)
	  (map merge-strs x))
      x))

(define (rm-blank-strs x)
  (cond ((list? x)   (map rm-blank-strs x))
	((string? x) (string-filter (lambda (char) (not (eq? char #\x00))) x))
	(else x)))

(define (find-0.26922 x)
  (cond ((list? x)   (map find-0.26922 x))
	((number? x) (about-eq x 0.22656))
	(else #f)))

(define (is-there-0.26922 x)
  (< 0 (length (filter (lambda (x) (eq? x #t))
		       (flatten (find-0.26922 x))))))

(define data-file (open-file "../data/sexps_blend_data.lisp" "r"))
(define data (read data-file))
(define data2 (rm-blank-strs (merge-strs data)))

(define sdna-file (open-file "../data/sexps_sdna.lisp" "r"))
(define sdna (read sdna-file))

(define blocks-file (open-file "../data/sexps_blocks.lisp" "r"))
(define blocks (read blocks-file))

(define (get-sdna-by-name name)
  (search sdna
	  (lambda (strc) (string=? (car strc) name))))

(define (get-sdna-by-block-index index)
  (list-ref sdna
	    (list-ref (list-ref blocks index) 3)))

(define each-blk-sdna-name
  (map (lambda (x) (car (list-ref sdna (list-ref x 3)))) blocks))
;(filter (lambda (x) (string=? (symbol->string x) "MVert")) each-blk-sdna-name)

(define vert-block-index (car
  (indexes each-blk-sdna-name
	   (lambda (x) (string=? (symbol->string x) "MVert")))))

(define verts-file (open-file "../data/floats.file" "w"))

(define verts
  (list-ref data vert-block-index))

;(display verts verts-file)
;(write verts verts-file)

(define (rec-verts lst)
  (if (null? lst)
      'ok
      (let ((v (car lst)))
	(map (lambda (x) (display x verts-file) (display " " verts-file)) (car  v))
	(map (lambda (x) (display x verts-file) (display " " verts-file)) (cadr v))
	(rec-verts (cdr lst)))))

(rec-verts (cddr verts))
(flush-all-ports)

(define offsets  ;flat offset
  (filter-pos (lambda (x) (not (eq? x #f)))
	      (flatten (find-0.26922 data2))))
(define offsets1 ;which data block
  (map (lambda (x) (flat-offset-to-nested-offset 1 x data2))
       offsets))
(define offsets2 ;which Object field
  (map (lambda (x) (flat-offset-to-nested-offset 2 x data2))
       offsets))
(define offsets3
  (map (lambda (x) (flat-offset-to-nested-offset 3 x data2))
       offsets))

;(61034 60985 60933 58174 58171 58168 36813) ;offsets
;(754 754 754 727 727 727 374)    ;depth 1
;(65 62 48 14 13 12 26)           ;depth 2
;(3 3 1 1 1 1 3)                  ;depth 3
;(2 1 0 0 0 0 1)                  ;depth 4


(define obj-sdna
  (map get-sdna-by-block-index
       offsets1)) ;(range 0 (length blocks))))

(define obj-sdna-name (map car obj-sdna))
(define obj-sdna-val  (map cdr obj-sdna))

;(define obj-data
;  (map (lambda (off)
;	 (let ((tmp (list-ref data2 off)))
;	   (if (eq? 'a (car tmp))
;	       (cdr tmp)
;	       (list 1 tmp))))
;       offsets1))

;(map obj-data)

;(define obj-sdna (rec-obj-sdna num-objs))

(define (merge a b) (map (lambda (n m) (list n m)) a b))

;(define vert (merge obj-sdna obj-data))

;x  1.61864, y  4.96090, z  1.44427
;x  0.64827, y -1.15880, z -2.31872

;x -0.22656
;y -0.09375
;z  0.91602
