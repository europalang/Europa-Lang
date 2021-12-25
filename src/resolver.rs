use crate::{
    error::{Error, ErrorType},
    interpreter::Interpreter,
    nodes::{
        expr::Expr,
        stmt::{ImportType, Stmt},
    },
    token::{TType, Token},
};

pub struct Resolver {
    scopes: Vec<Vec<String>>,
    interpreter: Interpreter,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
        }
    }

    pub fn init(&mut self) -> Result<Interpreter, Error> {
        let nodes = self.interpreter.nodes.clone();

        self.resolves(&nodes)?;

        Ok(self.interpreter.clone())
    }

    // resolve
    fn resolve_stmt(&mut self, node: &Stmt) -> Result<(), Error> {
        match node {
            Stmt::ExprStmt(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::VarDecl(vars) => {
                for (name, val) in vars {
                    self.resolve_expr(val)?;
                    self.define(name);
                }
            }
            Stmt::Block(stmts) => {
                self.resolves(stmts)?;
            }
            Stmt::IfStmt(cond, true_br, elif_brs, else_br) => {
                self.resolve_if(cond, true_br, elif_brs, else_br)?;
            }
            Stmt::WhileStmt(cond, body) => {
                self.resolve_expr(cond)?;
                self.resolves(body)?;
            }
            Stmt::Return(_, val) => {
                if let Some(v) = val {
                    self.resolve_expr(v)?;
                }
            }
            Stmt::Function(name, args, optional_args, block) => {
                let func_name = match &name.ttype {
                    TType::Identifier(s) => s,
                    _ => panic!(),
                };

                self.define(&func_name);

                self.begin_scope();
                for param in args {
                    let name = match &param.ttype {
                        TType::Identifier(x) => x,
                        _ => panic!(),
                    };

                    self.define(&name);
                }

                for (param, expr) in optional_args {
                    let name = match &param.ttype {
                        TType::Identifier(x) => x,
                        _ => panic!(),
                    };

                    self.define(&name);
                    self.resolve_expr(expr)?;
                }

                self.resolves(block)?;
                self.end_scope();
            }
            Stmt::Break(_) => {}
            Stmt::Continue(_) => {}

            // todo
            Stmt::UseStmt(module, import_type) => {
                let lf = module.lineinfo;

                let name = match &module.ttype {
                    TType::Identifier(n) => n,
                    _ => panic!(),
                };

                let stdlib = self.interpreter.stdlib.mods.clone();
                let module = stdlib.get(name).clone();

                if let Some(module) = module {
                    let fns = &module.fns;

                    match &import_type {
                        ImportType::Star => {
                            let keys = fns.keys();
                            for name in keys {
                                self.define(&name);
                            }
                        }
                        ImportType::Mod => {
                            self.define(&name);
                        }
                        ImportType::Multiple(itms) => {
                            for fn_name in itms {
                                let name_string = match &fn_name.ttype {
                                    TType::Identifier(s) => s,
                                    _ => panic!(),
                                };

                                let maybe_func = fns.get(name_string);

                                match maybe_func {
                                    Some(_) => {
                                        self.define(&name_string);
                                    }
                                    None => {
                                        return Err(Error::new(
                                            lf,
                                            format!(
                                                "The item '{}' does not exist in the module '{}'.",
                                                name_string, name
                                            )
                                            .into(),
                                            ErrorType::ReferenceError,
                                        ))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    return Err(Error::new(
                        lf,
                        format!("Module '{}' not found.", name),
                        ErrorType::ReferenceError,
                    ));
                }
            }
        }

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Assign(var, val) => {
                self.resolve_local(var);
                self.resolve_expr(val)?;
            }
            Expr::Binary(left, _, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Grouping(expr) => self.resolve_expr(expr)?,
            Expr::Unary(_, expr) => {
                self.resolve_expr(expr)?;
            }
            Expr::Variable(var) => {
                self.resolve_local(var);
            }
            Expr::Block(stmts) => {
                self.resolves(stmts)?;
            }
            Expr::Logical(left, _, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Ternary(cond, left, right) => {
                self.resolve_expr(cond)?;
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call(call, _, args, optional_args) => {
                self.resolve_expr(call)?;

                for arg in args {
                    self.resolve_expr(arg)?;
                }

                for (_, arg) in optional_args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::IfExpr(cond, true_br, elif_brs, else_br) => {
                self.resolve_if(cond, true_br, elif_brs, else_br)?;
            }
            Expr::Literal(_) => {}
            Expr::Get(val, _, key) => {
                self.resolve_expr(val)?;
                self.resolve_expr(key)?;
            }
            Expr::Array(itms) => {
                for itm in itms {
                    self.resolve_expr(itm)?;
                }
            }
            Expr::Range(left, _, right, _) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Map(map) => {
                for (key, value) in map {
                    self.resolve_expr(key)?;
                    self.resolve_expr(value)?;
                }
            }
            Expr::Set(var, _, i, val) => {
                self.resolve_expr(var)?;
                self.resolve_expr(i)?;
                self.resolve_expr(val)?;
            }
            Expr::Prop(var, _) => {
                self.resolve_expr(var)?;
            }
        }

        Ok(())
    }

    // util
    fn begin_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    // resolve
    fn resolves(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        self.begin_scope();
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();

        Ok(())
    }
    // resolve_local resolves a variable
    fn resolve_local(&mut self, name: &Token) {
        let var = match &name.ttype {
            TType::Identifier(v) => v,
            _ => panic!(),
        };

        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains(var) {
                return self
                    .interpreter
                    .resolve(name.clone(), self.scopes.len() - 1 - i);
            }
        }
    }

    fn resolve_if(
        &mut self,
        cond: &Expr,
        true_br: &Vec<Stmt>,
        elif_brs: &Vec<(Expr, Vec<Stmt>)>,
        else_br: &Option<Vec<Stmt>>,
    ) -> Result<(), Error> {
        self.resolve_expr(cond)?;

        self.resolves(true_br)?;

        for (cond, block) in elif_brs {
            self.resolve_expr(cond)?;

            self.resolves(block)?;
        }

        if let Some(br) = else_br {
            self.resolves(br)?;
        }

        Ok(())
    }

    // define
    fn define(&mut self, name: &String) {
        if self.scopes.is_empty() {
            return;
        }

        let len = self.scopes.len();
        self.scopes[len - 1].push(name.clone());
    }
}
