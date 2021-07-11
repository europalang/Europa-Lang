use std::usize;

use super::token::*;
use super::error::Error;

struct Lexer {
    code: String,
    chars: Vec<char>,
    i: usize, // index
    line: i32,
    col: i32,
    tokens: Vec<Token>
}

impl Lexer {
    fn new(&mut self, code: &String) {
        self.code = code.to_string();
        self.chars = code.chars().collect();
    }

    fn init(&mut self) -> Result<Vec<Token>, Error> {
        while self.i < self.code.len() {
            let char = self.chars[self.i];

            // match char {

            //     _ => 
            // };

            self.i += 1;
        }

        Ok(self.tokens.to_vec())
    }
}