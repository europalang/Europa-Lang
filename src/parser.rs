use std::rc::Rc;

use super::error::*;
use super::expr::Expr;
use super::token::*;
use super::types::Type;

type PResult = Result<Expr, Error>;

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, i: 0 }
    }

    pub fn init(&mut self) -> PResult {
        self.expr()
    }

    // recursive descent
    fn expr(&mut self) -> PResult {
        self.equality()
    }

    fn equality(&mut self) -> PResult {
        let mut expr = self.comp()?;

        while self.get(&[TType::NotEq, TType::Eq]) {
            let op = self.prev();
            let right = self.comp()?;
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn comp(&mut self) -> PResult {
        let mut expr = self.add()?;

        while self.get(&[TType::Greater, TType::Less, TType::GreaterEq, TType::LessEq]) {
            let op = self.prev();
            let right = self.add()?;
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn add(&mut self) -> PResult {
        let mut expr = self.mult()?;

        while self.get(&[TType::Minus, TType::Plus]) {
            let op = self.prev();
            let right = self.mult()?;
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn mult(&mut self) -> PResult {
        let mut expr = self.unary()?;

        while self.get(&[TType::Times, TType::Divide, TType::Mod]) {
            let op = self.prev();
            let right = self.unary()?;
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> PResult {
        if self.get(&[TType::Not, TType::Minus]) {
            let op = self.prev();
            let right = self.unary()?;
            return Ok(Expr::Unary(op, Rc::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> PResult {
        if self.get(&[TType::False]) {
            return Ok(Expr::Literal(Type::Bool(false)));
        }
        if self.get(&[TType::True]) {
            return Ok(Expr::Literal(Type::Bool(true)));
        }
        if self.get(&[TType::Nil]) {
            return Ok(Expr::Literal(Type::Nil));
        }

        if let Some(x) = self.get_tup(&[TType::String, TType::Number]) {
            return Ok(match x {
                Value::String(v) => Expr::Literal(Type::String(v)),
                Value::Float(v) => Expr::Literal(Type::Float(v)),
                _ => Expr::Literal(Type::Nil)
            })
        }
        
        if self.get(&[TType::LeftParen]) {
            let expr = self.expr()?;
            self.consume(
                TType::RightParen,
                String::from("Expected ')' after grouping expression."),
            )?;
            return Ok(Expr::Grouping(Rc::new(expr)));
        }

        Ok(Expr::Literal(Type::Nil))
    }

    // errors
    fn consume(&mut self, token: TType, message: String) -> Result<Token, Error> {
        if self.check(token) {
            return Ok(self.next());
        }
        Err(Error::new(self.peek().lineinfo, message))
    }

    fn synchronize(&mut self) {
        self.next();

        while self.is_valid() {
            if self.prev().ttype == TType::Semi {
                return;
            }

            match self.peek().ttype {
                TType::Use
                | TType::Fn
                | TType::Var
                | TType::For
                | TType::If
                | TType::Return
                | TType::While => {
                    return;
                }
                _ => (),
            }

            self.next();
        }
    }

    // lookahead
    fn get(&mut self, tokens: &[TType]) -> bool {
        for i in tokens.iter() {
            if self.check(*i) {
                self.next();
                return true;
            }
        }

        false
    }

    fn get_tup(&mut self, tokens: &[TType]) -> Option<Value> {
        for i in tokens.iter() {
            if self.check(*i) {
                self.next();
                return Some(self.prev().value);
            }
        }

        None
    }

    fn check(&self, token: TType) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.peek().ttype == token
    }

    fn peek(&self) -> Token {
        self.tokens[self.i].clone()
    }

    fn prev(&self) -> Token {
        self.tokens[self.i - 1].clone()
    }

    // other
    fn next(&mut self) -> Token {
        if self.is_valid() {
            self.i += 1;
        }

        self.prev()
    }

    fn is_valid(&self) -> bool {
        self.peek().ttype != TType::EOF
    }
}
