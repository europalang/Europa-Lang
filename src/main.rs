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
use std::io::{stdin, stdout, Write};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use crate::token::Token;
use crate::nodes::stmt::Stmt;
use crate::environment::Environment;

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

    // Load code and create Environment
    let environ = load(code, Environment::new(None));
    repl(environ);
}

// Loader for code, returns Environment mutated from environ
fn load(code: String, environ: Environment) -> Environment {
    let mut time = Instant::now();
    let tokens: Vec<Token> = match Lexer::new(&code).init() {
        Err(e) => {
            e.display();
            return environ;
        },
        Ok(toks) => {
            println!("lexer {:?}", time.elapsed());
            toks
        }
    };

    // Turn tokens into AST
    time = Instant::now();
    let tree: Vec<Stmt> = match Parser::new(tokens).init() {
        Err(e) => {
            e.display();
            return environ;
        },
        Ok(tree) => {
            println!("parser {:?}", time.elapsed());
            tree
        }
    };

    // Interpret and return environment
    time = Instant::now();
    let mut interpreter = Interpreter::new(tree, environ.clone());
    match interpreter.init() {
        Err(e) => {
            e.display();
            environ
        },
        Ok(env) => {
            println!("interpreter {:?}", time.elapsed());
            env
        }
    }
}

// Loops until exited
fn repl(mut environ: Environment) {
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
            },
            Ok(_) => input = input.trim().to_string()
        }

        // Exit out of program
        if input.eq("exit") {
            process::exit(0);
        }

        environ = load(input, environ);
        println!("{:?}", environ);
    }
}