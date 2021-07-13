use super::error::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TType {
    // delims
    LeftS,  // {{
    RightS, // }}
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBrack,
    RightBrack,
    Comma,
    Dot,
    Semi,

    // comparison
    Not,
    EqEq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    // assignment
    Eq,
    PlusEq,
    MinusEq,
    TimesEq,
    DivideEq,
    PowEq,
    ModEq,

    // operators
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
    Mod,

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
    If,
    Else,
    Elif,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Ident(String),
    Float(f32),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TType,
    pub lineinfo: LineInfo,
    pub value: Value,
}

impl Token {}
