use super::{error::*,types::Type};

#[derive(Debug, Clone, Copy)]
pub enum TType {
    // delims
    LeftS, // {{
    RightS, // }}
    LeftBrace,
    RightBrace,
    LeftParen,
    RigthParen,
    LeftBrack,
    RightBrack,
    Comma,
    Dot,
    Semi,

    // comparison
    Not,
    EqEq,
    NotEq,

    // assignment
    Eq,
    PlusEq,
    MinusEq,
    TimesEq,
    DivideEq,
    PowEq,
    
    // operators
    Plus,
    Minus,
    Times,
    Divide,
    Pow,

    // literals
    Identifier,
    String,
    Number,
    True,
    False,
    Nil,

    // keywords
    Fn,
    Return,
    Var,
    Use,
    While,
    For,
    In,
    Break,
    Continue,
    Or,
    And,

    EOF
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Ident(String),
    Float(f32),
    Nil
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TType,
    pub lineinfo: LineInfo,
    pub value: Value
}

impl Token {

}