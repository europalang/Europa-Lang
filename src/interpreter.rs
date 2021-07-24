use std::{collections::HashMap, rc::Rc};

use crate::{
    environment::Environment,
    error::{Error, ErrorType, LineInfo},
    functions::{Call, Func, FuncCallable, FuncType},
    nodes::{expr::Expr, stmt::Stmt},
    token::{TType, Token},
    types::Type,
};

type IResult = Result<Type, Error>;
// type SResult = Result<(), Error>;

#[derive(Clone)]
pub struct Interpreter {
    pub nodes: Vec<Stmt>,
    pub environ: Box<Environment>,
    locals: HashMap<LineInfo, usize>,
}

impl Interpreter {
    // static methods
    pub fn new(nodes: Vec<Stmt>, environ: Box<Environment>) -> Self {
        Self {
            nodes,
            environ,
            locals: HashMap::new(),
        }
    }

    pub fn stringify(&mut self, value: Type) -> Result<String, Error> {
        Ok(match value {
            Type::Array(v) => {
                let mut out = String::from('[');

                for (idx, i) in v.iter().enumerate() {
                    let val = self.eval_expr(i)?;
                    out += self.stringify(val)?.as_str();

                    if idx < v.len() - 1 {
                        out += ", ";
                    }
                }

                out + "]"
            }
            Type::Nil => "nil".into(),
            Type::Float(n) => n.to_string(),
            Type::String(n) => n,
            Type::Bool(n) => n.to_string(),
            Type::Func(n) => n.to_string(),
        })
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.environ.define(
            &String::from("println"),
            &Type::Func(FuncType::Native(Func::new(
                Rc::new(|i: &mut Interpreter, args: Vec<Type>| {
                    println!("{}", i.stringify(args[0].clone())?);
                    Ok(Type::Nil)
                }),
                1,
            ))),
        );

        for stmt in self.nodes.clone() {
            self.eval_stmt(&stmt.clone())?;
        }

        Ok(())
    }

    // eval
    fn eval_stmt(&mut self, node: &Stmt) -> IResult {
        match node {
            Stmt::ExprStmt(s) => self.eval_expr(s),
            Stmt::VarDecl(decls) => {
                for (name, val) in decls {
                    let val = self.eval_expr(&val)?;
                    self.environ.define(&name, &val);
                }
                Ok(Type::Nil)
            }
            Stmt::Block(stmts) => {
                self.eval_block(
                    Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                    stmts,
                    false,
                )?;
                Ok(Type::Nil)
            }
            Stmt::IfStmt(cond, true_br, elif_brs, else_br) => {
                Ok(self.eval_if(cond, true_br, elif_brs, else_br)?)
            }
            Stmt::WhileStmt(cond, block) => {
                loop {
                    let cond = self.eval_expr(cond)?;
                    if !self.is_truthy(&cond) {
                        break;
                    }

                    let out = self.eval_block(
                        Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                        block,
                        false,
                    );

                    if let Err(e) = out {
                        if e.error_type == ErrorType::Break {
                            break;
                        }

                        if e.error_type == ErrorType::Continue {
                            continue;
                        }
                    }
                }

                Ok(Type::Nil)
            }
            Stmt::Break(t) => Err(Error::new(
                t.lineinfo,
                "Break statements can only be inside loops.".into(),
                ErrorType::Break,
            )),
            Stmt::Continue(t) => Err(Error::new(
                t.lineinfo,
                "Continue statements can only be inside loops.".into(),
                ErrorType::Continue,
            )),
            Stmt::Return(t, val) => {
                let expr;
                if let Some(v) = val {
                    expr = self.eval_expr(v)?;
                } else {
                    expr = Type::Nil;
                }

                Err(Error::new(
                    t.lineinfo,
                    "Return statements can only be inside functions.".into(),
                    ErrorType::Return(expr),
                ))
            }
            Stmt::Function(name, args, block) => {
                let var_name = match &name.ttype {
                    TType::Identifier(x) => x,
                    _ => panic!(),
                };

                self.environ.define(
                    &var_name,
                    &Type::Func(FuncType::User(FuncCallable::new(
                        name.clone(),
                        args.clone(),
                        block.clone(),
                    ))),
                );

                Ok(Type::Nil)
            }
            Stmt::ForStmt(name, arr, block) => {
                let val = self.eval_expr(arr)?;
                match val {
                    Type::Array(exprs) => {
                        for expr in exprs {
                            let v = self.eval_expr(&expr)?;
                            let name_str = match &name.ttype {
                                TType::Identifier(v) => v,
                                _ => panic!(),
                            };

                            self.environ.define(name_str, &v);
                            self.eval_block(
                                Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                                block,
                                false,
                            )?;
                        }

                        Ok(Type::Nil)
                    }
                    _ => {
                        return Err(Error::new(
                            name.lineinfo,
                            "Only arrays can be iterated over.".into(),
                            ErrorType::TypeError,
                        ))
                    }
                }
            }
        }
    }

