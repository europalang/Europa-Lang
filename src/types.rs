use crate::{error::ErrorType, functions::FuncType};

use std::cmp::Ordering;

type TResult = Result<Type, (String, ErrorType)>;

#[derive(Debug, Clone)]
pub enum Type {
    Float(f32),
    String(String),
    Bool(bool),
    Array(Vec<Type>),
    Func(FuncType),
    Nil,
}

impl Type {
    pub fn add(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            return Ok(Self::Float(a + b));
        }

        if let (Self::String(a), Self::Float(b)) = (self, other) {
            return Ok(Self::String(format!("{}{}", a, b)));
        }

        if let (Self::Float(a), Self::String(b)) = (self, other) {
            return Ok(Self::String(format!("{}{}", a, b)));
        }

        if let (Self::String(a), Self::String(b)) = (self, other) {
            return Ok(Self::String(format!("{}{}", a, b)));
        }

        Err((
            "Operator '+' can only be applied to strings and numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn sub(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            return Ok(Self::Float(a - b));
        }

        Err((
            "Operator '-' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn mult(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            return Ok(Self::Float(a * b));
        }

        Err((
            "Operator '*' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn div(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            if *b == 0f32 {
                return Err(("Division by 0.".into(), ErrorType::MathError));
            }
            return Ok(Self::Float(a / b));
        }

        Err((
            "Operator '/' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn modulo(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            if *b == 0f32 {
                return Err(("Division by 0.".into(), ErrorType::MathError));
            }
            return Ok(Self::Float(a % b));
        }

        Err((
            "Operator '%' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn index(&self, num: Type) -> TResult {
        match self {
            Self::Array(v) => {
                let val = &v[self.validate_index(num, v.len())?];

                Ok(val.clone())
            }

            _ => Err((
                "The [...] operator can only be applied to arrays and maps.".into(),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn assign(&mut self, i: Type, value: Type) -> TResult {
        match self {
            Type::Array(v) => {
                let len = v.len();
                v[self.validate_index(i, len)?] = value;
                Ok(value)
            }
            _ => Err((
                "The [...]= operator can only be applied to arrays and maps.".into(),
                ErrorType::ReferenceError,
            )),
        }
    }

    // private
    fn validate_index(&self, num: Type, len: usize) -> Result<usize, (String, ErrorType)> {
        match num {
            Type::Float(i) => {
                if i.is_infinite() || i.is_nan() || // infinite
                                    i.round() != i
                // not whole
                {
                    return Err((
                        format!("Only whole numbers are valid index ranges (got {}).", i).into(),
                        ErrorType::TypeError,
                    ));
                }

                let idx;

                if i < 0f32 {
                    idx = len as f32 + i;
                } else {
                    idx = i;
                }

                if idx < 0f32 || idx as usize >= len {
                    return Err((
                        format!("Index {} out of array range 0-{}.", i, len - 1).into(),
                        ErrorType::ReferenceError,
                    ));
                }

                Ok(idx as usize)
            }
            _ => {
                return Err((
                    "Arrays can only be indexed with numbers.".into(),
                    ErrorType::TypeError,
                ))
            }
        }
    }
}

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
