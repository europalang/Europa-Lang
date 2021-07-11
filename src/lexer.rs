use std::usize;

use super::token::*;
use super::error::Error;

pub struct Lexer {
    code: String,
    chars: Vec<char>,
    i: usize, // index
    line: i32,
    col: i32,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(code: &String) -> Self {
        Self {
            code: code.to_string(),
            chars: code.chars().collect(),
            i: 0,
            line: 1,
            col: 1,
            tokens: vec![]
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            let char = self.chars[self.i];

            match char {
                _ => return Err(Error::new(self.line, self.col, format!("Unknown token {}", char)))
            };

            // self.i += 1;
        }

        Ok(self.tokens.to_vec())
    }
}