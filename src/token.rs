#[derive(Debug, Clone, Copy)]
pub enum Token {
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
    And
}

impl Token {

}