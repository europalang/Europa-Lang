use std::{cell::RefCell, rc::Rc};

use crate::functions::FuncType;
use array::Array;
use map::Map;

pub mod map;
pub mod tostring;
pub mod ops;
pub mod hash;
pub mod array;


#[derive(Debug, Clone)]
pub enum Type {
    Float(f32),
    String(String),
    Bool(bool),
    Array(Rc<RefCell<Array>>),
    Map(Rc<RefCell<Map>>),
    Func(FuncType),
    Nil,
}
