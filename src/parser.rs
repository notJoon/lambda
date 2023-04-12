use std::{iter::Peekable, str::Chars};

use crate::term::Term;

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

/// A parser for lambda calculus expressions.
impl<'a> Parser<'a> {
    /// Create a new parser for the given input.
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn parse_lambda(&mut self) -> Term {
        let _ = self.chars.next(); // skip `λ`
        let bind = self.parse_var();
        let _ = self.chars.next(); // skip `.`

        let body = self.parse_term();

        Term::Lambda {
            bind: bind,
            body: Box::new(body),
        }
    }

    /// Parse an application
    fn parse_application(&mut self) -> Term {
        let _ = self.chars.next(); // skip `(`

        let func = self.parse_term();
        let _ = self.chars.next(); // skip whitespace

        let arg = self.parse_term();
        let _ = self.chars.next(); // skip `)`

        Term::Application {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    /// Parse a variable
    fn parse_var(&mut self) -> String {
        let mut var = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                var.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        var
    }

    fn parse_term(&mut self) -> Term {
        loop {
            match self.chars.peek() {
                Some(c) if c.is_whitespace() => {
                    self.chars.next(); // Skip whitespace
                }
                Some('λ') => return self.parse_lambda(),
                Some('(') => return self.parse_application(),
                Some(c) if c.is_alphabetic() => {
                    return Term::Variable {
                        name: self.parse_var(),
                    }
                }
                _ => break,
            }
        }

        Term::Null
    }
}

pub fn parse(input: &str) -> Term {
    let mut parser = Parser::new(input);
    parser.parse_term()
}
