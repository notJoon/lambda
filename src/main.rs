mod error;
mod parser;
mod term;

use crate::parser::parse;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the lambda calculus REPL!");
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

        match parse(input.trim()) {
            Ok(term) => {
                let term = serde_json::to_string_pretty(&term).unwrap();
                println!("\n{term}\n")
            }
            Err(e) => println!("{e}"),
        }
    }
}
