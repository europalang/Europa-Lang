use std::usize;

use super::error::*;
use super::token::*;

pub struct Lexer {
    code: String,
    chars: Vec<char>,
    i: usize, // index
    info: LineInfo,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &String) -> Self {
        Self {
            code: code.to_string(),
            chars: code.chars().collect(),
            i: 0,
            info: LineInfo::new(1, 0),
            tokens: vec![],
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            let char = self.chars[self.i];
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
                ')' => self.append_token(TType::RigthParen),
                '[' => self.append_token(TType::LeftBrack),
                ']' => self.append_token(TType::RightBrack),

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
                                self.info.line,
                                self.info.col,
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
                    return Err(Error::new(
                        self.info.line,
                        self.info.col,
                        format!("Invalid token {}", char),
                    ))
                }
            };
        }

        self.append_token(TType::EOF);

        Ok(self.tokens.to_vec())
    }

    fn next(&mut self) {
        self.i += 1;
        self.info.col += 1;
    }

    fn newline(&mut self) {
        self.info.line += 1;
        self.info.col = 0;
    }

    fn append_token(&mut self, token: TType) {
        self.tokens.push(Token {
            ttype: token,
            lineinfo: self.info,
        });
    }

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
