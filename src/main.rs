#[macro_use] extern crate maplit;

mod error;
mod lexer;
mod token;
mod types;

use std::time::Instant;
use std::{env, fs, process};

use lexer::*;

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
    let end = start.elapsed();
    match lexer.init() {
        Err(e) => e.display(),
        Ok(tok) => println!("{:#?} {:?}", tok, end),
    }
}
