use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH, Duration},
};
use chrono::{DateTime, Utc};

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
            }, 0),
            "fmt".into() => native_func!(|_, args, _| {

                let millis: f32 = match args[0] {
                    Type::Float(value) => value,
                    _ => {
                        eprintln!("Invalid argument passed to clock.fmt!");
                        0.0
                    }
                };
                let datetime: DateTime<Utc> = (UNIX_EPOCH + Duration::from_millis(millis as u64)).into();

                Ok(Type::String(datetime.format(args[1].to_string().as_str()).to_string()))
            }, 2)
        },
    }
}
