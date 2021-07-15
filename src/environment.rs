use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    token::{TType, Token},
    types::Type,
};

pub struct Environment {
    values: HashMap<String, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, tok: &Token) -> Result<Type, Error> {
        match &tok.ttype {
            TType::Identifier(name) => {
                if self.values.contains_key(name) {
                    return Ok(self.values[name].clone());
                } else {
                    Err(Error::new(
                        tok.lineinfo,
                        format!("{}", name),
                        ErrorType::TypeError,
                    ))
                }
            }
            _ => panic!()
        }
    }

    pub fn define(&mut self, name: &String, val: &Type) {
        let env = &mut self.values;
        env.insert(name.clone(), val.clone());
    }
}
