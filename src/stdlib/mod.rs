/*
    The stdlib is the entry point. It contains a hashmap of modules.
    Each module can contain a function bound to a function name.
*/

use std::collections::HashMap;

use maplit::hashmap;

use crate::functions::FuncType;

mod io;

#[derive(Debug, Clone)]
pub struct Module {
    pub fns: HashMap<String, FuncType>,
}

#[derive(Clone)]
pub struct Stdlib {
    pub mods: HashMap<String, Module>,
}

impl Stdlib {
    pub fn new() -> Self {
        Stdlib {
            mods: hashmap! {
                "io".into() => io::new()
            },
        }
    }
}
