use crate::error::{Error, ErrorNote, ErrorType, LineInfo};
use crate::token::{TType, Token};

use maplit::hashmap;
use std::char::from_u32 as char_from_u32;
use std::collections::HashMap;

pub struct Lexer {
    code: String,
    chars: Vec<char>,
    i: usize, // index
    info: LineInfo,
    tokens: Vec<Token>,
    keywords: HashMap<String, TType>,
}

impl Lexer {
    pub fn new(code: &String) -> Self {
        Self {
            code: code.to_string(),
            chars: code.chars().collect(),
            i: 0,
            info: LineInfo::new(1, 0),
            tokens: vec![],
            keywords: hashmap! {
                "true".into() => TType::True,
                "false".into() => TType::False,
                "nil".into() => TType::Nil,
                "fn".into() => TType::Fn,
                "return".into() => TType::Return,
                "var".into() => TType::Var,
                "use".into() => TType::Use,
                "do".into() => TType::Do,
                "while".into() => TType::While,
                "for".into() => TType::For,
                "in".into() => TType::In,
                "break".into() => TType::Break,
                "continue".into() => TType::Continue,
                "or".into() => TType::Or,
                "and".into() => TType::And,
                "if".into() => TType::If,
                "else".into() => TType::Else,
                "elif".into() => TType::Elif,
            },
        }
    }

    pub fn set_lineinfo(&mut self, info: LineInfo) {
        self.info = info;
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            self.lex_char()?;
        }

        self.append_token(TType::EOF);