    fn eval_expr(&mut self, node: &Expr) -> IResult {
        match node {
            Expr::Binary(left, tok, right) => {
                let lval = self.eval_expr(&left.as_ref())?;
                let rval = self.eval_expr(&right.as_ref())?;

                Ok(match tok.ttype {
                    TType::Plus => self.out(&lval.add(&rval), &tok)?,
                    TType::Minus => self.out(&lval.sub(&rval), &tok)?,
                    TType::Times => self.out(&lval.mult(&rval), &tok)?,
                    TType::Divide => self.out(&lval.div(&rval), &tok)?,
                    TType::Mod => self.out(&lval.modulo(&rval), &tok)?,

                    TType::EqEq => Type::Bool(lval == rval),
                    TType::NotEq => Type::Bool(lval != rval),

                    TType::Less => Type::Bool(lval < rval),
                    TType::Greater => Type::Bool(lval > rval),
                    TType::LessEq => Type::Bool(lval <= rval),
                    TType::GreaterEq => Type::Bool(lval >= rval),
                    _ => panic!(),
                })
            }
            Expr::Grouping(expr) => Ok(self.eval_expr(&expr.as_ref())?),
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Unary(tok, right) => {
                let rval = self.eval_expr(&right.as_ref())?;

                match tok.ttype {
                    TType::Not => Ok(Type::Bool(self.is_truthy(&rval))),
                    _ => panic!(),
                }
            }
            Expr::Variable(v) => {
                let some_key = self.locals.get(&v.lineinfo);

                if let Some(key) = some_key {
                    Ok(self.environ.get_at(*key, v))
                } else {
                    self.environ.get(v)
                }
            }
            Expr::Assign(k, v) => {
                let val = self.eval_expr(&v)?;
                let some_key = self.locals.get(&k.lineinfo);

                if let Some(key) = some_key {
                    self.environ.assign_at(*key, k, &val)?;
                } else {
                    self.environ.assign(k, &val)?;
                }
                Ok(val)
            }
            Expr::Block(stmts) => Ok(self
                .eval_block(
                    Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                    stmts,
                    true,
                )?
                .unwrap()),
            Expr::Logical(left, tok, right) => {
                let lval = self.eval_expr(left)?;

                if tok.ttype == TType::Or {
                    if self.is_truthy(&lval) {
                        return Ok(lval);
                    }
                } else {
                    if !self.is_truthy(&lval) {
                        return Ok(lval);
                    }
                }

                self.eval_expr(right)
            }
            Expr::Ternary(condition, true_br, else_br) => {
                let cond = self.eval_expr(condition)?;

                if self.is_truthy(&cond) {
                    return Ok(self.eval_expr(true_br)?);
                }

                Ok(self.eval_expr(else_br)?)
            }
            Expr::Call(func, tok, args) => {
                let callee = self.eval_expr(func)?;

                let mut params: Vec<Type> = Vec::new();
                for arg in args {
                    params.push(self.eval_expr(arg)?);
                }

                if let Type::Func(func) = callee {
                    if params.len() != func.arity() {
                        return Err(Error::new(
                            tok.lineinfo,
                            format!(
                                "Expected {} arguments, but got {}.",
                                func.arity(),
                                params.len()
                            )
                            .into(),
                            ErrorType::TypeError,
                        ));
                    }

                    return func.call(self, params);
                } else {
                    return Err(Error::new(
                        tok.lineinfo,
                        "Only functions can be called.".into(),
                        ErrorType::TypeError,
                    ));
                }
            }
            Expr::IfExpr(cond, true_br, elif_brs, else_br) => {
                Ok(self.eval_if(cond, true_br, elif_brs, else_br)?)
            }
        }
    }

    pub fn eval_block(
        &mut self,
        env: Box<Environment>,
        block: &Vec<Stmt>,
        ret_val: bool,
    ) -> Result<Option<Type>, Error> {
        self.environ = env.clone();
        let mut val = Type::Nil;

        for stmt in block {
            if ret_val {
                val = self.eval_stmt(stmt)?;
            } else {
                self.eval_stmt(stmt)?;
            }
        }

        self.environ = self.environ.parent.clone().unwrap();

        if ret_val {
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    fn eval_if(
        &mut self,
        cond: &Expr,
        true_br: &Vec<Stmt>,
        elif_brs: &Vec<(Expr, Vec<Stmt>)>,
        else_br: &Option<Vec<Stmt>>,
    ) -> IResult {
        let cond_val = self.eval_expr(cond)?;

        if self.is_truthy(&cond_val) {
            return Ok(self
                .eval_block(
                    Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                    true_br,
                    true,
                )?
                .unwrap());
        }
        if elif_brs.len() != 0 {
            for (cond, elif_block) in elif_brs {
                let cond_val = self.eval_expr(cond)?;
                if self.is_truthy(&cond_val) {
                    return Ok(self
                        .eval_block(
                            Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                            elif_block,
                            true,
                        )?
                        .unwrap());
                }
            }
        }
        if let Some(else_block) = else_br {
            return Ok(self
                .eval_block(
                    Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                    else_block,
                    true,
                )?
                .unwrap());
        }

        Ok(Type::Nil)
    }

    // util
    fn out(&self, val: &Result<Type, (String, ErrorType)>, tok: &Token) -> Result<Type, Error> {
        match val {
            Ok(r) => Ok(r.clone()),
            Err(t) => return Err(Error::new(tok.lineinfo, t.clone().0, t.clone().1)),
        }
    }

    fn is_truthy(&self, v: &Type) -> bool {
        match v {
            Type::Nil => false,
            Type::Bool(v) => *v,
            _ => true,
        }
    }

    pub fn resolve(&mut self, tok: Token, depth: usize) {
        self.locals.insert(tok.lineinfo, depth);
    }
}
