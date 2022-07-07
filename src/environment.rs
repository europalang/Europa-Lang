use std::collections::HashMap;

use crate::{types::Type, token::{Token, TType}, error::{Error, ErrorType}};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Type>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, val: Type) {
        self.values.insert(name, val);
    }

    pub fn get(&self, name: Token) -> Result<Type, Error> {
        let ident = match name.ttype {
            TType::Identifier(n) => n,
            _ => unreachable!()
        };

        if self.values.contains_key(&ident) {
            Ok(self.values[&ident])
        } else {
            Err(Error::new(name.lineinfo, format!("Undefined variable '{}'", ident), ErrorType::ReferenceError))
        }
    }
}