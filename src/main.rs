#[macro_use]
extern crate maplit;

mod error;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod types;

use std::time::Instant;
use std::{env, fs, process};

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: europa <file>");
        process::exit(1);
    }

    let code = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        println!("Error reading file: {}", err.to_string());
        process::exit(1);
    });

    let start = Instant::now();
    let mut lexer = Lexer::new(&code);
    
    match lexer.init() {
        Err(e) => e.display(),
        Ok(toks) => {
            let end = start.elapsed();
            println!("lexer {:?}", end);

            let start = Instant::now();
            let mut parser = Parser::new(toks);
            match parser.init() {
                Err(e) => e.display(),
                Ok(tree) => {
                    let end = start.elapsed();
                    println!("parser {:?}", end);

                    let start = Instant::now();
                    let interpreter = Interpreter::new(tree);
                    match interpreter.init() {
                        Err(e) => e.display(),
                        Ok(t) => {
                            let end = start.elapsed();
                            println!("interpreter {:?}", end);
                            println!("{}", Interpreter::stringify(t));
                        }
                    }
                }
            }
        }
    };
}
