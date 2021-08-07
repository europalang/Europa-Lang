use super::Type;
use crate::error::ErrorType;

pub type TResult = Result<Type, (String, ErrorType)>;

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

    pub fn pow(&self, other: &Type) -> TResult {
        if let (Self::Float(a), Self::Float(b)) = (self, other) {
            if *b == 0f32 {
                return Err(("Division by 0.".into(), ErrorType::MathError));
            }
            return Ok(Self::Float(a.powf(*b)));
        }

        Err((
            "Operator '%' can only be applied to numbers.".into(),
            ErrorType::TypeError,
        ))
    }

    // arrays and maps
    pub fn index(&self, num: Type) -> TResult {
        match self {
            Self::Array(v) => {
                v.get(num)
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
                v.set(i, value)
            }
            _ => Err((
                "The [...]= operator can only be applied to arrays and maps.".into(),
                ErrorType::ReferenceError,
            )),
        }
    }
}