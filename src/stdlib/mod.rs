/*
    The stdlib is the entry point. It contains a hashmap of modules.
    Each module can contain a function bound to a function name.
*/

use std::collections::HashMap;

use maplit::hashmap;

use crate::types::module::Module;

mod io;
mod math;
mod clock;

/**
Easier coding.

# Usage
```
native_func!(|interpreter, args| {
    // ...
}, 1)
```

## Explanation
`interpreter` is the interpreter instance, use `_` if not using this argument.
`args` is a `Vec<Type>`, use `_` if not using this argument.

*/
#[macro_export]
macro_rules! native_func {
    ($func:expr, $arity:expr) => {
        native_func!("<native function>", $func, $arity)
    };

    ($name:literal, $func:expr, $arity:expr) => {
        Type::Func(FuncType::Native(Func::new(
            $name,
            Rc::new($func),
            $arity,
        )))
    };
}

#[derive(Clone)]
pub struct Stdlib {
    pub mods: HashMap<String, Module>,
}

impl Stdlib {
    pub fn new() -> Self {
        Stdlib {
            mods: hashmap! {
                "io".into() => io::new(),
                "math".into() => math::new(),
                "clock".into() => clock::new(),
            },
        }
    }
}
