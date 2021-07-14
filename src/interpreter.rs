use crate::error::{Error, ErrorType};
use crate::expr::Expr;
use crate::statement::Stmt;
use crate::token::{TType, Token};
use crate::types::Type;

type IResult = Result<Type, Error>;
type SResult = Result<(), Error>;

pub struct Interpreter {
    nodes: Vec<Stmt>,
}

impl Interpreter {
    // static methods
    pub fn new(nodes: Vec<Stmt>) -> Self {
        Self { nodes }
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
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        for stmt in &self.nodes {
            self.eval_stmt(stmt.clone())?;
        }

        Ok(())
    }

    fn eval_stmt(&self, node: Stmt) -> SResult {
        match node {
            Stmt::ExprStmt(s) => self.eval_expr(s)?,
        };

        Ok(())
    }

    fn eval_expr(&self, node: Expr) -> IResult {
        match node {
            Expr::Binary(left, tok, right) => {
                let lval = self.eval_expr(left.as_ref().clone())?;
                let rval = self.eval_expr(right.as_ref().clone())?;

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
                    _ => Type::Nil,
                })
            }
            Expr::Grouping(expr) => Ok(self.eval_expr(expr.as_ref().clone())?),
            Expr::Literal(val) => Ok(val),
            Expr::Unary(tok, right) => {
                let rval = self.eval_expr(right.as_ref().clone())?;

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

    fn out(&self, val: &Result<Type, (String, ErrorType)>, tok: &Token) -> Result<Type, Error> {
        match val {
            Ok(r) => Ok(r.clone()),
            Err(t) => return Err(Error::new(tok.lineinfo, t.clone().0, t.clone().1)),
        }
    }
}
