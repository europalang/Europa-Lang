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
            info: LineInfo::new(1, 1),
            tokens: vec![],
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            let char = self.chars[self.i];
            self.i += 1;

            match char {
                '(' => self.append_token(TType::LeftParen),
                ')' => self.append_token(TType::RigthParen),
                '[' => self.append_token(TType::LeftBrack),
                ']' => self.append_token(TType::RightBrack),

                ',' => self.append_token(TType::Comma),
                '.' => self.append_token(TType::Dot),
                ';' => self.append_token(TType::Semi),
                
                // operators
                '+' => if self.get('=') { self.append_token(TType::PlusEq) } else { self.append_token(TType::Plus) },
                '-' => if self.get('=') { self.append_token(TType::MinusEq) } else { self.append_token(TType::Minus) },
                '*' => if self.get('=') { self.append_token(TType::TimesEq) } else { self.append_token(TType::Times) },
                '/' => if self.get('=') { self.append_token(TType::DivideEq) } else { self.append_token(TType::Divide) },

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

    fn append_token(&mut self, token: TType) {
        self.tokens.push(Token {
            ttype: token,
            lineinfo: self.info,
        });
    }

    fn get(&mut self, char: char) -> bool {
        if self.peek() != char { return false; }

        self.i += 1;
        true
    }

    fn peek(&self) -> char {
        if self.i >= self.chars.len() { return '\0'; }
        return self.chars[self.i];
    }
}
