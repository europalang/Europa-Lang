use std::collections::HashMap;

use crate::{error::Error, interpreter::Interpreter, types::Type};

pub type FResult = Result<Type, Error>;

pub trait Call {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>, opt_args: HashMap<String, Type>) -> FResult;
    fn to_string(&self) -> String;
    fn name(&self) -> String;
}
