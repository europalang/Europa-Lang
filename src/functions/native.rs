use std::{fmt::Debug, rc::Rc};

use crate::{error::Error, interpreter::Interpreter, types::Type};

use super::traits::{Call, FResult};

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
