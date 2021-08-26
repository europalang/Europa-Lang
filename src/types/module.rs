use std::collections::HashMap;

use crate::functions::FuncType;

#[derive(Debug, Clone)]
pub struct ModImport {
    pub name: String,
    pub fns: HashMap<String, FuncType>,
}

impl ModImport {
    pub fn new(name: String, fns: HashMap<String, FuncType>) -> Self {
        Self { name, fns }
    }

    pub fn to_string(&self, idt: usize) -> String {
        let mut out = String::from(format!("mod {} {{\n", self.name));

        for (key, _) in &self.fns {
            let itm = format!("{}()", key);
            out += &format!("{}{},\n", " ".repeat(idt), itm);
        }

        out += &"  ".repeat(idt - 1);
        out += "}";

        out
    }
}
