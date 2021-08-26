use std::collections::HashMap;

use crate::functions::FuncType;

#[derive(Debug, Clone)]
pub struct ModImport {
    pub fns: HashMap<String, FuncType>,
}

impl ModImport {
    pub fn new(fns: HashMap<String, FuncType>) -> Self {
        Self { fns }
    }

    pub fn to_string(&self, idt: usize) -> String {
        let mut out = String::from("mod {\n");

        for (key, _) in &self.fns {
            let itm = format!("{}()", key);
            out += &format!("{}{},\n", " ".repeat(idt), itm);
        }

        out += &"  ".repeat(idt - 1);
        out += "}";

        out
    }
}
