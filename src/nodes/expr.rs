use std::rc::Rc;

use crate::token::{Token, Value};
use crate::types::Type;

#[derive(Clone)]
pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Type),
    Unary(Token, Rc<Expr>),
    Variable(Value)
}
