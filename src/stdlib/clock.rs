use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use maplit::hashmap;

use crate::{
    functions::{Func, FuncType},
    native_func,
    types::module::Module,
    types::Type,
};

pub fn new() -> Module {
    Module {
        name: "clock".into(),
        fns: hashmap! {
            "now".into() => native_func!(|_, _, _| {
                let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting time.");
                Ok(Type::Float(start.as_millis() as f32))
            }, 0)
        },
    }
}
