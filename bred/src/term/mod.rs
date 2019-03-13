mod parser;

use std::fmt;
use std::str::FromStr;
use std::ops::Deref;

pub use parser::ParseError;

#[derive(Debug, Clone)]
pub enum Term {
    App(Box<Term>, Box<Term>),
    Abs(u32, Box<Term>),
    Var(u32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Normal,
    Applicative,
}

impl Term {
    pub fn normal_form(self, strategy: Strategy) -> Term {
        if let Some(term) = self.reduce(strategy) {
            term.normal_form(strategy)
        } else {
            self
        }
    }

    fn reduce(&self, strategy: Strategy) -> Option<Term> {
        match self {
            Term::App(f, a) => f.apply(&a, strategy),
            Term::Abs(v, f) => f.reduce(strategy).map(|t| Term::Abs(*v, Box::new(t))),
            _ => None
        }
    }

    fn apply(&self, arg: &Term, strategy: Strategy) -> Option<Term> {
        match strategy {
            Strategy::Normal => self.reduce(strategy).map_or_else(|| {
                if let Term::Abs(_, b) = self {
                    Some(b.substitute(0, arg))
                } else {
                    arg.reduce(strategy).map(|term| Term::App(Box::new(self.clone()), Box::new(term)))
                }
            }, |term| { Some(Term::App(Box::new(term), Box::new(arg.clone()))) }),
            Strategy::Applicative => arg.reduce(strategy).map_or_else(|| {
                if let Term::Abs(_, b) = self {
                    Some(b.substitute(0, arg))
                } else {
                    None
                }
            }, |term| { Some(Term::App(Box::new(term), Box::new(arg.clone()))) })
        }
    }

    fn substitute(&self, var: u32, arg: &Term) -> Term {
        match self {
            Term::App(f, a) => Term::App(Box::new(f.substitute(var, arg)), Box::new(a.substitute(var, arg))),
            Term::Abs(v, f) => Term::Abs(*v, Box::new(f.substitute(var + 1, &arg.shift(1, 0)))),
            Term::Var(v) if *v == var => arg.clone(),
            Term::Var(_) => self.clone()
        }
    }

    fn shift(&self, d: u32, c: u32) -> Term {
        match self {
            Term::App(f, a) => Term::App(Box::new(f.shift(d, c)), Box::new(a.shift(d, c))),
            Term::Abs(v, f) => Term::Abs(*v, Box::new(f.shift(d, c + 1))),
            Term::Var(v) if *v < c => Term::Var(*v),
            Term::Var(v) => Term::Var(v + d)
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        match self {
            Term::App(f_self, a_self) => {
                if let Term::App(f_other, a_other) = other {
                    f_self == f_other && a_self == a_other
                } else {
                    false
                }
            }
            Term::Abs(v_self, f_self) => {
                if let Term::Abs(v_other, f_other) = other {
                    if v_self == v_other {
                        f_self == f_other
                    } else {
                        *f_self.deref() == f_other.substitute(*v_other, &Term::Var(*v_self))
                    }
                } else {
                    false
                }
            }
            Term::Var(v_self) => {
                if let Term::Var(v_other) = other {
                    v_other == v_self
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Term {}

impl fmt::Display for Term {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::App(f, a) => {
                match f.deref() {
                    Term::Abs(_, _) => if let Term::App(_, _) = a.deref() {
                        write!(formatter, "({}) ({})", f, a)
                    } else {
                        write!(formatter, "({}) {}", f, a)
                    }
                    Term::App(_, _) => {
                        let mut args = vec![];

                        let mut current = f.deref();
                        while let Term::App(fun, arg) = current {
                            args.push(arg.deref());
                            current = fun.deref();
                        }
                        args.push(current);

                        while let Some(term) = args.pop() {
                            if let Term::Var(_) = term {
                                write!(formatter, "{} ", term)?
                            } else {
                                write!(formatter, "({}) ", term)?
                            }
                        }

                        if let Term::App(_, _) = a.deref() {
                            write!(formatter, "({})", a)
                        } else {
                            write!(formatter, "{}", a)
                        }
                    }
                    Term::Var(_) => if let Term::App(_, _) = a.deref() {
                        write!(formatter, "{} ({})", f, a)
                    } else {
                        write!(formatter, "{} {}", f, a)
                    }
                }
            }
            Term::Abs(a, f) => write!(formatter, "\\{}. {}", a, f),
            Term::Var(v) => write!(formatter, "{}", v)
        }
    }
}

impl FromStr for Term {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_var_normal() {
        let var = Term::Var(0);
        assert_eq!(var.clone(), var.normal_form(Strategy::Normal));
    }

    #[test]
    fn reduce_var_applicative() {
        let var = Term::Var(0);
        assert_eq!(var.clone(), var.normal_form(Strategy::Applicative));
    }

    #[test]
    fn reduce_id_normal() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        assert_eq!(id.clone(), id.normal_form(Strategy::Normal));
    }

    #[test]
    fn reduce_id_applicative() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        assert_eq!(id.clone(), id.normal_form(Strategy::Applicative));
    }

    #[test]
    fn reduce_application_id_normal() {
        let term = Term::App(Box::new(Term::Abs(0, Box::new(Term::Var(0)))), Box::new(Term::Var(0)));
        assert_eq!(Term::Var(0), term.normal_form(Strategy::Normal));
    }

    #[test]
    fn reduce_application_id_applicative() {
        let term = Term::App(Box::new(Term::Abs(0, Box::new(Term::Var(0)))), Box::new(Term::Var(0)));
        assert_eq!(Term::Var(0), term.normal_form(Strategy::Applicative));
    }

    #[test]
    fn reduce_application_to_omega_normal() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(fst), Box::new(id.clone()))), Box::new(big_omega));

