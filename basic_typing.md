```console
  term ::=                      terms:
  true                          constant true
  false                         constant false
  if term then term else them   conditional
  0                             constant zero
  succ term                     successor
  pred term                     predecessor
  is_zero term                  zero test

  value ::=                     values:
  true                          true value
  false                         false value
  numeric_value                 numeric value

  numeric_value ::=             numeric values:
  0                             zero value
  succ numeric_value            successor value

  typed booleans
  T ::=                         types:
  Bool                          type of booleans

  typing rules                  t : T
  true : Bool                   (T-True)
  false : Bool                  (T-False)

  t1 : Bool  t2 : T    t3 : T   (T-If)
  --------------------------
  if t1 then t2 else t3 : T

  typed numbers                 t : T
  T ::=                         types:
  Nat                           type of natural numbers

  typing rules
  0 : Nat                       (T-Zero)

    t1 : Nat                    (T-Succ)
  -------------
  succ t1 : Nat

    t1 : Nat                    (T-Pred)
  -------------
  pred t1 : Nat

      t1 : Nat                  (T-IsZero)
  ----------------
  is_zero t1 : Bool
```
