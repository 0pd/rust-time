use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum Term {
    App(Box<Term>, Box<Term>),
    Abs(u32, Box<Term>),
    Var(u32)
}

impl fmt::Display for Term {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParseError {}

impl FromStr for Term {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Normal,
    Applicative
}

fn shift(d: u32, c: u32, term: &Term) -> Term {
    match term {
        Term::App(f, a) => Term::App(Box::new(shift(d, c, f)), Box::new(shift(d, c, a))),
        Term::Abs(v, f) => Term::Abs(*v, Box::new(shift(d, c + 1, f))),
        Term::Var(v) if *v < c => Term::Var(*v),
        Term::Var(v) => Term::Var(v + d)
    }
}

fn substitute(var: u32, arg: &Term, function: &Term) -> Term {
    match function {
        Term::App(f, a) => Term::App(Box::new(substitute(var, arg, f)), Box::new(substitute(var, arg, a))),
        Term::Abs(v, f) => Term::Abs(*v, Box::new(substitute(var + 1, &shift(1, 0, arg), f))),
        Term::Var(v) if *v == var => arg.clone(),
        Term::Var(v) => function.clone()
    }
}

// dirty hack, IDK how to handle Box<Term> with match arms
fn handle_app(fun: &Term, arg: &Term, strategy: Strategy) -> Option<Term> {
    match strategy {
        Strategy::Normal => match reduce(fun, strategy) {
            Some(term) => Some(Term::App(Box::new(term), Box::new(arg.clone()))),
            None => match fun {
                Term::Abs(v, b) => Some(substitute(*v, arg, b)),
                _ => match reduce(arg, strategy) {
                    Some(term) => Some(Term::App(Box::new(fun.clone()), Box::new(term))),
                    None => None
                }
            }
        },
        Strategy::Applicative => match reduce(arg, strategy) {
            Some(term) => Some(Term::App(Box::new(fun.clone()), Box::new(term))),
            None => match reduce(fun, strategy) {
                Some(term) => Some(Term::App(Box::new(term), Box::new(arg.clone()))),
                None => match fun {
                    Term::Abs(v, b) => Some(substitute(*v, arg, b)),
                    _ => None
                }
            }
        }
    }
}

fn reduce(term: &Term, strategy: Strategy) -> Option<Term> {
    match term {
        Term::App(f, a) => handle_app(f, a, strategy),
        Term::Abs(v, f) => match reduce(f, strategy) {
            Some(t) => Some(Term::Abs(*v, Box::new(t))),
            None => None
        },
        _ => None
    }
}

pub fn normal_form(term: &Term, strategy: Strategy) -> Term {
    match reduce(term, strategy) {
        Some(t) => normal_form(&t, strategy),
        None => term.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_var_normal() {
        let var = Term::Var(0);
        assert_eq!(var, normal_form(&var, Strategy::Normal));
    }

    #[test]
    fn test_reduce_var_applicative() {
        let var = Term::Var(0);
        assert_eq!(var, normal_form(&var, Strategy::Applicative));
    }

    #[test]
    fn test_reduce_id_normal() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        assert_eq!(id, normal_form(&id, Strategy::Normal));
    }

    #[test]
    fn test_reduce_id_applicative() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        assert_eq!(id, normal_form(&id, Strategy::Applicative));
    }

    #[test]
    fn test_reduce_application_id_normal() {
        let term = Term::App(Box::new(Term::Abs(0, Box::new(Term::Var(0)))), Box::new(Term::Var(0)));
        assert_eq!(Term::Var(0), normal_form(&term, Strategy::Normal));
    }

    #[test]
    fn test_reduce_application_id_applicative() {
        let term = Term::App(Box::new(Term::Abs(0, Box::new(Term::Var(0)))), Box::new(Term::Var(0)));
        assert_eq!(Term::Var(0), normal_form(&term, Strategy::Applicative));
    }

    #[test]
    fn test_reduce_application_to_omega_normal() {
        let x = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(x), Box::new(id))), Box::new(big_omega));

        assert_eq!(Term::Var(1), normal_form(&term, Strategy::Normal));
    }

    #[test]
    #[ignore]
    fn test_reduce_application_to_omega_applicative() {
        let x = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(x), Box::new(id))), Box::new(big_omega));

        assert_eq!(Term::Var(1), normal_form(&term, Strategy::Applicative));
    }
}