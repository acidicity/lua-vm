pub mod lexer;
pub mod token;
pub mod parser;
pub mod ast;

use std::io::{self, Write};
use lexer::Lexer;

const PROMPT: &'static str = ">> ";
fn main() {

    println!("Welcome to the lua programming language!");
    loop {
        
        let mut buffer = String::new();
        let stdin = io::stdin();
        
        print!("{}", PROMPT);
        
        io::stdout().flush().expect("Failed to flush stdout");
        
        stdin.read_line(&mut buffer).expect("Error: Failed to read line");

        let lexer = Lexer::new(buffer.as_str());
        
        for token in lexer {
            println!("{:?}", token);
        }
    }
}
