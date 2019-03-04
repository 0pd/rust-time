use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
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

pub fn reduce(term: &Term) -> Term {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_id() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        assert_eq!(id, reduce(&id).clone());
    }
}