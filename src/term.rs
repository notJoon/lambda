use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Term {
    Lambda { bind: String, body: Box<Term> },
    Application { func: Box<Term>, arg: Box<Term> },
    Variable { name: String },
    Null,
}

/// Convert a parsed term to a JSON value.
pub fn term_to_json(term: &Term) -> serde_json::Value {
    match term {
        Term::Lambda { bind, body } => json!({
            "tag": "lambda",
            "bind": bind,
            "body": term_to_json(body),
        }),
        Term::Application { func, arg } => json!({
            "tag": "application",
            "func": term_to_json(func),
            "arg": term_to_json(arg),
        }),
        Term::Variable { name } => json!({
            "tag": "var",
            "name": name,
        }),
        Term::Null => serde_json::Value::Null,
    }
}
