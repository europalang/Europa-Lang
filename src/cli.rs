use std::fs::File;
use std::io::prelude::*;

use indoc::printdoc; // Unindent multiline print

// Parsed instruction from user
pub struct Instruction {
    pub file: String,
    pub run_repl: bool,
    pub debug: bool,
}

// Take list of arguments passed, and parse into instruction
pub fn parse_arguments(args: &Vec<String>) -> Instruction {
    let mut instruction = Instruction {
        file: String::from(""),
        run_repl: false,
        debug: false,
    };
    let mut options: Vec<String> = args.clone();

    // Check if first argument is option
    if args.len() > 0 && &args[0][..1] != "-"  {
        options = args[1..].to_vec();

        match &args[0][..] {
            // Start REPL
            "repl" => instruction.run_repl = true,

            // Initialize `main.eo` with an example program
            "init" => {
                // Create and write to `main.eo`
                let mut file = File::create("main.eo").expect(
                    "\x1b[1m\x1b[31mError in creating `main.eo`\x1b[0m"
                );
                file.write(b"var a = {\n    2 + 3;\n};").expect(
                    "\x1b[1m\x1b[31mError in writing to file `main.eo`\x1b[0m"
                );

                // Print on success
                println!("Initialized \x1b[1m`main.eo`\x1b[0m");
            }

            // Help screen
            "help"|"usage" => {
                printdoc!("
                    \x1b[1m\x1b[33mUsage: europa `file?` `options?`

                    \x1b[0m\x1b[1mOptions:
                        \x1b[0m-v, --version: Print version to console

                    \x1b[1mInstructions:
                        \x1b[0mhelp, usage: List commands and usage
                ");
            }

            // First argument passed was file
            instr => instruction.file = String::from(instr)
        }
    }

    // Iterate over arguments
    for option in options.iter() {
        match &option[..] {
            // Print version in Cargo.toml to console
            "-v"|"--version" => println!("\x1b[33mEuropa Lang {}\x1b[0m", env!("CARGO_PKG_VERSION")),

            // Run REPL after code run
            "-r"|"--repl" => instruction.run_repl = true,

            // Debug statements
            "-d"|"--debug" => instruction.debug = true,

            // Other
            _ => printdoc!("
                \x1b[1m\x1b[31mUnknown option \x1b[0m'{}'
                \x1b[0mTry \x1b[1m`europa help`\x1b[0m or \x1b[1m`europa usage`\x1b[0m for usage
            ", option)
        }
    }

    instruction
}