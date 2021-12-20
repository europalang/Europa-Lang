use crate::{interpreter::Interpreter, types::Type};

use std::fmt::{self, Display};

pub use self::{
    native::Func,
    traits::{Call, FResult},
    user::FuncCallable,
};

mod native;
mod traits;
mod user;

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

    fn name(&self) -> String {
        match self {
            Self::Native(n) => n.name(),
            Self::User(n) => n.name(),
        }
    }
}

impl Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {} [{} args]", self.name(), self.arity())
    }
}
