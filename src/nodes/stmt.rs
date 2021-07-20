use crate::{nodes::expr::Expr, token::Token};

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(String, Expr),
    Block(Vec<Stmt>),
    IfStmt(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
    WhileStmt(Expr, Vec<Stmt>),
    Break(Token),
    Continue(Token),
    Return(Option<Expr>, Token),
    Function(Token, Vec<Token>, Vec<Stmt>),
}
