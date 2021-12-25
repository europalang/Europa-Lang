use std::{cell::RefCell, rc::Rc};

use crate::functions::FuncType;
use array::Array;
use map::Map;
use module::Module;

pub mod array;
pub mod hash;
pub mod map;
pub mod module;
pub mod ops;
pub mod tostring;

#[derive(Debug, Clone)]
pub enum Type {
    Float(f32),
    String(String),
    Bool(bool),
    Array(Rc<RefCell<Array>>),
    Map(Rc<RefCell<Map>>),
    Module(Module),
    Func(FuncType),
    Nil,
}
