use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    token::{TType, Token},
    types::Type,
};

#[derive(Debug)]
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
                        format!("Undefined variable {}", name),
                        ErrorType::TypeError,
                    ))
                }
            }
            _ => panic!(),
        }
    }

    pub fn define(&mut self, name: &String, val: &Type) {
        self.values.insert(name.clone(), val.clone());
    }

    pub fn assign(&mut self, name: &Token, val: &Type) -> Result<(), Error> {
        let ttype = name.ttype.clone();

        match ttype {
            TType::Identifier(k) => {
                if self.values.contains_key(&k) {
                    self.values.insert(k, val.clone());
                    return Ok(());
                }
                
                Err(Error::new(
                    name.lineinfo,
                    format!("Undefined variable {}", k).into(),
                    ErrorType::TypeError,
                ))
            }
            _ => panic!(),
        }
    }
}
