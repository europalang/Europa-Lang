use std::{
    io::{stdin, stdout, Write},
    rc::Rc,
};

use maplit::hashmap;

use crate::{
    functions::{Func, FuncType},
    native_func,
    types::module::Module,
    types::Type,
};

pub fn new() -> Module {
    Module {
        name: "io".into(),
        fns: hashmap! {
            "println".into() => native_func!(|_, args, _| {
                println!("{}", args[0].to_string());
                Ok(Type::Nil)
            }, 1),
            "print".into() => native_func!(|_, args, _| {
                print!("{}", args[0].to_string());
                Ok(Type::Nil)
            }, 1),
            "flush".into() => native_func!(|_, _, _| {
                stdout().flush().expect("Failed to flush.");
                Ok(Type::Nil)
            }, 0),
            "readln".into() => native_func!(|_, args, _| {
                let msg = args[0].to_string();
                print!("{}", msg);
                stdout().flush().expect("Failed to flush.");

                let mut out = String::new();
                stdin().read_line(&mut out).expect("Failed to read user input.");

                Ok(Type::String(out.trim().to_string()))
            }, 1),
            "exit".into() => native_func!(|_, args, _| {
                std::process::exit(match args[0] {
                    Type::Float(value) => value as i32,
                    _ => {
                        eprintln!("Expected a number for the exit code.");
                        0
                    }
                });
            }, 1),
        },
    }
}
