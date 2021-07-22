use crate::{nodes::expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(Vec<(String, Expr)>),
    Block(Vec<Stmt>),
    IfStmt(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
    WhileStmt(Expr, Vec<Stmt>),
    Break(Token),
    Continue(Token),
    Return(Token, Option<Expr>),
    Function(Token, Vec<Token>, Vec<Stmt>),
}
