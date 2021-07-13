use std::rc::Rc;

use super::types::Type;
use super::expr::Expr;
use super::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    // todo: result
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, i: 0 }
    }

    pub fn init(&mut self) -> Expr {
        self.expr()
    }

    // recursive descent
    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comp();

        while self.get(&[TType::NotEq, TType::Eq]) {
            let op = self.prev();
            let right = self.comp();
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        expr
    }

    fn comp(&mut self) -> Expr {
        let mut expr = self.add();

        while self.get(&[TType::Greater, TType::Less, TType::GreaterEq, TType::LessEq]) {
            let op = self.prev();
            let right = self.add();
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        expr
    }

    fn add(&mut self) -> Expr {
        let mut expr = self.mult();

        while self.get(&[TType::Minus, TType::Plus]) {
            let op = self.prev();
            let right = self.mult();
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        expr
    }

    fn mult(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.get(&[TType::Times, TType::Divide, TType::Mod]) {
            let op = self.prev();
            let right = self.unary();
            expr = Expr::Binary(Rc::new(expr), op, Rc::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.get(&[TType::Not, TType::Minus]) {
            let op = self.prev();
            let right = self.unary();
            return Expr::Unary(op, Rc::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.get(&[TType::False]) { return Expr::Literal(Type::Bool(false)); }
        if self.get(&[TType::True]) { return Expr::Literal(Type::Bool(true)); }
        if self.get(&[TType::Nil]) { return Expr::Literal(Type::Nil); }

        match self.get_tup(&[TType::String]) {
            Some(Value::String(x)) => return Expr::Literal(Type::String(x)),
            Some(Value::Float(x)) => return Expr::Literal(Type::Float(x)),
            _ => (),
        }

        if self.get(&[TType::LeftParen]) {
            let expr = self.expr();
            // self.consume(TType::RightParen, "Expected ')' after grouping expression.");
            return Expr::Grouping(Rc::new(expr));
        }

        Expr::Literal(Type::Nil)
    }

    // util
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
                return Some(self.peek().value);
            }
        }

        None
    }

    fn next(&mut self) -> Token {
        if self.is_valid() {
            self.i += 1;
        }

        self.prev()
    }

    fn check(&self, token: TType) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.peek().ttype == token
    }

    fn is_valid(&self) -> bool {
        self.peek().ttype == TType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.i].clone()
    }

    fn prev(&self) -> Token {
        self.tokens[self.i - 1].clone()
    }
}
