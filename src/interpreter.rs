use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{environment::Environment, error::{Error, ErrorType, LineInfo}, functions::{Call, FuncCallable, FuncType}, nodes::{
        expr::Expr,
        stmt::{ImportType, Stmt},
    }, stdlib::Stdlib, token::{TType, Token}, types::{array::Array, module::ModImport}, types::{map::Map, Type}};

type IResult = Result<Type, Error>;
// type SResult = Result<(), Error>;

#[derive(Clone)]
pub struct Interpreter {
    pub nodes: Vec<Stmt>,
    pub environ: Environment,
    pub locals: HashMap<LineInfo, usize>,
    stdlib: Stdlib,
}

impl Interpreter {
    // static methods
    pub fn new(nodes: Vec<Stmt>, environ: Environment) -> Self {
        Self {
            nodes,
            environ,
            locals: HashMap::new(),
            stdlib: Stdlib::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
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
                self.eval_block(stmts, false)?;
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

                    let out = self.eval_block(block, false);

                    if let Err(e) = out {
                        if e.error_type == ErrorType::Break {
                            break;
                        }

                        if e.error_type == ErrorType::Continue {
                            continue;
                        }

                        return Err(e);
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
                        // for i in itm {
                        for expr in &exprs.borrow().arr {
                            let v = expr;
                            let name_str = match &name.ttype {
                                TType::Identifier(v) => v,
                                _ => panic!(),
                            };

                            self.environ.define(name_str, v);
                            match self.eval_block(block, false) {
                                Err(e) => match e.error_type {
                                    ErrorType::Break => break,
                                    ErrorType::Continue => continue,
                                    _ => return Err(e),
                                },
                                _ => {}
                            };
                        }

                        self.environ.pop_scope();
                        self.environ.pop_scope();

                        Ok(Type::Nil)
                    }
                    Type::String(str) => {
                        for i in str.chars() {
                            let v = Type::String(String::from(i));
                            let name_str = match &name.ttype {
                                TType::Identifier(v) => v,
                                _ => panic!(),
                            };

                            self.environ.define(name_str, &v);
                            match self.eval_block(block, false) {
                                Err(e) => match e.error_type {
                                    ErrorType::Break => break,
                                    ErrorType::Continue => continue,
                                    _ => return Err(e),
                                },
                                _ => {}
                            };
                        }

                        self.environ.pop_scope();
                        self.environ.pop_scope();

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
            Stmt::UseStmt(module, import_type) => {
                let lf = module.lineinfo;

                if let TType::Identifier(name) = &module.ttype {
                    if !self.stdlib.mods.contains_key(name) {
                        return Err(Error::new(
                            module.lineinfo,
                            format!("Module {} not found.", name),
                            ErrorType::ReferenceError,
                        ));
                    }

                    let module = &self.stdlib.mods[name].fns;

                    match &import_type {
                        ImportType::Star => {
                            for (name, func) in module {
                                self.environ.define(name, &Type::Func(func.clone()));
                            }
                        }
                        ImportType::Multiple(fns) => {
                            for fn_name in fns {
                                let name_string = match &fn_name.ttype {
                                    TType::Identifier(s) => s,
                                    _ => panic!(),
                                };

                                let maybe_func = module.get(name_string);

                                match maybe_func {
                                    Some(func) => {
                                        self.environ.define(name_string, &Type::Func(func.clone()));
                                    }
                                    None => {
                                        return Err(Error::new(
                                            lf,
                                            format!(
                                                "The item '{}' does not exist in the module '{}'",
                                                name_string, name
                                            )
                                            .into(),
                                            ErrorType::ReferenceError,
                                        ))
                                    }
                                }
                            }
                        }
                        ImportType::Mod => {
                            self.environ.define(name, &Type::Module(ModImport::new(name.clone(), module.clone())))
                        }
                    }

                    Ok(Type::Nil)
                } else {
                    panic!();
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
                    TType::Pow => self.out(&lval.pow(&rval), &tok)?,

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
                    TType::Minus => match rval {
                        Type::Float(v) => Ok(Type::Float(-v)),
                        _ => Err(Error::new(
                            tok.lineinfo,
                            "Only numbers can be negated.".into(),
                            ErrorType::TypeError,
                        )),
                    },
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

                self.assign(k, &val)
            }
            Expr::Block(stmts) => Ok(self.eval_block(stmts, true)?.unwrap()),
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
                    let ar = func.arity();

                    if params.len() != ar {
                        return Err(Error::new(
                            tok.lineinfo,
                            format!(
                                "Expected {} argument{}, but got {}.",
                                ar,
                                if ar == 1 { "" } else { "s" },
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
            Expr::Get(val, tok, key) => {
                let v = self.eval_expr(val)?;
                let k = self.eval_expr(key)?;

                self.out(&v.index(k), &tok)
            }
            Expr::Array(itms) => {
                let mut out: Vec<Type> = Vec::new();

                for itm in itms {
                    out.push(self.eval_expr(itm)?);
                }

                Ok(Type::Array(Rc::new(RefCell::new(Array::new(out)))))
            }
            Expr::Range(left, tok, right, inclusive) => {
                let left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;

                if let (Type::Float(l), Type::Float(r)) = (left, right) {
                    let mut out: Vec<Type> = Vec::new();
                    let (l, r) = (l.floor() as i32, r.floor() as i32);

                    if l > r {
                        if *inclusive {
                            for i in (r..=l).rev() {
                                out.push(Type::Float(i as f32));
                            }
                        } else {
                            for i in (r + 1..=l).rev() {
                                out.push(Type::Float(i as f32));
                            }
                        }
                    } else {
                        if *inclusive {
                            for i in l..=r {
                                out.push(Type::Float(i as f32));
                            }
                        } else {
                            for i in l..r {
                                out.push(Type::Float(i as f32));
                            }
                        }
                    }

                    return Ok(Type::Array(Rc::new(RefCell::new(Array::new(out)))));
                } else {
                    Err(Error::new(
                        tok.lineinfo,
                        "Ranges can only contain numbers.".into(),
                        ErrorType::TypeError,
                    ))
                }
            }
            Expr::Map(v) => {
                let mut out = HashMap::new();

                for (key, value) in v {
                    let key = self.eval_expr(key)?.to_string();
                    let value = self.eval_expr(value)?;

                    out.insert(key, value);
                }

                Ok(Type::Map(Rc::new(RefCell::new(Map::new(out)))))
            }
            Expr::Set(var, brack, i, val) => {
                let collection = self.eval_expr(var)?;
                let i = self.eval_expr(i)?;
                let val = self.eval_expr(val)?;

                self.out(&collection.assign(i, val), brack)
            }
            Expr::Prop(var, prop) => {
                let module = self.eval_expr(var)?;
                match module {
                    Type::Module(module) => {
                        // correct
                        let prop_string = match &prop.ttype {
                            TType::Identifier(v) => v,
                            _ => panic!(),
                        };

                        let maybe_fn = module.fns.get(prop_string);

                        if let Some(out) = maybe_fn {
                            Ok(Type::Func(out.clone()))
                        } else {
                            Err(Error::new(
                                prop.lineinfo,
                                format!("The item '{}' does not exist in the module '{}'.", prop_string, module.name).into(),
                                ErrorType::TypeError,
                            ))
                        }
                    }
                    _ => Err(Error::new(
                        prop.lineinfo,
                        "Only modules have properties.".into(),
                        ErrorType::TypeError,
                    )),
                }
            }
        }
    }

    pub fn eval_block(&mut self, block: &Vec<Stmt>, ret_val: bool) -> Result<Option<Type>, Error> {
        self.environ.push_scope();

        let mut val = Type::Nil;
        for stmt in block {
            if ret_val {
                val = self.eval_stmt(stmt)?;
            } else {
                self.eval_stmt(stmt)?;
            }
        }

        self.environ.pop_scope();

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
            return Ok(self.eval_block(true_br, true)?.unwrap());
        }
        if elif_brs.len() != 0 {
            for (cond, elif_block) in elif_brs {
                let cond_val = self.eval_expr(cond)?;
                if self.is_truthy(&cond_val) {
                    return Ok(self.eval_block(elif_block, true)?.unwrap());
                }
            }
        }
        if let Some(else_block) = else_br {
            return Ok(self.eval_block(else_block, true)?.unwrap());
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

    fn assign(&mut self, var: &Token, val: &Type) -> Result<Type, Error> {
        let some_key = self.locals.get(&var.lineinfo);

        if let Some(key) = some_key {
            self.environ.assign_at(*key, var, val)?;
        } else {
            self.environ.assign(var, val)?;
        }

        Ok(val.clone())
    }

    pub fn resolve(&mut self, tok: Token, depth: usize) {
        self.locals.insert(tok.lineinfo, depth);
    }
}
