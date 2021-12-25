use std::{collections::HashMap, fmt::Debug};

use crate::{
    error::ErrorType,
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
    optional_args: HashMap<String, Type>,
    block: Vec<Stmt>,
}

impl FuncCallable {
    pub fn new(
        name: Token,
        args: Vec<Token>,
        optional_args: HashMap<String, Type>,
        block: Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            args,
            optional_args,
            block,
        }
    }
}

impl Call for FuncCallable {
    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Type>,
        opt_args: HashMap<String, Type>,
    ) -> FResult {
        interpreter.environ.push_scope();

        for (i, name) in self.args.iter().enumerate() {
            match &name.ttype {
                TType::Identifier(n) => interpreter.environ.define(&n, &args[i]),
                _ => panic!(),
            }
        }

        for (name, val) in self.optional_args.iter() {
            interpreter.environ.define(
                name,
                match &opt_args.get(name) {
                    Some(t) => t,
                    None => val,
                },
            );
        }

        let out = interpreter.eval_block(&self.block, false);
        interpreter.environ.pop_scope();

        return match out {
            Ok(v) => match v {
                Some(v) => Ok(v),
                _ => Ok(Type::Nil),
            },
            Err(e) => match e.error_type {
                ErrorType::Return(v) => Ok(v),
                _ => Err(e),
            },
        };

        // if let Err(e) = out {
        //     if let ErrorType::Return(v) = e.error_type {
        //         return Ok(v)
        //     }

        //     return Err(e)
        // }

        // Ok(Type::Nil)
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

    fn name(&self) -> String {
        match &self.name.ttype {
            TType::Identifier(name) => name.clone(),
            _ => panic!(),
        }
    }
}

impl Debug for FuncCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Native Function")
    }
}
