use std::rc::Rc;

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
                }), 1))
        },
    }
}
