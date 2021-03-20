Derivation Tree:
A derivation tree or pase tree is an ordered rooted tree that
graphically represents the semantic information of strings
derived from a context free grammar.

Example:
Grammar G = {V, T, P, S}
where
V = non terminal symbols
T = terminal symbols
P = set of production rules
S = start symbol

          P = {S -> 0B, A -> 1AA|ε, Β -> 0ΑΑ}

          every leaf must be a terminal or ε

            S
           / \
          0   B
             /|\
            0 A A
             /|\ \
            1 A A ε
              | |
              ε ε
