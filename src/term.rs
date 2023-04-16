use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Term {
    Lambda { bind: String, body: Box<Term> },
    Application { func: Box<Term>, arg: Box<Term> },
    Variable { name: String },
    Null,
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Lambda { bind, body } => write!(
                f,
                r#"{{"tag": "lambda", "bind": "{bind}", "body": {body}}}"#,
            ),
            Term::Application { func, arg } => write!(
                f,
                r#"{{"tag": "application", "func": {func}, "arg": {arg}}}"#,
            ),
            Term::Variable { name } => write!(f, r#"{{"tag": "var", "name": "{name}"}}"#),
            Term::Null => write!(f, "null"),
        }
    }
}
