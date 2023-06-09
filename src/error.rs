use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter(char),
    UnmatchedParenthesis,
    InvalidLambda,
    InvalidApplication,
    InvalidVariable,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidLambda => write!(f, "Invalid lambda expression"),
            ParseError::InvalidApplication => write!(f, "Invalid application expression"),
            ParseError::InvalidVariable => write!(f, "Invalid variable expression"),
            ParseError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {}", c),
            ParseError::UnmatchedParenthesis => write!(f, "Unmatched parenthesis"),
        }
    }
}
