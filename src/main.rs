extern crate maplit;

mod environment;
mod error;
mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod token;
mod types;

use std::io::{stdin, stdout, Read};
use std::time::Instant;
use std::{env, fs, process};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use crate::environment::Environment;
use crate::error::Error;
use crate::nodes::stmt::Stmt;
use crate::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Welcome to the Europa Interactive Repl."
        );

        let environ = Box::new(Environment::new());
        init_repl(environ); // Start REPL with no context

        process::exit(0);
    }

    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        println!("Europa Lang {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let code = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        println!("Error reading file: {}", err.to_string());
        process::exit(1);
    });

    // Load code and create Environment
    match init(code, Box::new(Environment::new())) {
        Err(e) => e.display(),
        Ok(environ) => {
            if args.len() == 3 && args.contains(&String::from("--repl")) {
                init_repl(environ); // Start repl with context
            }
        }
    }
}

// Loader for code, returns Environment mutated from environ
fn init(code: String, environ: Box<Environment>) -> Result<Box<Environment>, Error> {
    let mut time = Instant::now();
    let tokens: Vec<Token> = match Lexer::new(&code).init() {
        Err(e) => return Err(e),
        Ok(toks) => {
            println!("lexer {:?}", time.elapsed());
            toks
        }
    };

    // Turn tokens into AST
    time = Instant::now();
    let tree: Vec<Stmt> = match Parser::new(tokens).init() {
        Err(e) => return Err(e),
        Ok(tree) => {
            println!("parser {:?}", time.elapsed());
            tree
        }
    };

    // Interpret and return environment
    time = Instant::now();
    let mut interpreter = Interpreter::new(tree, environ.clone());
    match interpreter.init() {
        Err(e) => return Err(e),
        Ok(_) => {
            println!("interpreter {:?}", time.elapsed());
            Ok(interpreter.environ)
        }
    }
}

// Loops until exited
fn init_repl(mut environ: Box<Environment>) {
    loop {
        // Same line print
        print!("\x1b[33m>\x1b[0m ");
        stdout().flush().unwrap();

        // Wait for input from user
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Err(e) => {
                println!("Unexpected REPL Error: {:?}", e);
                process::exit(1);
            }
            Ok(n) => {
                if n == 0 {
                    println!("\n");
                    process::exit(0);
                }
                input = input.trim().to_string();
            },
        }

        // Exit out of program
        if input.eq("exit") {
            process::exit(0);
        }

        // Attempt to run code
        match init(input, environ.clone()) {
            Err(e) => e.display(),
            Ok(env) => {
                // Change environ values if no errors
                environ = env;
                println!("{:?}", environ);
            }
        };
    }
}
