use super::error::*;
use super::token::*;

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

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            let char = self.peek();
            self.next();

            match char {
                '{' => {
                    if self.get('{') {
                        self.append_token(TType::LeftS)
                    } else {
                        self.append_token(TType::LeftBrace)
                    }
                }
                '}' => {
                    if self.get('}') {
                        self.append_token(TType::RightS)
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
                    let str_type = char;
                    let mut str = String::new();

                    while self.is_valid() && self.peek() != str_type {
                        str += &self.peek().to_string();

                        if self.peek() == '\n' {
                            self.newline();
                        }

                        self.next();
                    }

                    if !self.is_valid() {
                        return Err(Error::new(self.info, String::from("Unterminated string.")));
                    }

                    self.next(); // "

                    self.tokens.push(Token {
                        ttype: TType::String,
                        lineinfo: self.info,
                        value: Value::String(str),
                    });
                }

                ',' => self.append_token(TType::Comma),
                '.' => self.append_token(TType::Dot),
                ';' => self.append_token(TType::Semi),

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
                        while self.is_valid() && (self.peek() != '*' && self.peek_n(1) != '/') {
                            if self.peek() == '\n' {
                                self.newline();
                            }

                            self.next();
                        }

                        if !self.is_valid() {
                            return Err(Error::new(
                                self.info,
                                String::from("Unterminated multiline comment."),
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
                                ttype: self.keywords[&name],
                                lineinfo: self.info,
                                value: Value::Nil,
                            });
                        } else {
                            self.tokens.push(Token {
                                ttype: TType::Identifier,
                                lineinfo: self.info,
                                value: Value::Ident(name),
                            });
                        }
                    } else if self.is_number(char) {
                        let mut num = String::from(char);

                        while self.is_valid() && (self.is_number(self.peek()) || self.peek() == '_')
                        {
                            let n = self.peek();
                            if n != '_' {
                                num += &n.to_string();
                            }
                            self.next();
                        }

                        if self.peek() == '.' {
                            num += &self.peek().to_string();
                            self.next(); // .

                            while self.is_valid()
                                && (self.is_number(self.peek()) || self.peek() == '_')
                            {
                                let n = self.peek();
                                if n != '_' {
                                    num += &n.to_string();
                                }
                                self.next();
                            }
                        }

                        self.tokens.push(Token {
                            ttype: TType::Number,
                            lineinfo: self.info,
                            value: Value::Float(num.parse().unwrap()),
                        });
                    } else {
                        return Err(Error::new(self.info, format!("Invalid token {}", char)));
                    }
                }
            };
        }

        self.append_token(TType::EOF);

        Ok(self.tokens.clone())
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
            value: Value::Nil,
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

    fn is_valid(&self) -> bool {
        self.i < self.chars.len()
    }
}
