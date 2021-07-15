#[macro_use]
extern crate maplit;

mod environment;
mod error;
mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod token;
mod types;

use std::time::Instant;
use std::{env, fs, process};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: europa <file>");
        process::exit(1);
    }

    if args[1] == "--version" || args[1] == "-v" {
        println!("Europa Lang {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
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
                    let mut interpreter = Interpreter::new(tree);
                    match interpreter.init() {
                        Err(e) => e.display(),
                        Ok(()) => {
                            let end = start.elapsed();
                            println!("interpreter {:?}", end);
                        }
                    }
                }
            }
        }
    };
}
