use std::{fmt::Display, collections::HashMap};

use crate::error::LineInfo;

#[derive(Debug, Clone, PartialEq)]
pub enum TType {
    // delims
    LeftBBrace,  // {{
    RightBBrace, // }}
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBrack,
    RightBrack,
    Comma,
    Dot,
    DotDot,
    DotEq,
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

    Colon,
    Question,

    // literals
    Identifier(String),
    String(String),
    TemplateString(String, HashMap<usize, Vec<Token>>),
    Number(f32),
    True,
    False,
    Nil,

    // keywords
    Fn,
    Return,
    Var,
    Use,
    Do,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TType,
    pub lineinfo: LineInfo,
}

impl Display for TType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
