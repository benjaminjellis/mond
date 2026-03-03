# Opal
Opal is an experimental lisp language

Below is a sample program

```
;; custom record type
(type MyType (
  (field_one ~ String)
  (field_two ~ Int)
  (field_three ~ Bool)
))

;; custom variant types
(type ['e 'a] Result
  (Error ~ 'e)
  (Ok ~ 'a))

(type ['a] Option
  None
  (Some ~ 'a))

(type MyOtherType
  VariantOne
  (VariantTwo ~ String))

;; a function
(let add_three [a b c]
  ;; this is a comment
  (let [intermediate (+ a b)
        final (+ intermediate c)]
    final))

(let division [a b]
  (if (= b 0)
    None
    (Some (/ a b))))

;; a recursive vie function
(let rec fib [n]
  (if (or (= n 0) (= n 1))
    n
    (+ (fib (- n 1)) (fib (- n 2)))))

(let demo []
  (let [v [1 2 3]
        t VariantOne]
    (if true
      (match t
        VariantOne "ok"
        (VariantTwo msg) (str "msg=" msg))
      "missing")))
```


TODO list 
[ ] parser (recursive descent)
