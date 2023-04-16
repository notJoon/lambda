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
        let mut terms: Vec<Term> = vec![self.parse_non_application_term()?];

        while let Ok(term) = self.parse_non_application_term() {
            terms.push(term);
        }

        if terms.is_empty() {
            Err(ParseError::InvalidApplication)
        } else if terms.len() == 1 {
            Ok(terms.pop().unwrap())
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

        if self.chars.peek() == Some(&'(') {
            self.chars.next();

            let term = self.parse_application()?;
            self.chars
                .next()
                .and_then(|c| if c == ')' { Some(term) } else { None })
                .ok_or(ParseError::InvalidApplication)
        } else {
            self.parse_non_application_term()
        }
    }

    /// Parse a non-application term (i.e., a lambda abstraction or a variable) from the input.
    ///
    /// This function is used to parse the sub-expressions of an application. Since an application
    /// consists of a sequence of non-application terms, this function ensures that only lambda
    /// abstractions or variables are parsed within an application.
    ///
    /// # Returns
    ///
    /// * `Ok(JsonTerm)` - A successfully parsed non-application term (lambda abstraction or variable).
    /// * `Err(ParseError::InvalidApplication)` - If the input doesn't match a valid non-application term.
    fn parse_non_application_term(&mut self) -> TermResult {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&'λ') => self.parse_lambda(),
            Some(c) if c.is_alphanumeric() || *c == '_' => Ok(Term::Variable {
                name: self.parse_var()?,
            }),
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
    fn test_parse_constant_function() {
        let input = "λx.λy.x";
        let expected = Term::Lambda {
            bind: "x".to_string(),
            body: Box::new(Term::Lambda {
                bind: "y".to_string(),
                body: Box::new(Term::Variable {
                    name: "x".to_string(),
                }),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_y_combinator_form() {
        let input = "λx.(x x)";
        let expected = Term::Lambda {
            bind: "x".to_string(),
            body: Box::new(Term::Application {
                func: Box::new(Term::Variable {
                    name: "x".to_string(),
                }),
                arg: Box::new(Term::Variable {
                    name: "x".to_string(),
                }),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_function_composition() {
        let input = "λf.λg.λx.(f(g x))";
        let expected = Term::Lambda {
            bind: "f".to_string(),
            body: Box::new(Term::Lambda {
                bind: "g".to_string(),
                body: Box::new(Term::Lambda {
                    bind: "x".to_string(),
                    body: Box::new(Term::Application {
                        func: Box::new(Term::Variable {
                            name: "f".to_string(),
                        }),
                        arg: Box::new(Term::Application {
                            func: Box::new(Term::Variable {
                                name: "g".to_string(),
                            }),
                            arg: Box::new(Term::Variable {
                                name: "x".to_string(),
                            }),
                        }),
                    }),
                }),
            }),
        };

        assert_eq!(parse(input), Ok(expected));
    }
}
