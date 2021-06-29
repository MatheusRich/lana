# Lana lang

A LISP built in Rust. I followed [Stepan Parunashvili's RISP tutorial][risp], but borrowed a few
concepts from [Clojure][clojure].

## Features

### Improved REPL

Lana's REPL is a bit more powerful than RISP's. It has

#### Input provided by ReadLine

[ Add gif here ]

#### Colored output

[ Add gif here ]

### Macros (coming soon)

## Syntax

### `nil`

Lana has a null value called `nil`.

```clojure
(nil? nil)
;; => true

(some? nil)
;; => false
```

### `if`

Macro for evaluating a conditional. All values are accepted as a condition, `false` and `nil` are
**the only** falsey values.

```clojure
(if true true false)
;; => true

(if 0 true false)
;; => true

(if false true false)
;; => false

(if nil true false)
;; => false
```

### `do`

Macro for evaluating expressions in order and returns the value of the last one.

```clojure
(do
  (def x 40)
  (def y 2)
  (+ x y))
;; => 42
```

### `defn`

Syntax sugar for `def` + `fn`.

```clojure
(defn fib (n)
    (if (<= n 2)
        n
        (+ (fib (- n 1)) (fib (- n 2)))))

(fib 10)
;; => 89
```

## Examples

[risp]: https://stopa.io/post/222
[clojure]: https://clojure.org/
