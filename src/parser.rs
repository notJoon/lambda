use std::{iter::Peekable, str::Chars};

use crate::error::ParseError;
use crate::term::Term;

type TermResult = Result<Term, ParseError>;

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

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_lambda(&mut self) -> TermResult {
        if self.chars.next() != Some('λ') {
            return Err(ParseError::InvalidLambda);
        }

        self.skip_whitespace();

        let bind = self.parse_var()?;
        if self.chars.next() != Some('.') {
            return Err(ParseError::InvalidLambda);
        }

        let body = self.parse_term()?;

        Ok(Term::Lambda {
            bind: bind,
            body: Box::new(body),
        })
    }

    /// Parse an application
    fn parse_application(&mut self) -> TermResult {
        let mut terms: Vec<Term> = vec![self.parse_term()?];
        while let Ok(term) = self.parse_term() {
            terms.push(term);
        }

        if terms.len() < 2 {
            return Err(ParseError::InvalidApplication);
        } else {
            let mut iter = terms.into_iter();
            let mut app = iter.next().unwrap();

            for term in iter {
                app = Term::Application {
                    func: Box::new(app),
                    arg: Box::new(term),
                };
            }

            Ok(app)
        }
    }

    /// Parse a variable
    fn parse_var(&mut self) -> Result<String, ParseError> {
        let mut name = String::new();

        while let Some(c) = self.chars.peek() {
            if c.is_alphabetic() || *c == '_' {
                name.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            Err(ParseError::InvalidVariable)
        } else {
            Ok(name)
        }
    }

    fn parse_term(&mut self) -> TermResult {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&'λ') => self.parse_lambda(),
            Some(&'(') => {
                self.chars.next();
                let term = self.parse_application()?;
                if self.chars.next() == Some(')') {
                    Ok(term)
                } else {
                    Err(ParseError::InvalidApplication)
                }
            }
            Some(c) if c.is_alphanumeric() || *c == '_' => {
                Ok(Term::Variable {
                    name: self.parse_var()?,
                })
            }
            _ => Err(ParseError::InvalidApplication),
        }
    }
}

pub fn parse(input: &str) -> TermResult {
    let mut parser = Parser::new(input);
    let term = parser.parse_term()?;
    if parser.chars.peek().is_some() {
        Err(ParseError::InvalidApplication)
    } else {
        Ok(term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_lambda_expr() {
        let input = "λx.x";
        let expected = Term::Lambda {
            bind: "x".to_string(),
            body: Box::new(Term::Variable {
                name: "x".to_string(),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_valid_application() {
        let input = "(λx.x) y";
        let expected = Term::Application {
            func: Box::new(Term::Lambda {
                bind: "x".to_string(),
                body: Box::new(Term::Variable {
                    name: "x".to_string(),
                }),
            }),
            arg: Box::new(Term::Variable {
                name: "y".to_string(),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_valid_application_with_parenthesis() {
        let input = "(f x)";
        let expected = Term::Application {
            func: Box::new(Term::Variable {
                name: "f".to_string(),
            }),
            arg: Box::new(Term::Variable {
                name: "x".to_string(),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_invalid_lambda_expr() {
        let input = "λx";
        assert_eq!(parse(input), Err(ParseError::InvalidLambda));
    }

    #[test]
    fn test_parse_valid_complex_term() {
        let input = "λx.(λy.x y) z";
        let expected = Term::Lambda {
            bind: "x".to_string(),
            body: Box::new(Term::Application {
                func: Box::new(Term::Lambda {
                    bind: "y".to_string(),
                    body: Box::new(Term::Application {
                        func: Box::new(Term::Variable {
                            name: "x".to_string(),
                        }),
                        arg: Box::new(Term::Variable {
                            name: "y".to_string(),
                        }),
                    }),
                }),
                arg: Box::new(Term::Variable {
                    name: "z".to_string(),
                }),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }
}