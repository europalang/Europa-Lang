use crate::nodes::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(String, Expr),
    Block(Vec<Stmt>),
    IfStmt(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
    WhileStmt(Expr, Vec<Stmt>)
}
