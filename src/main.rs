mod error;
mod parser;
mod term;

use crate::{term::term_to_json, parser::parse};
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

        match parse(&input.trim()) {
            Ok(term) => {
                let json = term_to_json(&term);
                println!("\n{}\n", serde_json::to_string_pretty(&json).unwrap());
            }
            Err(err) => {
                println!("\nError: {}\n", err);
            }
        }
    }
}
