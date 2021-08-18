use crate::error::ErrorType;

use super::{ops::TResult, Type};

#[derive(Debug, Clone)]
pub struct Array {
    pub arr: Vec<Type>,
}

impl Array {
    pub fn new(arr: Vec<Type>) -> Self {
        Self { arr }
    }

    pub fn get(&self, i: Type) -> TResult {
        let i = self.check_index(i)?;
        Ok(self.arr[i].clone())
    }

    pub fn set(&mut self, i: Type, v: Type) -> Result<(), (String, ErrorType)> {
        let i = self.check_index(i)?;
        self.arr[i] = v.clone();
        Ok(())
    }

    fn check_index(&self, num: Type) -> Result<usize, (String, ErrorType)> {
        let len = self.arr.len();

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
