use crate::nodes::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(String, Expr),
    Block(Vec<Stmt>)
}
