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
    Nil
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
            if *b == 0f32 { return Err(("Division by 0.".into(), ErrorType::MathError)) }
            return Ok(Self::Float(a / b));
        }

        Err((
            "Operator '/' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    pub fn modulo(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            if *b == 0f32 { return Err(("Division by 0.".into(), ErrorType::MathError)) }
            return Ok(Self::Float(a % b));
        }

        Err((
            "Operator '%' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::String(a), Type::String(b)) => a == b,
            (Type::Float(a), Type::Float(b)) => a == b,
            (Type::Nil, Type::Nil) => true,
            (Type::Bool(a), Type::Bool(b)) => a == b,
            _ => false
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Type::String(a), Type::String(b)) => a.len().partial_cmp(&b.len()),
            (Type::Float(a), Type::Float(b)) => a.partial_cmp(b),
            _ => None
        }
    }
}