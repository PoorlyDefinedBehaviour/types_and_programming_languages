```console
Set of simple types over the type Bool:

T ::=               types:
  Bool              type of booleans
  T -> T            type of functions

  -> is a right associative type constructor -- that is, the expression T1 -> T2 -> T2 stands for T1 -> (T2 -> T3).

  T1 -> T2 -> T3 == T1 -> (T2 -> T3)
  (T1 -> T2) -> (T3 -> T4) == (T1 -> T2) -> T3 -> T4

  \x: T1. t2 -- x has type T1

  Since we know the type of the argument to the abstraction, the function return type will be the type of t2.

      x: T1 ͱ t2: T2
  -----------------------   (T-Abs)
  ͱ \x: T1. t2 : T1 -> T2


  Abstraction typing rule with type enviroment which is basically a function Variable -> Type

    Γ, x: T1 ͱ t2: T2
  -------------------------
  Γ ͱ \x: T1. t2 : T1 -> T2

  Variable typing rule, variable has whatever type the environment says it has.

  x: T ∈ Γ
  ---------   (T-Var)
  Γ ͱ x: T

  Application typing rule, the type of the result of applying T11 to an abstraction of type T11 -> T22 will be T22.

  Γ ͱ t1: T11 -> T22    Γ  t2 : T11
  ---------------------------------   (T-App)
        Γ ͱ t1 t2 : T12

  If typing rule, given a t1 of type Bool, a t2 of type T, and a t3 of type T, the type of T-If is T

  Γ ͱ t1: Bool    Γ ͱ t2: T   Γ ͱ t3:T
  ------------------------------------    (T-If)
      Γ ͱ if t1 then t2 else t3: T

  if true then (\x: Bool. x) else (\x: Bool. not x)
  -> (\x: Bool. x): Bool -> Bool

  Syntax
  t ::=         terms:
    x           variable
    \x: T. t    abstraction
    t t         application

  v ::=         values:
    \x: T. t    abstraction value

  T ::=         types:
    T -> T      type of functions

  Γ ::=         contexts:
    ∅           empty context
    Γ, x: T      term variable binding

  Evaluation                              t -> t'
      t1 -> t'1
    ---------------                       (E-App1)
    t1 t2 -> t'1 t2

      t2 -> t'2
    ---------------                       (E-App2)
    v1 t2 -> v1 t'2

    (\x: T11. t12) v2 -> [x |-> v2]t12    (E-AppAbs)

  Typing                              Γ ͱ t: T
  x: T ∈ Γ
  ---------                           (T-Var)
  Γ ͱ x: T

    Γ, x: T1 ͱ t2: T2
  -----------------------             (T-Abs)
  Γ ͱ \x: T1. t2: T1 -> T2

  Γ ͱ t1: T11 -> T12    Γ ͱ t2: T11
  ---------------------------------   (T-App)
          Γ ͱ t1 t2: T12

   x: Bool ∈ x: Bool
  ------------------- T-Var
   x: Bool ͱ x: Bool
  --------------------------- T-Abs --------------------------- T-True
  ͱ \x: Bool. x: Bool -> Bool                ͱ true: Bool
  ------------------------------------------------------------- T-App
               ͱ (\x: Bool. x) true : Bool

   x: Bool ∈ x: Bool
  ------------------- T-Var
   x: Bool ͱ x: Bool
  -------------------
  ͱ f: Bool -> Bool
  ---------------------------------- T-Abs
  ͱ \x: Bool. f(if x then false else x): Bool -> Bool
```
