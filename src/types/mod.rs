use crate::functions::FuncType;
use array::Array;

pub mod tostring;
pub mod ops;
pub mod hash;
pub mod array;


#[derive(Debug, Clone)]
pub enum Type {
    Float(f32),
    String(String),
    Bool(bool),
    Array(Array),
    Func(FuncType),
    Nil,
}
