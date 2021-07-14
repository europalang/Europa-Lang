use std::borrow::Borrow;
use std::rc::Rc;

use super::error::Error;
use super::expr::*;
use super::token::*;
use super::types::Type;

type IResult = Result<Type, Error>;

pub struct Interpreter {
    nodes: Expr,
}

impl Interpreter {
    // static methods
    pub fn new(nodes: Expr) -> Self {
        Self { nodes }
    }

    pub fn stringify(value: Type) -> String {
        match value {
            Type::Array(v) => {
                let mut out = String::from('[');

                for (idx, i) in v.iter().enumerate() {
                    out += Self::stringify(*i).as_str();

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
        }
    }

    pub fn init(&self) -> IResult {
        self.eval(self.nodes)
    }

    fn eval(&self, node: Expr) -> IResult {
        match node {
            Expr::Binary(left, tok, right) => {
                let lval = self.eval(*left)?;
                let rval = self.eval(*right)?;

                Ok(match tok.ttype {
                    TType::Plus => self.out(&lval.add(&rval), &tok)?,
                    TType::Minus => self.out(&lval.sub(&rval), &tok)?,
                    TType::Times => self.out(&lval.mult(&rval), &tok)?,
                    TType::Divide => self.out(&lval.div(&rval), &tok)?,
                    TType::Mod => self.out(&lval.modulo(&rval), &tok)?,

                    TType::Eq => Type::Bool(lval == rval),
                    TType::NotEq => Type::Bool(lval != rval),

                    TType::Less => Type::Bool(lval < rval),
                    TType::Greater => Type::Bool(lval > rval),
                    TType::LessEq => Type::Bool(lval <= rval),
                    TType::GreaterEq => Type::Bool(lval >= rval),
                    _ => Type::Nil,
                })
            }
            Expr::Grouping(ref expr) => Ok(self.eval(*expr)?),
            Expr::Literal(val) => Ok(val),
            Expr::Unary(tok, right) => {
                let rval = self.eval(*right)?;

                match tok.ttype {
                    TType::Not => Ok(match rval {
                        Type::Nil => Type::Bool(false),
                        Type::Bool(v) => Type::Bool(v),
                        _ => Type::Bool(true),
                    }),
                    _ => Ok(Type::Nil),
                }
            }
        }
    }

    fn out(&self, val: &Result<Type, String>, tok: &Token) -> Result<Type, Error> {
        match val {
            Ok(r) => Ok(*r),
            Err(t) => return Err(Error::new(tok.lineinfo, *t)),
        }
    }
}
