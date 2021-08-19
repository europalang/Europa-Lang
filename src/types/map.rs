use std::{collections::HashMap, rc::Rc};

use crate::error::ErrorType;

use super::{ops::TResult, Type};

#[derive(Debug, Clone)]
pub struct Map {
    pub map: HashMap<String, Type>,
}

impl Map {
    pub fn new(map: HashMap<String, Type>) -> Self {
        Self { map }
    }

    pub fn get(&self, key: Type) -> TResult {
        let key = self.check_index(key)?;
        let val = self.map.get(&key);

        match val {
            Some(val) => Ok(match val {
                Type::Array(v) => Type::Array(Rc::clone(v)),
                Type::Map(v) => Type::Map(Rc::clone(v)),
                _ => val.clone()
            }),
            _ => Err((
                format!("'{}' is not a key in the map.", key,),
                ErrorType::ReferenceError,
            )),
        }
    }

    pub fn set(&mut self, key: Type, value: Type) -> Result<(), (String, ErrorType)> {
        let key = self.check_index(key)?;
        self.map.insert(key, value);
        Ok(())
    }

    pub fn to_string(&self, idt: usize) -> String {
        let mut out = String::from("{{\n");

        for (key, value) in &self.map {
            out += &format!("{}\"{}\": {},\n", "  ".repeat(idt), key, match value {
                Type::Map(v) => {
                    v.borrow().to_string(idt + 1)
                },
                _ => value.to_string()
            });
        }

        out += &"  ".repeat(idt - 1);
        out += "}}";

        out
    }

    fn check_index(&self, key: Type) -> Result<String, (String, ErrorType)> {
        match key {
            _ => Ok(key.to_string()),
        }
    }
}
