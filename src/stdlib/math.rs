use std::rc::Rc;
use rand::Rng;
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
            }, 1),
            "random".into() => native_func!(|_, args| {
                println!("{:?}", args);
                let range1 = args[0].to_string().parse::<i32>().unwrap();
                let range2 = args[1].to_string().parse::<i32>().unwrap();
                println!("{}", range1);
                let mut rng = rand::thread_rng();
                Ok(Type::Float(rng.gen_range(range1..range2) as f32))
            }, 1),
        },
    }
}
