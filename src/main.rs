mod parser;
mod term;

use crate::{parser::parse, term::term_to_json};
use std::io::{self, Write};

fn main() {
    print!("Welcome to the lambda calculus REPL!\n");
    print!("Type `exit` or `quit` to exit.\n\n");

    // Loop the REPL until the user exits
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        // Check if the user wants to exit
        if input == "exit" || input == "quit" {
            break;
        }

        let term = parse(&input);
        let json = term_to_json(&term);

        println!("\n{}\n", serde_json::to_string_pretty(&json).unwrap());
    }
}
