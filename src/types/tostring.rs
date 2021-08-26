use crate::functions::Call;

use super::Type;

impl Type {
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
            Type::Func(n) => n.to_string(),
            Type::Module(n) => {
                n.to_string(1)
            }
        }
    }
}