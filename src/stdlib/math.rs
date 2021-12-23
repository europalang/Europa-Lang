use std::rc::Rc;

use rand::{Rng, thread_rng};

use maplit::hashmap;

use crate::{
    functions::{Func, FuncType},
    native_func,
    types::module::Module,
    types::Type,
    error::{ErrorType},
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
                match args[0].parse::<f32>() {
                    Ok(x) => Ok(Type::Float(x.sin())),
                    Err(_) => Err((
                        "Incorrect Argument Type: Type is not Float".into(),
                        ErrorType::TypeError,
                    )),
                }
            }, 1),
            "cos".into() => native_func!(|_, args| {
                match args[0].parse::<f32>() {
                    Ok(x) => Ok(Type::Float(x.cos())),
                    Err(_) => Err((
                        "Incorrect Argument Type: Type is not Float".into(),
                        ErrorType::TypeError,
                    )),
                }
            }, 1),
            "tan".into() => native_func!(|_, args| {
                match args[0].parse::<f32>() {
                    Ok(x) => Ok(Type::Float(x.tan())),
                    Err(_) => Err((
                        "Incorrect Argument Type: Type is not Float".into(),
                        ErrorType::TypeError,
                    )),
                }
            }, 1),

            // RNG functions
            "randrange".into() => native_func!(|_, args| {
                if let (Ok(min), Ok(max)) = (args[0].parse::<f32>(), args[1].parse::<f32>()) {
                    Ok(Type::Float(thread_rng().gen_range(min..max)))
                } else {
                    Err((
                        "Incorrect Argument Type: Type is not Int".into(),
                        ErrorType::TypeError,
                    ))
                }
            }, 2)
        },
    }
}
