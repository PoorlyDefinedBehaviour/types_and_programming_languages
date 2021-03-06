/**
 * term ::=
 *   0
 *   succ term
 *   pred term
 *   is_zero term
 *
 * value ::=
 *   numeric_value
 *  
 * numeric_value ::=
 *   0
 *   succ numeric_value
 */

#[derive(Debug)]
enum Term {
    True,
    False,
    If {
        condition: Box<Term>,
        consequence: Box<Term>,
        alternative: Box<Term>,
    },
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

fn is_numeric_value(term: &Term) -> bool {
    match term {
        Term::Zero => true,
        Term::Succ(term) => is_numeric_value(term),
        Term::Pred(term) => is_numeric_value(term),
        _ => false,
    }
}

fn eval(term: Term) -> Term {
    match term {
        Term::If {
            condition,
            consequence,
            alternative,
        } => match *condition {
            Term::True => *consequence,
            Term::False => *alternative,
            _ => {
                let evaluated_condition = eval(*condition);
                eval(Term::If {
                    condition: Box::new(evaluated_condition),
                    consequence,
                    alternative,
                })
            }
        },
        Term::Succ(term) => {
            let evaluated_term = eval(*term);
            Term::Succ(Box::new(evaluated_term))
        }
        Term::Pred(term) => match *term {
            Term::Zero => Term::Zero,
            Term::Succ(term) if is_numeric_value(&term) => *term,
            Term::Pred(term) => {
                let evaluated_term = eval(*term);
                eval(Term::Pred(Box::new(evaluated_term)))
            }
            term => eval(term),
        },
        Term::IsZero(term) => match *term {
            Term::Zero => Term::True,
            Term::Succ(term) if is_numeric_value(&term) => Term::False,
            term => {
                let evaluated_term = eval(term);
                eval(Term::IsZero(Box::new(evaluated_term)))
            }
        },
        _ => term,
    }
}

fn main() {
    println!("{:?}", eval(Term::True)); // True

    println!("{:?}", eval(Term::False)); // False

    println!("{:?}", eval(Term::IsZero(Box::new(Term::Zero)))); // True

    println!(
        "{:?}",
        eval(Term::IsZero(Box::new(Term::Succ(Box::new(Term::Zero)))))
    ); // False

    println!("{:?}", eval(Term::Succ(Box::new(Term::Zero)))); // Term::Succ(Term::Zero)

    println!(
        "{:?}",
        eval(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero)))))
    ); // Term::Zero

    println!(
        "{:?}",
        eval(Term::IsZero(Box::new(Term::Pred(Box::new(Term::Succ(
            Box::new(Term::Zero)
        ))))))
    ); // True

    println!(
        "{:?}",
        eval(Term::If {
            condition: Box::new(Term::IsZero(Box::new(Term::Pred(Box::new(Term::Succ(
                Box::new(Term::Zero)
            )))))),
            consequence: Box::new(Term::Succ(Box::new(Term::Succ(Box::new(Term::Zero))))),
            alternative: Box::new(Term::Zero)
        })
    ) // Term::Succ(Term::Succ(Term::Zero )) -> 2
}
