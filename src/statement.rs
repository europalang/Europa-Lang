use crate::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr)
}