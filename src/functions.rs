use std::{fmt::Debug, rc::Rc};

use crate::{
    environment::Environment,
    error::Error,
    interpreter::Interpreter,
    nodes::stmt::Stmt,
    token::{TType, Token},
    types::Type,
};

type FResult = Result<Type, Error>;

pub trait Call {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult;
    fn to_string(&self) -> String;
}

// native functions
#[derive(Clone)]
pub struct Func {
    args: usize,
    exec: Rc<dyn Fn(&mut Interpreter, Vec<Type>) -> Result<Type, Error>>,
}

impl Func {
    pub fn new(
        func: Rc<dyn Fn(&mut Interpreter, Vec<Type>) -> Result<Type, Error>>,
        args: usize,
    ) -> Self {
        Self { exec: func, args }
    }
}

impl Call for Func {
    fn arity(&self) -> usize {
        self.args
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult {
        (self.exec)(interpreter, args)
    }

    fn to_string(&self) -> String {
        "<Native Fn>".into()
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Native Function")
    }
}

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

// exported type
#[derive(Debug, Clone)]
pub enum FuncType {
    Native(Func),
    User(FuncCallable),
}

impl Call for FuncType {
    fn arity(&self) -> usize {
        match self {
            Self::Native(n) => n.arity(),
            Self::User(n) => n.arity(),
        }
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult {
        match self {
            Self::Native(n) => n.call(interpreter, args),
            Self::User(n) => n.call(interpreter, args),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Native(n) => n.to_string(),
            Self::User(n) => n.to_string(),
        }
    }
}
