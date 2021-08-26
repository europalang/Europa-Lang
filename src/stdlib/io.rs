use std::{
    io::{stdout, Write, stdin},
    rc::Rc,
};

use maplit::hashmap;

use crate::{
    functions::{Func, FuncType},
    interpreter::Interpreter,
    types::Type,
};

use super::Module;

pub fn new() -> Module {
    Module {
        fns: hashmap! {
            "println".into() => FuncType::Native(Func::new(Rc::new(|_: &mut Interpreter, args: Vec<Type>| {
                println!("{}", args[0].to_string());
                Ok(Type::Nil)
            }), 1)),
            "print".into() => FuncType::Native(Func::new(Rc::new(|_: &mut Interpreter, args: Vec<Type>| {
                print!("{}", args[0].to_string());
                Ok(Type::Nil)
            }), 1)),
            "flush".into() => FuncType::Native(Func::new(Rc::new(|_: &mut Interpreter, _: Vec<Type>| {
                stdout().flush().expect("Failed to flush.");
                Ok(Type::Nil)
            }), 0)),
            "readln".into() => FuncType::Native(Func::new(Rc::new(|_: &mut Interpreter, args: Vec<Type>| {
                let msg = args[0].to_string();
                print!("{}", msg);
                stdout().flush().expect("Failed to flush.");

                let mut out = String::new();
                stdin().read_line(&mut out).expect("Failed to read user input.");

                Ok(Type::String(out.trim().to_string()))
            }), 1))
        },
    }
}
