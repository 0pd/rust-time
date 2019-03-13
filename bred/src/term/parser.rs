use std::str::FromStr;
use super::Term;

#[derive(Clone, Copy, Debug)]
pub struct ParseError {}

pub struct Parser {}

impl Parser {
    pub fn parse(s: &str) -> Result<Term, ParseError> {
        Self::parse_term(s).map(|(term, _)| { term }).ok_or(ParseError {})
    }

    fn parse_term(s: &str) -> Option<(Term, &str)> {
        Some(()).and_then(|_| {
            Self::parse_app(s).or_else(|| {
                Self::parse_term_in_parentheses(s)
            })
        })
    }

    fn parse_term_in_parentheses(s: &str) -> Option<(Term, &str)> {
        Self::parse_symbol(s, '(').and_then(|open_parenthesis_slice| {
            Self::parse_term(open_parenthesis_slice).and_then(|(term, term_slice)| {
                Self::parse_symbol(term_slice, ')').map(|close_parenthesis_slice| {
                    (term, close_parenthesis_slice)
                })
            })
        }).map(Self::skip_whitespace_with_value)
    }

    fn parse_abs(s: &str) -> Option<(Term, &str)> {
        Self::parse_symbol(s, '\\').and_then(|slash_slice| {
            Self::parse_number(slash_slice).and_then(|(var, var_slice)| {
                Self::parse_symbol(var_slice, '.').and_then(|dot_slice| {
                    Self::parse_term(dot_slice).map(|(term, term_slice)| {
                        (Term::Abs(var, Box::new(term)), term_slice)
                    })
                })
            })
        }).map(Self::skip_whitespace_with_value)
    }

    fn parse_app(s: &str) -> Option<(Term, &str)> {
        Self::parse_sub_term(s).map(|(f, slice)| {
            let mut result = f;
            let mut current_slice = slice;

            while let Some((term, new_slice)) = Self::parse_sub_term(current_slice) {
                result = Term::App(Box::new(result), Box::new(term));
                current_slice = new_slice;
            }

            (result, current_slice)
        }).map(Self::skip_whitespace_with_value)
    }

    fn parse_sub_term(s: &str) -> Option<(Term, &str)> {
        Self::parse_var(s).or_else(|| {
            Self::parse_abs(s).or_else(|| {
                Self::parse_term_in_parentheses(s)
            })
        })
    }

    fn parse_var(s: &str) -> Option<(Term, &str)> {
        Self::parse_number(s).map(|(n, slice)| { (Term::Var(n), slice) })
    }

    fn parse_symbol(s: &str, symbol: char) -> Option<&str> {
        s.chars().next().filter(|c| { *c == symbol }).map(|_| { &s[1..] }).map(Self::skip_whitespace)
    }

    fn parse_number(s: &str) -> Option<(u32, &str)> {
        let count = s.chars().take_while(|c| { c.is_ascii_digit() }).count();

        u32::from_str(&s[..count]).ok().map(|n| { (n, &s[count..]) }).map(Self::skip_whitespace_with_value)
    }

    fn skip_whitespace_with_value<T>((value, slice): (T, &str)) -> (T, &str) {
        (value, Self::skip_whitespace(slice))
    }

    fn skip_whitespace(slice: &str) -> &str {
        let count = slice.chars().take_while(|c| { c.is_ascii_whitespace() }).count();

        &slice[count..]
    }
}