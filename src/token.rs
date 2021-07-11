use super::error::*;

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
    Bool,
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

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ttype: TType,
    pub lineinfo: LineInfo
}

impl Token {

}