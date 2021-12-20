use std::io::{stdin, stdout, Write};
use std::process;

use crate::environment::Environment;
use crate::types::Type;

use super::run_string;

pub fn init(mut environ: Environment, verbose: bool) {
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
            }
        }

        // Exit out of program
        if input.eq("exit") {
            process::exit(0);
        }

        // Attempt to run code
        match run_string(&input, &mut environ, verbose) {
            Err(e) => e.display(&input),
            Ok(eval) => if eval != Type::Nil {
                println!("{}", eval)
            },
        };
    }
}
