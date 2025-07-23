// src/main.rs
use std::fs;
use std::io;

use lisp::parser::Parser;
use lisp::tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let file_path = "./code_examples/code.example";
    let contents = fs::read_to_string(file_path)?;

    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    println!("--- Tokens ---");
    print!("{:?}", ast);
    println!("--- End of Tokens ---");

    Ok(()) 
}