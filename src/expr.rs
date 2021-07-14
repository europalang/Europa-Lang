use std::rc::Rc;

use crate::token::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Type),
    Unary(Token, Rc<Expr>)
}
