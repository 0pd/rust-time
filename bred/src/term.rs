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

#[derive(Clone, Copy)]
pub enum Strategy {
    Normal,
    Applicative
}

fn substitute(var: u32, arg: &Term, function: &Term) -> Term {
    match function {
        Term::Var(v) if *v == var => {
            arg.clone()
        },
        Term::Var(v) => {
            function.clone()
        },
        _ => unimplemented!()
    }
}

fn reduce(term: &Term, strategy: Strategy) -> Option<Term> {
    match term {
        Term::App(_, _) => {
            unimplemented!()
        },
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
}