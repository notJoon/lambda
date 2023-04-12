# Lambda Calculus Parser

This is a parser for **λ-calculus** expressions. It takes a  λ-terms as input, parses it and returns a JSON representation of the term. The parser currently supports  λ-abstractions, variables and function applications. It uses a recursive descent parsing technique.

## Features

- Parses λ-abstractions, variables and function applications
- Returns a JSON representation of the parsed λ-term
- Includes a REPL for interactive parsing

## Usage

The parser is implemented as a **`Parser`** struct with several methods. It uses a `Peekable<Chars>` iterator to read the input string single characters at a time. The main parsing function, `parse_term`, loop over the input string and determine the appropriate parsing function to call based on the current character.

The parsing functions are:

- `parse_lambda` : Parsed a λ-abstraction(e.g., `λx.x`)
- `parse_application` : Parses a function application (e.g., `(f x)`)
- `parse_variable` : Parses a variable (e.g., `x`)

The parser returns an enum **`Term`**, which can be one of the following:

- `Lambda`: Represents a λ-abstraction with a binding variable and a body
- `Application`: Represents a function application with a function and an argument
- `Variable`: Represents a variable with a name
- `Null`: Represents an empty term or failed parsing

The parsed `Term` is then converted to a JSON format using the `term_to_json` function. which converts the `Term` to a `serde_json::Value` object.

## Example

Here's an example of how to parse a λ-term using the parser:

```rust
fn main() {
    let input = "λf. λx. (f x)";
    let term = parse(input);
    let json = term_to_json(&term);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
```

This will output:

```json
{
  "bind": "f",
  "body": {
    "bind": "x",
    "body": {
      "arg": {
        "name": "x",
        "tag": "var"
      },
      "func": {
        "name": "f",
        "tag": "var"
      },
      "tag": "application"
    },
    "tag": "lambda"
  },
  "tag": "lambda"
}
```

> **Note:** There is a problem that the position of the tag goes down when print the current JSON. This will be fixed in the future.

To use the REPL, run the following command:

```bash
cargo run
```

After that, you can enter a λ-term and press enter to parse it. The REPL will print the parsed JSON representation of the term.
