; =====
; Can two positive numbers be recovered from their sum, difference,
; product, and quotient?
;
; This is my first attempted solution using plain SMT2 syntax
; and arrays, but it doesn't work
; (at least, Z3 does not find a solution in 10 minutes or so).
; The other solution in `four-numbers.py` using itertools to iterate
; over permutations rather than encoding the sets of four numbers
; as bags is fast (under a second).
;
; Problem statement:
; For two positive numbers x and y, you are given four numbers
; {a,b,c,d} and informed that they are the values of x+y, x-y, xy,
; and x/y (in an unknown order). Show that this is enough
; information to determine the values of both x and y.
;
; This is an SMT2 file. Install Z3 to run:
;     https://github.com/Z3Prover/z3
; =====

; Consider two possible pairs x1, y1 and x2, y2

(declare-fun x1 () Real)
(declare-fun y1 () Real)
(assert (> x1 0))
(assert (> y1 0))

(declare-fun x2 () Real)
(declare-fun y2 () Real)
(assert (> x2 0))
(assert (> y2 0))

(assert (or
    (not (= x1 x2))
    (not (= y1 y2))
))

; Make the bags {x + y, x - y, xy, x/y} and assert equal

(define-sort Bag () (Array Real Int))
(define-fun bag-union ((x Bag) (y Bag)) Bag
    ((_ map (+ (Int Int) Int)) x y))

(declare-fun Bempty () (Array Real Int))
(assert (= Bempty ((as const (Array Real Int)) 0)))

(declare-const B1 (Array Real Int))
(assert (= B1 (bag-union
    (bag-union
        (store Bempty (+ x1 y1) 1)
        (store Bempty (- x1 y1) 1)
    )
    (bag-union
        (store Bempty (* x1 y1) 1)
        (store Bempty (/ y1 x1) 1)
    )
)))

(declare-const B2 (Array Real Int))
(assert (= B2 (bag-union
    (bag-union
        (store Bempty (+ x2 y2) 1)
        (store Bempty (- x2 y2) 1)
    )
    (bag-union
        (store Bempty (* x2 y2) 1)
        (store Bempty (/ y2 x2) 1)
    )
)))

; Declare the bags to be equal and check satisfiability

(assert (= B1 B2))

(check-sat)
(get-model)
