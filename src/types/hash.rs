use super::Type;
use std::cmp::Ordering;

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::String(a), Type::String(b)) => a == b,
            (Type::Float(a), Type::Float(b)) => a == b,
            (Type::Nil, Type::Nil) => true,
            (Type::Bool(a), Type::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Type::String(a), Type::String(b)) => a.len().partial_cmp(&b.len()),
            (Type::Float(a), Type::Float(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// todo: hash
