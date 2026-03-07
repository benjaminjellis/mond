(use std)
(use std/result)

(pub let test_length {}
  (let? [_ (test/assert_eq (string/length "hello") 5)]
    (test/assert_eq (string/length "") 0)))

;; (pub let test_is_empty {}
;;   (let? [_ (test/assert (string/is_empty ""))]
;;     (test/assert_ne (string/is_empty "hi") True)))
;;
;; (pub let test_trim {}
;;   (test/assert_eq (string/trim "  hello  ") "hello"))
;;
;; (pub let test_uppercase {}
;;   (test/assert_eq (string/uppercase "hello") "HELLO"))
;;
;; (pub let test_lowercase {}
;;   (test/assert_eq (string/lowercase "HELLO") "hello"))
;;
;; (pub let test_casefold {}
;;   (test/assert_eq (string/casefold "HELLO") "hello"))
;;
;; (pub let test_concat {}
;;   (test/assert_eq (string/concat "hello" " world") "hello world"))
;;
;; (pub let test_contains {}
;;   (let? [_ (test/assert (string/contains "hello world" "world"))]
;;     (test/assert_ne (string/contains "hello world" "xyz") True)))
;;
;; (pub let test_split {}
;;   (test/assert_eq (string/split "a,b,c" ",") ["a" "b" "c"]))