        Ok(self.tokens.clone())
    }

    fn lex_char(&mut self) -> Result<(), Error> {
        let char = self.peek();
        self.next();

        match char {
            '{' => {
                if self.get('{') {
                    self.append_token(TType::LeftBBrace)
                } else {
                    self.append_token(TType::LeftBrace)
                }
            }
            '}' => {
                if self.get('}') {
                    self.append_token(TType::RightBBrace)
                } else {
                    self.append_token(TType::RightBrace)
                }
            }
            '(' => self.append_token(TType::LeftParen),
            ')' => self.append_token(TType::RightParen),
            '[' => self.append_token(TType::LeftBrack),
            ']' => self.append_token(TType::RightBrack),

            '!' => {
                if self.get('=') {
                    self.append_token(TType::NotEq)
                } else {
                    self.append_token(TType::Not)
                }
            }
            '=' => {
                if self.get('=') {
                    self.append_token(TType::EqEq)
                } else {
                    self.append_token(TType::Eq)
                }
            }

            '>' => {
                if self.get('=') {
                    self.append_token(TType::GreaterEq)
                } else {
                    self.append_token(TType::Greater)
                }
            }
            '<' => {
                if self.get('=') {
                    self.append_token(TType::LessEq)
                } else {
                    self.append_token(TType::Less)
                }
            }

            '"' | '\'' => {
                let lf = self.info;
                let str_type = char; // " or '
                let mut str = String::new();

                while self.is_valid() && self.peek() != str_type {
                    if self.peek() == '\n' {
                        self.newline();
                    }

                    if self.peek() == '\\' {
                        self.next();
                        let ch = self.peek();
                        self.next();

                        str.push(match ch {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'a' => '\x07', // bell
                            'b' => '\x08', // backspace
                            'e' => '\x1b', // ansii escape
                            'f' => '\x0c', // form feed
                            'v' => '\x0b', // vertical tab
                            '\\' => '\\',
                            '\'' => '\'',
                            '\"' => '\"',
                            '?' => '?',
                            'o' => u8::from_str_radix(self.read_n(3).as_str(), 8)
                                .map_err(|_| ())
                                .and_then(|c| char_from_u32(c as u32).ok_or(()))
                                .map_err(|_| {
                                    Error::new(
                                        self.info,
                                        "Invalid string escape.".into(),
                                        ErrorType::SyntaxError,
                                    )
                                })?,
                            'x' => u8::from_str_radix(self.read_n(2).as_str(), 16)
                                .map_err(|_| ())
                                .and_then(|c| char_from_u32(c as u32).ok_or(()))
                                .map_err(|_| {
                                    Error::new(
                                        self.info,
                                        "Invalid string escape.".into(),
                                        ErrorType::SyntaxError,
                                    )
                                })?,
                            'u' => u16::from_str_radix(self.read_n(4).as_str(), 16)
                                .map_err(|_| ())
                                .and_then(|c| char_from_u32(c as u32).ok_or(()))
                                .map_err(|_| {
                                    Error::new(
                                        self.info,
                                        "Invalid string escape.".into(),
                                        ErrorType::SyntaxError,
                                    )
                                })?,
                            'U' => u32::from_str_radix(self.read_n(8).as_str(), 16)
                                .map_err(|_| ())
                                .and_then(|c| char_from_u32(c).ok_or(()))
                                .map_err(|_| {
                                    Error::new(
                                        self.info,
                                        "Invalid string escape.".into(),
                                        ErrorType::SyntaxError,
                                    )
                                })?,
                            _ => {
                                return Err(Error::new(
                                    self.info,
                                    "Invalid string escape.".into(),
                                    ErrorType::SyntaxError,
                                ))
                            }
                        });
                    } else {
                        str.push(self.peek());
                        self.next();
                    }
                }

                if !self.is_valid() {
                    return Err(Error::new_n(
                        self.info,
                        String::from("Unterminated string."),
                        ErrorType::SyntaxError,
                        vec![ErrorNote::Expect(lf, "String starts here.".into())],
                    ));
                }

                self.next(); // "

                self.tokens.push(Token {
                    ttype: TType::String(str),
                    lineinfo: self.info,
                });
            }

            ',' => self.append_token(TType::Comma),
            '.' => {
                if self.get('.') {
                    self.append_token(TType::DotDot)
                } else if self.get('=') {
                    self.append_token(TType::DotEq)
                } else {
                    self.append_token(TType::Dot)
                }
            }
            ';' => self.append_token(TType::Semi),

            // ternary operator
            '?' => self.append_token(TType::Question),
            ':' => self.append_token(TType::Colon),

            // operators
            '+' => {
                if self.get('=') {
                    self.append_token(TType::PlusEq)
                } else {
                    self.append_token(TType::Plus)
                }
            }
            '-' => {
                if self.get('=') {
                    self.append_token(TType::MinusEq)
                } else {
                    self.append_token(TType::Minus)
                }
            }
            '*' => {
                if self.get('=') {
                    self.append_token(TType::TimesEq)
                } else if self.get('*') {
                    if self.get('=') {
                        self.append_token(TType::PowEq)
                    } else {
                        self.append_token(TType::Pow)
                    }
                } else {
                    self.append_token(TType::Times)
                }
            }
            '%' => {
                if self.get('=') {
                    self.append_token(TType::ModEq)
                } else {
                    self.append_token(TType::Mod)
                }
            }
            '/' => {
                if self.get('=') {
                    self.append_token(TType::DivideEq)
                } else if self.get('/') {
                    while self.peek() != '\n' && self.is_valid() {
                        self.next();
                    }
                } else if self.get('*') {
                    let lf = self.info;

                    while self.is_valid() && !(self.peek() == '*' && self.peek_n(1) == '/') {
                        if self.peek() == '\n' {
                            self.newline();
                        }

                        self.next();
                    }

                    if !self.is_valid() {
                        return Err(Error::new_n(
                            self.info,
                            String::from("Unterminated multiline comment."),
                            ErrorType::SyntaxError,
                            vec![ErrorNote::Expect(lf, "Expected '*/' to match this.".into())],
                        ));
                    }

                    self.next(); // *
                    self.next() // /
                } else {
                    self.append_token(TType::Divide)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.newline(),

            _ => {
                if self.is_alpha(char) {
                    let mut name = String::from(char);
                    while self.is_valid() && self.is_alphanum(self.peek()) {
                        name += &self.peek().to_string();
                        self.next();
                    }

                    if self.keywords.contains_key(&name) {
                        self.tokens.push(Token {
                            ttype: self.keywords[&name].clone(),
                            lineinfo: self.info,
                        });
                    } else {
                        self.tokens.push(Token {
                            ttype: TType::Identifier(name),
                            lineinfo: self.info,
                        });
                    }
                } else if self.is_number(char) {
                    let mut num = String::from(char);

                    while self.is_valid() && (self.is_number(self.peek()) || self.peek() == '_') {
                        let n = self.peek();
                        if n != '_' {
                            num += &n.to_string();
                        }
                        self.next();
                    }

                    if self.peek() == '.' && self.is_number(self.peek_n(1)) {
                        num += &self.peek().to_string();
                        self.next(); // .

                        while self.is_valid() && (self.is_number(self.peek()) || self.peek() == '_')
                        {
                            let n = self.peek();
                            if n != '_' {
                                num += &n.to_string();
                            }
                            self.next();
                        }
                    }

                    self.tokens.push(Token {
                        ttype: TType::Number(num.parse().unwrap()),
                        lineinfo: self.info,
                    });
                } else {
                    return Err(Error::new(
                        self.info,
                        format!("Invalid token '{}'.", char),
                        ErrorType::SyntaxError,
                    ));
                }
            }
        };
        Ok(())
    }

    // characters
    fn is_alpha(&self, char: char) -> bool {
        ('a' <= char && char <= 'z') || ('A' <= char && char <= 'Z') || char == '_'
    }

    fn is_number(&self, char: char) -> bool {
        '0' <= char && char <= '9'
    }

    fn is_alphanum(&self, char: char) -> bool {
        self.is_alpha(char) || self.is_number(char)
    }

    // advancing
    fn next(&mut self) {
        self.i += 1;
        self.info.col += 1;
    }

    fn newline(&mut self) {
        self.info.line += 1;
        self.info.col = 0;
    }

    // util
    fn append_token(&mut self, token: TType) {
        self.tokens.push(Token {
            ttype: token,
            lineinfo: self.info,
        });
    }

    // lookahead
    fn get(&mut self, char: char) -> bool {
        if self.peek() != char {
            return false;
        }

        self.i += 1;
        true
    }

    fn peek(&self) -> char {
        if !self.is_valid() {
            return '\0';
        }
        self.chars[self.i]
    }

    fn peek_n(&self, n: usize) -> char {
        if self.i + n >= self.chars.len() {
            return '\0';
        }
        self.chars[self.i + n]
    }

    fn read_n(&mut self, n: usize) -> String {
        let mut string = String::new();
        for _ in 0..n {
            string.push(self.peek());
            self.next();
        }

        string
    }

    fn is_valid(&self) -> bool {
        self.i < self.chars.len()
    }
}
