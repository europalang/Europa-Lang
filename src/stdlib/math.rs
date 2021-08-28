use std::rc::Rc;

use maplit::hashmap;

use crate::{
    functions::{Func, FuncType},
    native_func,
    types::module::Module,
    types::Type,
};

pub fn new() -> Module {
    Module {
        name: "math".into(),
        fns: hashmap! {
            // nums
            "infinity".into() => Type::Float(f32::INFINITY),
            "nan".into() => Type::Float(f32::NAN),

            // funcs
            "sin".into() => native_func!(|_, _args| {
                Ok(Type::Float(3f32))
            }, 1)
        },
    }
}
