use crate::{
    interpreter::Interpreter,
    nodes::{expr::Expr, stmt::Stmt},
    token::{TType, Token},
    types::Type,
};

pub struct Resolver {
    scopes: Vec<Vec<String>>,
    interpreter: Interpreter,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![vec![]],
        }
    }

    pub fn init(&mut self) -> Interpreter {
        let nodes = self.interpreter.nodes.clone();
        
        self.resolves(&nodes);

        self.interpreter.clone()
    }

    // resolve
    fn resolve_stmt(&mut self, node: &Stmt) {
        match node {
            Stmt::ExprStmt(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::VarDecl(vars) => {
                for (name, val) in vars {
                    self.resolve_expr(val);
                    self.define(name);
                }
            }
            Stmt::Block(stmts) => {
                self.begin_scope();
                self.resolves(stmts);
                self.end_scope();
            }
            Stmt::IfStmt(cond, true_br, elif_brs, else_br) => {
                self.resolve_if(cond, true_br, elif_brs, else_br);
            }
            Stmt::WhileStmt(cond, body) => {
                self.resolve_expr(cond);
                self.begin_scope();
                self.resolves(body);
                self.end_scope();
            }
            Stmt::Return(_, val) => {
                if let Some(v) = val {
                    self.resolve_expr(v);
                }
            }
            Stmt::Function(name, args, block) => {
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

                self.resolves(block);
                self.end_scope();
            }
            Stmt::ForStmt(name, val, block) => {
                let str = match &name.ttype {
                    TType::Identifier(x) => x,
                    _ => panic!()
                };

                self.define(str);
                self.resolve_expr(val);
                self.resolves(block);
            },
            Stmt::Break(_) => {}
            Stmt::Continue(_) => {}
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Assign(var, val) => {
                self.resolve_local(var);
                self.resolve_expr(val);
            }
            Expr::Binary(left, _, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Unary(_, expr) => {
                self.resolve_expr(expr);
            }
            Expr::Variable(var) => {
                self.resolve_local(var);
            }
            Expr::Block(stmts) => {
                self.resolves(stmts);
            }
            Expr::Logical(left, _, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Ternary(cond, left, right) => {
                self.resolve_expr(cond);
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call(call, _, args) => {
                self.resolve_expr(call);

                for arg in args {
                    self.resolve_expr(arg);
                }
            }
            Expr::IfExpr(cond, true_br, elif_brs, else_br) => {
                self.resolve_if(cond, true_br, elif_brs, else_br);
            }
            Expr::Literal(val) => {
                match val {
                    Type::Array(itms) => {
                        for itm in itms {
                            self.resolve_expr(itm);
                        }
                    }
                    _ => {}
                };
            }
        }
    }

    // util
    fn begin_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    // resolve
    fn resolves(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }
    // resolve_local resolves a variable
    fn resolve_local(&mut self, name: &Token) {
        let var = match &name.ttype {
            TType::Identifier(v) => v,
            _ => panic!(),
        };

        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains(var) {
                // println!("depth: {} {:?} {:?}", self.scopes.len() - 1 - i, name.ttype, self.scopes);
                self.interpreter
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
    ) {
        self.resolve_expr(cond);
        self.resolves(true_br);

        for (cond, block) in elif_brs {
            self.resolve_expr(cond);
            self.resolves(block);
        }

        if let Some(br) = else_br {
            self.resolves(br);
        }
    }

    // define
    fn define(&mut self, name: &String) {
        if self.scopes.is_empty() { return; }

        let len = self.scopes.len();
        self.scopes[len - 1].push(name.clone());
    }
}
