mod lexer;
mod token;
mod error;

use std::{env, fs, process};

use lexer::*;
use token::Token;

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
    
    // lexer(code);
}
