use crate::{nodes::expr::Expr, token::Value};

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(Value, Expr)
}