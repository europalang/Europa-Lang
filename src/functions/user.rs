use std::fmt::Debug;

use crate::{
    environment::Environment,
    interpreter::Interpreter,
    nodes::stmt::Stmt,
    token::{TType, Token},
    types::Type,
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
        let mut env = Environment::new_enclosing(interpreter.environ.clone());

        for (i, name) in self.args.iter().enumerate() {
            match &name.ttype {
                TType::Identifier(n) => env.define(&n, &args[i]),
                _ => panic!(),
            }
        }

        interpreter.eval_block(Box::new(env.clone()), &self.block, true)?;

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
