use std::collections::HashMap;

use super::Type;

use std::fmt::{ self, Display };

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub fns: HashMap<String, Type>,
}

impl Module {
    pub fn new(name: String, fns: HashMap<String, Type>) -> Self {
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

impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mod {} [{} items]", self.name, self.fns.len())
    }
}
