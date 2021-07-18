use std::{fmt::Debug, rc::Rc};

use crate::{error::Error, interpreter::Interpreter, types::Type};

type FResult = Result<Type, Error>;

pub trait Call {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult;
    fn to_string(&self) -> String;
}

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
        "aoeu".into()
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

// exported type
#[derive(Debug, Clone)]
pub enum FuncType {
    Native(Func),
    // User(FuncCallable)
}

impl Call for FuncType {
    fn arity(&self) -> usize {
        match self {
            Self::Native(n) => n.arity(),
        }
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult {
        match self {
            Self::Native(n) => n.call(interpreter, args),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Native(n) => n.to_string(),
        }
    }
}
