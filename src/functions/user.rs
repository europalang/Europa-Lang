use std::fmt::Debug;

use crate::{
    environment::Environment,
    interpreter::Interpreter,
    nodes::stmt::Stmt,
    token::{TType, Token},
    types::Type,
    error::ErrorType
};

use super::traits::{Call, FResult};

// user-defined functions
#[derive(Clone)]
pub struct FuncCallable {
    name: Token,
    args: Vec<Token>,
    block: Vec<Stmt>,
}

impl FuncCallable {
    pub fn new(name: Token, args: Vec<Token>, block: Vec<Stmt>) -> Self {
        Self { name, args, block }
    }
}

impl Call for FuncCallable {
    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult {
        // here we imply that any variables changed outside of the function will silently be ignored
        // todo: fix!
        let mut env = Environment::new_enclosing(interpreter.environ.clone());

        for (i, name) in self.args.iter().enumerate() {
            match &name.ttype {
                TType::Identifier(n) => env.define(&n, &args[i]),
                _ => panic!(),
            }
        }

        let out = interpreter.eval_block(&self.block, false);

        if let Err(e) = out {
            if let ErrorType::Return(v) = e.error_type {
                return Ok(v)
            }

            return Err(e)
        }

        Ok(Type::Nil)
    }

    fn to_string(&self) -> String {
        format!(
            "<User Fn {}>",
            match &self.name.ttype {
                TType::Identifier(s) => s,
                _ => panic!(),
            }
        )
        .into()
    }
}

impl Debug for FuncCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Native Function")
    }
}
