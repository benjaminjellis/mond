# Match

You can pattern match with the keyword `match`. Use `~>` to separate each pattern from its result. `_` is a wildcard.

Pattern matching works on single variables

```mond
(let describe {n}
  (match n
    0 ~> "zero"
    1 ~> "one"
    _ ~> "many"))
```


... on multiple variables

```mond
(let two_values {x y}
  (match x y
    10 12 ~> (io/println "matched")
    _ _ ~> (io/println "not matched")))
```

or on lists with the cons operator

```mond
(let iterate {list}
  (match list
    [] ~> (io/println "empty")
    [h | t] ~> (do (io/debug h)
                   (iterate t))))

```

For patterns with multiple cases on one branch you can use `or`

```mond
(let is_weekend {day}
  (match day
    "Saturday" or "Sunday" ~> True
    _                      ~> False))
```


The list example above also introduces the `do` keyword. In some places, the compiler is expecting only one expression but you might like to do more. This gives us an opportunity to demonstrate something else about `Mond`: the friendly compiler errors. You may be tempted not to use `do`. You could write the example above as:


```mond
(let iterate {list} 
  (match list 
    [] ~> (io/println "empty")
    [h | t] ~> (
                (io/debug h)
                (iterate t))))
```

But if you try to compile this, the compiler would say:

```shell
error: type mismatch: expected `Unit`, found `('a -> 'b)`
  ┌─ main.mond:6:85
  │
6 │ (let iterate {list} (match list [] ~> (io/println "empty") [h | t] ~> ((io/debug h) (iterate t))))
  │                                                                                     ^^^^^^^^^^^ this argument has type `'a`
  │
  = expected `Unit`, found `('a -> 'b)`
  = hint: `Unit` is not a function — if you meant to sequence multiple expressions, use `(do expr1 expr2 ...)`
```

