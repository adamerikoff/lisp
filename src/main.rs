// src/main.rs
use std::fs;
use std::io;

use lisp::tokenizer::Token;
use lisp::tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let file_path = "./code_examples/code.example";
    let contents = fs::read_to_string(file_path)?;

    let mut tokenizer = Tokenizer::new(&contents);

    println!("--- Tokens ---");
    loop {
        match tokenizer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
                if token == Token::Eof {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Tokenizer error: {:?}", e);
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Tokenizer error: {:?}", e)));
            }
        }
    }
    println!("--- End of Tokens ---");

    Ok(()) 
}