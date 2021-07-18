use std::rc::Rc;

use crate::environment::Environment;
use crate::error::{Error, ErrorType, LineInfo};
use crate::functions::{Call, Func, FuncType};
use crate::nodes::expr::Expr;
use crate::nodes::stmt::Stmt;
use crate::token::{TType, Token};
use crate::types::Type;

enum Return {
    Type(Type),
    Break,
    Continue
}

type IResult = Result<Return, Error>;
// type SResult = Result<(), Error>;

pub struct Interpreter {
    nodes: Vec<Stmt>,
    pub environ: Box<Environment>,
}

impl Interpreter {
    // static methods
    pub fn new(nodes: Vec<Stmt>, environ: Box<Environment>) -> Self {
        Self { nodes, environ }
    }

    pub fn stringify(value: Type) -> String {
        match value {
            Type::Array(v) => {
                let mut out = String::from('[');

                for (idx, i) in v.iter().enumerate() {
                    out += Self::stringify(i.clone()).as_str();

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
            Type::Func(n) => n.to_string()
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.environ.define(
            &String::from("println"),
            &Type::Func(FuncType::Native(Func::new(
                Rc::new(|_: &mut Interpreter, args: Vec<Type>| {
                    println!("{}", Self::stringify(args[0].clone()));
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
            Stmt::VarDecl(name, val) => {
                let val = self.eval_expr(&val)?;
                match val {
                    Return::Type(val) => {
                        self.environ.define(&name, &val);
                        Ok(Return::Type(Type::Nil))
                    },
                    _ => Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid variable assignment".into(),
                        ErrorType::RuntimeError
                    ))
                }
                
            }
            Stmt::Block(stmts) => {
                self.eval_block(
                    Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                    stmts,
                )?;
                Ok(Return::Type(Type::Nil))
            }
            Stmt::IfStmt(cond, true_br, elif_brs, else_br) => {
                let cond_val = self.eval_expr(cond)?;

                match cond_val {
                    Return::Type(val) => {
                        if self.is_truthy(&val) {
                            return self.eval_block(
                                Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                                true_br,
                            )
                        }
                        if elif_brs.len() != 0 {
                            for (cond, elif_block) in elif_brs {
                                let cond_val = self.eval_expr(cond)?;
                                match cond_val {
                                    Return::Type(val) => if self.is_truthy(&val) {
                                        return Ok(self.eval_block(
                                            Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                                            elif_block,
                                        )?);
                                    },
                                    _ => return Err(Error::new(
                                        LineInfo::new(0, 0),
                                        "Invalid condition in if statement".into(),
                                        ErrorType::RuntimeError
                                    ))
                                }
                                
                            }
                        }
                        if let Some(else_block) = else_br {
                            return self.eval_block(
                                Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                                else_block,
                            );
                        }
        
                        Ok(Return::Type(Type::Nil))
                    }
                    _ => Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid condition in if statement".into(),
                        ErrorType::RuntimeError
                    ))
                }
            }
            Stmt::WhileStmt(cond, block) => {
                loop {
                    let cond = self.eval_expr(cond)?;
                    match cond {
                        Return::Type(cond) => if !self.is_truthy(&cond) {
                            break;
                        }
                        _ => ()
                    };

                    let ret = self.eval_block(
                        Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                        block,
                    )?;
                    
                    match ret {
                        Return::Break => break,
                        _ => ()
                    };
                }

                Ok(Return::Type(Type::Nil))
            },
            Stmt::Break => Ok(Return::Break),
            Stmt::Continue => Ok(Return::Continue)
        }
    }

    fn eval_expr(&mut self, node: &Expr) -> IResult {
        match node {
            Expr::Binary(left, tok, right) => {
                let l = self.eval_expr(&left.as_ref())?;
                let r = self.eval_expr(&right.as_ref())?;

                let (lval, rval) = match (l, r) {
                    (Return::Type(l), Return::Type(r)) => (l, r),
                    _ => return Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid value".into(),
                        ErrorType::RuntimeError
                    ))
                };

                Ok(Return::Type(match tok.ttype {
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
                }))
            }
            Expr::Grouping(expr) => Ok(self.eval_expr(&expr.as_ref())?),
            Expr::Literal(val) => Ok(Return::Type(val.clone())),
            Expr::Unary(tok, right) => {
                let r = self.eval_expr(&right.as_ref())?;

                let rval = match r {
                    Return::Type(r) => r,
                    _ => return Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid value".into(),
                        ErrorType::RuntimeError
                    ))
                };

                match tok.ttype {
                    TType::Not => Ok(Return::Type(
                        Type::Bool(self.is_truthy(&rval))
                    )),
                    _ => panic!(),
                }
            }
            Expr::Variable(v) => Ok(Return::Type(self.environ.get(v)?)),
            Expr::Assign(k, v) => {
                let val = self.eval_expr(&v)?;
                match val {
                    Return::Type(v) => {
                        self.environ.assign(k, &v)?;
                        Ok(Return::Type(v))
                    },
                    _ => Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid value".into(),
                        ErrorType::RuntimeError
                    ))
                }
            }
            Expr::Block(stmts) => Ok(self.eval_block(
                Box::new(Environment::new_enclosing(Box::clone(&self.environ))),
                stmts,
            )?),
            Expr::Logical(left, tok, right) => {
                let lval = self.eval_expr(left)?;
                
                match lval {
                    Return::Type(v) => {
                        if tok.ttype == TType::Or {
                            if self.is_truthy(&v) {
                                return Ok(Return::Type(v));
                            }
                        } else {
                            if !self.is_truthy(&v) {
                                return Ok(Return::Type(v));
                            }
                        }

                        self.eval_expr(right)
                    },
                    _ => Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid condition in if statement".into(),
                        ErrorType::RuntimeError
                    ))
                }
            }
            Expr::Ternary(condition, true_br, else_br) => {
                let cond = self.eval_expr(condition)?;

                match cond {
                    Return::Type(cond) => {
                        if self.is_truthy(&cond) {
                            return Ok(self.eval_expr(true_br)?);
                        }
                        Ok(self.eval_expr(else_br)?)
                    },
                    _ => Err(Error::new(
                        LineInfo::new(0, 0),
                        "Invalid condition in if statement".into(),
                        ErrorType::RuntimeError
                    ))
                }
            }
            Expr::Call(func, tok, args) => {
                let callee = self.eval_expr(func)?;

                let mut params: Vec<Type> = Vec::new();
                for arg in args {
                    match self.eval_expr(arg)? {
                        Return::Type(val) => params.push(val),
                        _ => return Err(Error::new(
                            LineInfo::new(0, 0),
                            "Invalid condition in if statement".into(),
                            ErrorType::RuntimeError
                        ))
                    }
                }

                if let Return::Type(Type::Func(func)) = callee {
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

                    func.call(self, params)?;
                } else {
                    return Err(Error::new(
                        tok.lineinfo,
                        "Only functions can be called.".into(),
                        ErrorType::TypeError,
                    ));
                }

                Ok(Return::Type(Type::Nil))
            }
        }
    }

    fn eval_block(
        &mut self,
        env: Box<Environment>,
        block: &Vec<Stmt>,
    ) -> Result<Return, Error> {
        self.environ = env.clone();
        let mut val = Return::Type(Type::Nil);

        for stmt in block {
            match stmt {
                Stmt::Break => {
                    val = Return::Break;
                    break;
                },
                Stmt::Continue => {
                    val = Return::Continue;
                    break;
                },
                _ => {
                    let ret = self.eval_stmt(stmt)?;
                    match ret {
                        Return::Break => {
                            val = Return::Break;
                            break;
                        },
                        Return::Continue => {
                            val = Return::Continue;
                            break;
                        },
                        _ => val = ret
                    }
                }
            }
        }

        self.environ = self.environ.parent.clone().unwrap();

        Ok(val)
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
}
