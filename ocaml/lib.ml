type term = 
  True 
| False 
| If of term * term * term
| Zero
| Succ of term
| Pred of term
| IsZero of term

let rec is_numeric_val term = match term with 
  Zero -> true 
| Succ(term) -> is_numeric_val term
| Pred(term) -> is_numeric_val term
| _ -> false

let rec eval term = match term with 
  If(True, consequence, _) -> eval(consequence)
| If(False, _, alternative) -> eval(alternative)
| If(condition, consequence, alternative) -> eval(If(eval(condition), consequence, alternative))
| Succ(term) -> Succ(eval term)
| Pred(Zero) -> Zero 
| Pred(Succ(term)) when is_numeric_val term -> term
| IsZero(Zero) -> True 
| IsZero(Succ(term)) when is_numeric_val term -> False 
| IsZero(term) -> eval (IsZero (eval term))
| _ -> term

let rec to_string term = match term with 
  True -> "true"
| False -> "false"
| If(condition, consequence, alternative) -> Printf.sprintf "If(%s, %s, %s)" (to_string condition) (to_string consequence) (to_string alternative)
| Zero -> "zero"
| Succ(term) -> Printf.sprintf "Succ(%s)" (to_string term)
| Pred(term) -> Printf.sprintf "Pred(%s)" (to_string term)
| IsZero(term) -> Printf.sprintf "IsZero(%s)" (to_string term)

let () =
  print_endline (string_of_bool (is_numeric_val Zero));

  print_endline (string_of_bool (is_numeric_val (Succ(Zero))));

  print_endline (string_of_bool (is_numeric_val (Pred(Succ(Zero)))));

  print_endline (to_string (eval True));

  print_endline (to_string (eval False));

  print_endline (to_string (eval (IsZero(Zero)))); 

  print_endline (to_string (eval (IsZero(Succ(Zero)))));

  print_endline (to_string (eval (Pred(Succ(Zero)))));

  print_endline (to_string (eval (IsZero(Pred(Succ(Zero))))));

  print_endline (to_string (If(IsZero(Pred(Succ(Zero))), Succ(Succ(Zero)), Zero)));

  print_endline (to_string (eval (If(IsZero(Pred(Succ(Zero))), Succ(Succ(Zero)), Zero))));

