use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    token::{TType, Token},
    types::Type,
};

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Type>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn get(&self, tok: &Token) -> Result<Type, Error> {
        if let TType::Identifier(name) = &tok.ttype {
            let mut scopes = self.scopes.clone();
            scopes.reverse();

            for scope in scopes {
                if scope.contains_key(name) {
                    return Ok(scope[name].clone());
                }
            }

            Err(Error::new(
                tok.lineinfo,
                format!("Undefined variable {}", name),
                ErrorType::TypeError,
            ))
        } else {
            panic!();
        }
    }

    pub fn get_at(&mut self, distance: usize, name: &Token) -> Type {
        if let TType::Identifier(n) = &name.ttype {
            self.ancestor(distance).get(n).unwrap().clone()
        } else {
            panic!()
        }
    }

    fn ancestor(&mut self, distance: usize) -> &mut HashMap<String, Type> {
        let distance = self.scopes.len() - 1 - distance;

        &mut self.scopes[distance]
    }

    pub fn define(&mut self, name: &String, val: &Type) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.clone(), val.clone());
    }

    pub fn assign(&mut self, name: &Token, val: &Type) -> Result<(), Error> {
        if let TType::Identifier(n) = name.ttype.clone() {
            let mut scopes = self.scopes.clone();
            scopes.reverse();

            for (i, scope) in scopes.iter().enumerate() {
                if scope.contains_key(&n) {
                    let mut scope = self.scopes[i].clone();
                    scope.insert(n, val.clone());
                    self.scopes[i] = scope;
                    return Ok(());
                }
            }

            Err(Error::new(
                name.lineinfo,
                format!("Undefined variable {}", n),
                ErrorType::ReferenceError,
            ))
        } else {
            panic!();
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, val: &Type) -> Result<(), Error> {
        if let TType::Identifier(n) = &name.ttype {
            self.ancestor(distance).insert(n.clone(), val.clone());
        } else {
            panic!();
        }

        Ok(())
    }
}
