**Definition**

A grammar is left-recursive if and only if there exists a nonterminal symbol A that can derive a sententional form with itself as the leftmost symbol.

A => Aα

**Direct left recursion**  
Direct left recursion occurs when the definition of left recursion can be satistified with only one substitution.

Example:

Expression => Expression + Term

How a left-to-right recursive descent parser would look like for this rule:

```rust
fn expression() {
  expression();
  match("+");
  term();
}
```

this code would fall into infinite recursion.

**Indirect left recursion**

Indirect left recursion occurs when the definition of left recursion is satified via several substitutions.

**Eliminating left recursion**

Given a left recursive grammar:  
A -> Aα | β

To eliminate left recursion, apply this transformation

A -> Aα | β  
A -> βA'  
A'-> αA' | ε

Examples:

E -> E+T | T  
E -> TE'  
E'-> +TE' | ε

A -> ABα | Aa | a  
A -> aA'  
A'-> BαA' | aA' | ε

A -> AC | Aad | bd | c  
A -> bdA' | cA'  
A'-> CA' | adA' | ε

Expression -> Expression + Term  
Expression -> ε Expression'  
Expression'-> + Term Expression' | ε
