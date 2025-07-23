use std::env;
use std::fs;
use std::io::{self, Write};

use lisp::evaluator::Evaluator;
use lisp::evaluator::Value;
use lisp::parser::Parser;
use lisp::tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let evaluator = Evaluator::new();

    match args.len() {
        1 => {
            run_repl(evaluator)?;
        }
        2 => {
            let file_path = &args[1];
            run_file(evaluator, file_path)?;
        }
        _ => {
            eprintln!("Usage: {} [file_path]", args[0]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn run_repl(evaluator: Evaluator) -> io::Result<()> {
    println!("Lisp REPL (Rust Edition)");
    println!("Type 'exit' to quit.");

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();

        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!("\nExiting REPL.");
                break;
            }
            Ok(_) => {
                let line = input.trim();

                if line == "exit" {
                    println!("Exiting REPL.");
                    break;
                }

                if line.is_empty() {
                    continue;
                }

                match process_input(&evaluator, line) {
                    Ok(value) => println!("{}", value),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn run_file(evaluator: Evaluator, file_path: &str) -> io::Result<()> {
    println!("Running file: {}", file_path);

    let contents = fs::read_to_string(file_path)?;

    match process_input(&evaluator, &contents) {
        Ok(value) => {
            if value != Value::Nil {
                println!("{}", value);
            }
        }
        Err(e) => {
            eprintln!("Error in file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn process_input(evaluator: &Evaluator, input: &str) -> Result<Value, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer
        .tokenize()
        .map_err(|e| format!("Tokenization Error: {}", e))?;

    let mut parser = Parser::new(tokens);

    let ast = parser
        .parse()
        .map_err(|e| format!("Parsing Error: {}", e))?;

    let result = evaluator
        .eval_program(&ast)
        .map_err(|e| format!("Evaluation Error: {}", e))?;

    Ok(result)
}
