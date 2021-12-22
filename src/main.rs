extern crate maplit;

mod environment;
mod error;
mod functions;
mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod repl;
mod resolver;
mod tests;
mod token;
mod types;
mod stdlib;

use std::time::Instant;
use std::{env, fs, process};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use resolver::Resolver;

use crate::environment::Environment;
use crate::error::Error;
use crate::types::Type;

use clap::{App, Arg};

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROGRAM_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PROGRAM_ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author(PROGRAM_AUTHORS)
        .about(PROGRAM_ABOUT)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Log extra information"),
        )
        .arg(
            Arg::with_name("repl")
                .short("r")
                .long("repl")
                .help("Run file and continue to REPL"),
        )
        .arg(
            Arg::with_name("eval")
                .short("e")
                .long("eval")
                .value_name("CODE")
                .help("Evaluate CODE instead of a file")
                .takes_value(true)
                .conflicts_with("FILE"),
        )
        .arg(Arg::with_name("FILE").help("File to run").index(1))
        .get_matches();

    let verbose = matches.is_present("verbose");

    let code = if let Some(file) = matches.value_of("FILE") {
        // run file contents

        fs::read_to_string(file).unwrap_or_else(|err| {
            eprintln!("Error reading file: {}", err);
            process::exit(1)
        })
    } else if let Some(code) = matches.value_of("eval") {
        // run argument value

        String::from(code)
    } else {
        // no code to run, drop into repl

        println!("Welcome to the Europa interactive REPL.\nUse \".exit\" to exit.");

        // start no-context repl
        let environ = Environment::new();
        repl::init(environ, verbose);

        return;
    };

    // load and run code
    let mut environ = Environment::new();
    match run_string(&code, &mut environ, verbose) {
        Err(e) => {
            e.display(&code);
            process::exit(1);
        }
        Ok(eval) => {
            if matches.is_present("repl") {
                println!("{:?}", eval);

                // drop into repl with environment
                repl::init(environ, verbose);
            }
        }
    }
}

// Loader for code, mutates Environment and returns evaluated (probably Nil)
fn run_string(
    code: &String,
    environ: &mut Environment,
    verbose: bool,
) -> Result<Type, Error> {
    // Tokenise code
    let mut time = Instant::now();
    let tokens = Lexer::new(&code).init()?;

    if verbose {
        eprintln!("lexer {:?}", time.elapsed());
    }

    // Turn tokens into AST
    time = Instant::now();
    let tree = Parser::new(tokens).init()?;

    if verbose {
        eprintln!("parser {:?}", time.elapsed());
    }

    // Create interpreter
    let mut interpreter = Interpreter::new(tree, environ.clone());

    // Resolve variables
    time = Instant::now();
    interpreter = Resolver::new(interpreter).init()?;

    if verbose {
        eprintln!("resolver {:?}", time.elapsed());
    }

    // Run interpreter
    time = Instant::now();
    let eval = interpreter.init()?;

    if verbose {
        eprintln!("interpreter {:?}", time.elapsed());
    }

    *environ = interpreter.environ;

    Ok(eval)
}
