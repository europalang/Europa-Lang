use std::rc::Rc;

use rand::{Rng, thread_rng};

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
            // TODO: add more functions
            // Trigonometric functions
            "sin".into() => native_func!(|_, args| {
                Ok(Type::Float(args[0].parse::<f32>()
                    .expect("Invalid Argument: Type is not Float").sin()))
            }, 1),
            "cos".into() => native_func!(|_, args| {
                Ok(Type::Float(args[0].parse::<f32>()
                    .expect("Invalid Argument: Type is not Float").cos()))
            }, 1),
            "tan".into() => native_func!(|_, args| {
                Ok(Type::Float(args[0].parse::<f32>()
                    .expect("Invalid Argument: Type is not Float").tan()))
            }, 1),

            // RNG functions
            "randint".into() => native_func!(|_, args| {
                Ok(Type::Float(thread_rng().gen_range(
                    args[0].parse::<i32>()
                        .expect("Invalid Argument: Type is not Int")..
                    args[1].parse::<i32>()
                        .expect("Invalid Argument: Type is not Int")
                ) as f32))
            }, 2)
        },
    }
}