        assert_eq!(id, term.normal_form(Strategy::Normal));
    }

    #[test]
    #[ignore]
    fn reduce_application_to_omega_applicative() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(fst), Box::new(id.clone()))), Box::new(big_omega));

        assert_eq!(id, term.normal_form(Strategy::Applicative));
    }

    #[test]
    fn reduce_application_to_id() {
        let x = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let term = Term::App(Box::new(x), Box::new(id));

        assert_eq!(Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(0))))), term.normal_form(Strategy::Normal));
    }

    #[test]
    fn eq_id() {
        let id0 = Term::Abs(0, Box::new(Term::Var(0)));
        let id1 = Term::Abs(1, Box::new(Term::Var(1)));

        assert_eq!(id0, id1);
    }

    #[test]
    fn format_var() {
        let var = Term::Var(0);

        assert_eq!("0", format!("{}", var));
    }

    #[test]
    fn format_id() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));

        assert_eq!("\\0. 0", format!("{}", id));
    }

    #[test]
    fn format_fst() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));

        assert_eq!("\\1. \\0. 1", format!("{}", fst));
    }

    #[test]
    fn format_app_to_abs_in_abs() {
        let term = Term::from_str("\\1. 1 \\0. 0");

        assert_eq!(true, term.is_ok());
        assert_eq!("\\1. 1 \\0. 0", format!("{}", term.unwrap()));
    }

    #[test]
    fn format_application_to_omega() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(fst), Box::new(id.clone()))), Box::new(big_omega));

        assert_eq!("(\\1. \\0. 1) (\\0. 0) ((\\0. 0 0) \\0. 0 0)", format!("{}", term));
    }

    #[test]
    fn format_app_in_abs_four_times() {
        let term = Term::from_str("\\4. \\3. \\2. \\1. 1 2 3 4");

        assert_eq!(true, term.is_ok());
        assert_eq!("\\4. \\3. \\2. \\1. 1 2 3 4", format!("{}", term.unwrap()));
    }

    #[test]
    fn from_str_var() {
        let term = Term::from_str("0");

        assert_eq!(true, term.is_ok());
        assert_eq!(Term::Var(0), Term::from_str("0").unwrap())
    }

    #[test]
    fn from_str_id() {
        let term = Term::from_str("\\0.0");
        let id = Term::Abs(0, Box::new(Term::Var(0)));

        assert_eq!(true, term.is_ok());
        assert_eq!(id, term.unwrap())
    }

    #[test]
    fn from_str_id_whitespaced() {
        let term = Term::from_str("\\0. 0");
        let id = Term::Abs(0, Box::new(Term::Var(0)));

        assert_eq!(true, term.is_ok());
        assert_eq!(id, term.unwrap())
    }

    #[test]
    fn from_str_fst() {
        let term = Term::from_str("\\1.\\0.1");
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));

        assert_eq!(true, term.is_ok());
        assert_eq!(fst, term.unwrap())
    }

    #[test]
    fn from_str_fst_whitespaced() {
        let term = Term::from_str("\\1. \\0. 1");
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));

        assert_eq!(true, term.is_ok());
        assert_eq!(fst, term.unwrap())
    }

    #[test]
    fn from_str_omega_redundant_parentheses() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(fst), Box::new(id.clone()))), Box::new(big_omega));
        let converted = Term::from_str("((\\1.\\0.1) (\\0.0)) ((\\0.0 0) (\\0.0 0))");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_omega() {
        let fst = Term::Abs(1, Box::new(Term::Abs(0, Box::new(Term::Var(1)))));
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let omega = Term::Abs(0, Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))));
        let big_omega = Term::App(Box::new(omega.clone()), Box::new(omega));
        let term = Term::App(Box::new(Term::App(Box::new(fst), Box::new(id.clone()))), Box::new(big_omega));
        let converted = Term::from_str("(\\1. \\0. 1) (\\0. 0) ((\\0. 0 0) \\0. 0 0)");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_abs_in_abs_without_parentheses() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let term = Term::Abs(1, Box::new(Term::App(Box::new(Term::Var(1)), Box::new(id))));
        let converted = Term::from_str("\\1. 1 \\0. 0");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_app_three_times() {
        let term = Term::Abs(0, Box::new(Term::App(Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))), Box::new(Term::Var(0)))));
        let converted = Term::from_str("\\0. 0 0 0");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_app_four_times() {
        let term = Term::Abs(0, Box::new(Term::App(Box::new(Term::App(Box::new(Term::App(Box::new(Term::Var(0)), Box::new(Term::Var(0)))), Box::new(Term::Var(0)))), Box::new(Term::Var(0)))));
        let converted = Term::from_str("\\0. 0 0 0 0");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_abs_app_four_times() {
        let term = Term::Abs(
            4, Box::new(Term::Abs(
                3, Box::new(Term::Abs(
                    2, Box::new(Term::Abs(
                        1, Box::new(Term::App(
                            Box::new(Term::App(
                                Box::new(Term::App(
                                    Box::new(Term::Var(1)), Box::new(Term::Var(2)),
                                )), Box::new(Term::Var(3)),
                            )), Box::new(Term::Var(4)),
                        )),
                    )),
                )),
            )),
        );
        let converted = Term::from_str("\\4. \\3. \\2. \\1. 1 2 3 4");

        assert_eq!(true, converted.is_ok());
        assert_eq!(term, converted.unwrap());
    }

    #[test]
    fn from_str_outer_parentheses() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let converted = Term::from_str("(\\0. 0)");

        assert_eq!(true, converted.is_ok());
        assert_eq!(id, converted.unwrap());
    }

    #[test]
    fn from_str_many_outer_parentheses() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let converted = Term::from_str("((\\0. 0))");

        assert_eq!(true, converted.is_ok());
        assert_eq!(id, converted.unwrap());
    }

    #[test]
    fn from_str_redundant_parentheses() {
        let id = Term::Abs(0, Box::new(Term::Var(0)));
        let converted = Term::from_str("\\0. (0)");

        assert_eq!(true, converted.is_ok());
        assert_eq!(id, converted.unwrap());
    }
}
