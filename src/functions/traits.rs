use crate::{error::ErrorType, interpreter::Interpreter, types::Type};

pub type FResult = Result<Type, (String, ErrorType)>;

pub trait Call {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> FResult;
    fn to_string(&self) -> String;
    fn name(&self) -> String;
}
