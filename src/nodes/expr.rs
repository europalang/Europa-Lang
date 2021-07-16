use std::rc::Rc;

use crate::nodes::stmt::Stmt;
use crate::token::Token;
use crate::types::Type;

#[derive(Clone)]
pub enum Expr {
    Assign(Token, Rc<Expr>),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Type),
    Unary(Token, Rc<Expr>),
    Variable(Token),
    Block(Vec<Stmt>),
}
