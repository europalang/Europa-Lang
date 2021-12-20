use std::path::Path;
use std::time::Instant;
use std::{env, process};

use crate::environment::Environment;
use crate::error::{Error, LineInfo};
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::token::{Token, TType};
use crate::types::Type;

use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{
    Cmd, ConditionalEventHandler, Editor, Event, EventContext, KeyCode, KeyEvent,
    Modifiers, RepeatCount,
};

pub fn init(mut environ: Environment, verbose: bool) {
    let mut code; let mut tokens;

    let history_file = if cfg!(windows) {
        if let Ok(dir) = env::var("USERPROFILE") {
            Some(format!("{}\\.europa_history", dir))
        } else if let Ok(dir) = env::var("DEFAULTUSERPROFILE") {
            Some(format!("{}\\.europa_history", dir))
        } else { None }
    } else if cfg!(unix) {
        if let Ok(home) = env::var("HOME") {
            Some(format!("{}/.europa_history", home))
        } else { None }
    } else { None };

    let mut rl = Editor::<()>::new();
    rl.set_auto_add_history(true);
    rl.set_tab_stop(4);
    rl.set_indent_size(4);

    if let Some(history_file) = &history_file {
        if Path::new(&history_file).exists() {
            let _ = rl.load_history(&history_file);
        }
    }

    'main_loop: loop {
        tokens = Vec::new();
        code = String::new();

        let mut line = 1;

        while tokens.len() == 0 || has_unclosed_brackets(&tokens) {
            let prompt = match tokens.len() {
                0 => "> ",
                _ => {
                    tokens.pop().unwrap(); // remove the EOF
                    "... "
                },
            };

            let read = match rl.readline(prompt) {
                Ok(read) => read,
                Err(ReadlineError::Eof) => break 'main_loop,
                Err(ReadlineError::Interrupted) => continue 'main_loop,
                Err(err) => {
                    eprintln!("Unexpected error: {}", err);
                    process::exit(1);
                },
            };

            if read == ".exit" {
                break 'main_loop
            }

            if let Some(history_file) = &history_file {
                let _ = rl.append_history(&history_file);
            }

            code.push_str(&read);
            code.push('\n');

            let mut lexer = Lexer::new(&read);
            lexer.set_lineinfo(LineInfo::new(line, 0));
            match lexer.init() {
                Ok(mut lexed) => tokens.append(&mut lexed),
                Err(error) => {
                    error.display(&read);
                    continue 'main_loop
                },
            }

            line += 1;
        }

        match run_code(&tokens, &mut environ, verbose) {
            Err(error) => error.display(&code),
            Ok(eval) => if eval != Type::Nil {
                println!("{}", eval);
            },
        }
    }
}

fn run_code(
    code: &[Token],
    environ: &mut Environment,
    verbose: bool,
) -> Result<Type, Error> {
    // Turn tokens into AST
    let mut time = Instant::now();
    let tree = Parser::new(code.to_vec()).init()?;

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

fn has_unclosed_brackets(code: &[Token]) -> bool {
    #[derive(Clone, Copy, PartialEq)]
    enum BracketType {
        BBrace,
        Brace,
        Paren,
        Brack,
    }

    let mut stack = Vec::new();

    for token in code {
        match token.ttype {
            TType::LeftBBrace => stack.push(BracketType::BBrace),
            TType::LeftBrace => stack.push(BracketType::Brace),
            TType::LeftParen => stack.push(BracketType::Paren),
            TType::LeftBrack => stack.push(BracketType::Brack),
            TType::RightBBrace => if stack.pop() != Some(BracketType::BBrace) {
                return false
            },
            TType::RightBrace => if stack.pop() != Some(BracketType::Brace) {
                return false
            },
            TType::RightParen => if stack.pop() != Some(BracketType::Paren) {
                return false
            },
            TType::RightBrack => if stack.pop() != Some(BracketType::Brack) {
                return false
            },
            _ => (),
        }
    }

    stack.len() > 0
}
