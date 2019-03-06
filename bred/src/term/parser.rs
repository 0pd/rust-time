use std::str::FromStr;
use super::Term;

#[derive(Clone, Copy, Debug)]
pub struct ParseError {}

pub struct Parser {}

impl Parser {
    pub fn parse(s: &str) -> Result<Term, ParseError> {
        Self::parse_term(s, &mut 0).ok_or(ParseError {})
    }

    fn parse_term(s: &str, i: &mut usize) -> Option<Term> {
        Self::parse_abs(s, i).map_or_else(|| {
            Self::parse_app(s, i).map_or_else(|| {
                Self::parse_var(s, i)
            }, |app| { Some(app) })
        }, |abs| { Some(abs) })
    }

    fn parse_abs(s: &str, i: &mut usize) -> Option<Term> {
        let mut index = *i;

        Self::parse_symbol(s, &mut index, '\\').and_then(|_| {
            Self::parse_number(s, &mut index).and_then(|v| {
                Self::parse_symbol(s, &mut index, '.').and_then(|_| {
                    Self::parse_term(s, &mut index).map(|t| {
                        *i = index;
                        Term::Abs(v, Box::new(t))
                    })
                })
            })
        }).debug()
    }

    fn parse_app(s: &str, i: &mut usize) -> Option<Term> {
        let mut index = *i;

        Self::parse_var(s, &mut index).or_else(|| {
            Self::parse_term_in_parentheses(s, &mut index)
        }).and_then(|f| {
            Self::parse_symbol(s, &mut index, ' ').and_then(|_| {
                Self::parse_var(s, &mut index).or_else(|| {
                    Self::parse_term_in_parentheses(s, &mut index)
                }).map(|a| {
                    Term::App(Box::new(f), Box::new(a))
                })
            })
        }).map(|t| {
            *i = index;
            t
        }).debug()
    }

    fn parse_term_in_parentheses(s: &str, i: &mut usize) -> Option<Term> {
        let mut index = *i;

        Self::parse_symbol(s, &mut index, '(').and_then(|_| {
            Self::parse_term(s, &mut index).and_then(|t| {
                Self::parse_symbol(s, &mut index, ')').map(|_| {
                    *i = index;
                    t
                })
            })
        }).debug()
    }

    fn parse_var(s: &str, i: &mut usize) -> Option<Term> {
        Self::parse_number(s, i).map(|n| { Term::Var(n) })
    }

    fn parse_symbol(s: &str, i: &mut usize, symbol: char) -> Option<()> {
        s.chars().skip(*i).next().filter(|c| { *c == symbol }).map(|_| {
            *i += 1;
            ()
        })
    }

    fn parse_number(s: &str, i: &mut usize) -> Option<u32> {
        let count = s
            .chars()
            .skip(*i)
            .take_while(|c| { c.is_ascii_digit() })
            .count();

        u32::from_str(&s[*i..*i + count]).ok().map(|n| {
            *i += count;
            n
        })
    }

    fn space_shift(s: &str, head: usize) -> usize {
        let tail = s.chars().skip(head).skip_while(|c| { c.is_whitespace() }).count();
        s.len() - head - tail
    }
}

trait Dbug {
    fn debug(self) -> Self;
}

impl Dbug for Option<Term> {
    fn debug(self) -> Self  {
        self.map(|t| {
            println!("{:?}", t);
            t
        })
    }
}