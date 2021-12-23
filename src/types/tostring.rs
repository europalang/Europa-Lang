use crate::functions::Call;

use super::Type;

use std::fmt::{ self, Display };
use std::str::FromStr;

impl Type {
    // wtf is this????
    pub fn to_string(&self) -> String {
        match self {
            Type::Array(v) => {
                let mut out = String::from('[');

                for (idx, val) in v.borrow().arr.iter().enumerate() {
                    out += &val.to_string();

                    if idx < v.borrow().arr.len() - 1 {
                        out += ", ";
                    }
                }

                out + "]"
            }
            Type::Map(n) => {
                n.borrow().to_string(1)
            }
            Type::Nil => "nil".into(),
            Type::Float(n) => n.to_string(),
            Type::String(n) => n.clone(),
            Type::Bool(n) => n.to_string(),
            Type::Func(n) => Call::to_string(n),
            Type::Module(n) => {
                n.to_string(1)
            }
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            Type::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn parse_float(&self) -> Result<f32, String>{
        match self {
            Type::Nil => Ok(0.0),
            Type::Float(n) => Ok(*n),
            Type::String(n) => {
                f32::from_str(n).map_err(|e| e.to_string())
            }
            Type::Bool(n) => Ok(*n as i32 as f32),
            _ => Err(format!("{} is inconvertable to Type Float", self))
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array(items) => {
                let items = &items.borrow().arr;

                write!(f, "[")?;

                for (i, item) in items.iter().enumerate() {
                    write!(f, "{}", item)?;

                    if i + 1 < items.len() {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            },
            Self::Map(items) => {
                let items = &items.borrow().map;

                write!(f, "{{")?;

                for (key, value) in items.iter() {
                    write!(f, "\n\t{}: {},", Self::String(key.clone()), value)?;
                }

                write!(f, "\n}}")
            },
            Self::Nil => write!(f, "nil"),
            Self::Float(value) => write!(f, "{}", value),
            Self::String(value) => write!(
                f,
                "\"{}\"",
                value
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r"),
            ),
            Self::Bool(value) => write!(f, "{}", value),
            Self::Func(function) => write!(f, "{}", function),
            Self::Module(module) => write!(f, "{}", module),
        }
    }
}
