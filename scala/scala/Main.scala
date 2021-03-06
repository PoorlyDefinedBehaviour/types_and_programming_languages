sealed trait Term
case object TermTrue extends Term
case object TermFalse extends Term
case class If(condition: Term, consequence: Term, alternative: Term)
    extends Term
case object Zero extends Term
case class Succ(term: Term) extends Term
case class Pred(term: Term) extends Term
case class IsZero(term: Term) extends Term

object Term {
  def isNumericValue(term: Term): Boolean =
    term match {
      case Zero       => true
      case Succ(term) => isNumericValue(term)
      case Pred(term) => isNumericValue(term)
      case _          => false
    }

  def eval(term: Term): Term =
    term match {
      case If(TermTrue, consequence, _)  => eval(consequence)
      case If(TermFalse, _, alternative) => eval(alternative)
      case If(condition, consequence, alternative) =>
        eval(If(eval(condition), consequence, alternative))
      case Succ(term)                                 => Succ(eval(term))
      case Pred(Zero)                                 => Zero
      case Pred(Succ(term)) if isNumericValue(term)   => eval(term)
      case Pred(term)                                 => Pred(eval(term))
      case IsZero(Zero)                               => TermTrue
      case IsZero(Succ(term)) if isNumericValue(term) => TermFalse
      case IsZero(term)                               => eval(IsZero(eval(term)))
      case _                                          => term
    }
}

object Main extends App {
  println(Term.isNumericValue(Zero)) // true
  println(Term.isNumericValue(Succ(Zero))) // true
  println(Term.isNumericValue(Pred(Succ(Zero)))) // true

  println(Term.eval(TermTrue)) // True

  println(Term.eval(TermFalse)) // False

  println(Term.eval(IsZero(Zero))) // True

  println(Term.eval(IsZero(Succ(Zero)))) // False

  println(Term.eval(Succ(Zero))) // Succ(Zero)

  println(Term.eval(Pred(Succ(Zero)))) // Zero

  println(Term.eval(IsZero(Pred(Succ(Zero))))) // True

  println(
    Term.eval(If(IsZero(Pred(Succ(Zero))), Succ(Succ(Zero)), Zero))
  ) // Succ(Succ(Zero)) -> 2
}
