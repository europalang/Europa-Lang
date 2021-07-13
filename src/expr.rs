use std::rc::Rc;

use super::token::Token;
use super::types::Type;

#[derive(Debug)]
pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Type),
    Unary(Token, Rc<Expr>)
}
