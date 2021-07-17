use std::rc::Rc;

use crate::error::{Error, ErrorType};
use crate::nodes::expr::Expr;
use crate::nodes::stmt::Stmt;
use crate::token::{TType, Token};
use crate::types::Type;

type PResult = Result<Expr, Error>;
type SResult = Result<Stmt, Error>;

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, i: 0 }
    }

    pub fn init(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while self.is_valid() {
            stmts.push(self.stmt()?);
        }

        Ok(stmts)
    }

    // recursive descent
    // statements
    fn stmt(&mut self) -> SResult {
        if self.get(&[TType::If]) {
            return self.if_stmt();
        }
        if self.get(&[TType::Var]) {
            return self.var_decl();
        }
        if self.get(&[TType::While]) {
            return self.while_stmt();
        }
        if self.get(&[TType::Do]) {
            return self.dowhile_stmt();
        }
        if self.get(&[TType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }

        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> SResult {
        let expr = self.expr()?;
        self.consume(TType::Semi, "Expected ';' after statement.".into())?;
        Ok(Stmt::ExprStmt(expr))
    }

    fn if_stmt(&mut self) -> SResult {
        let cond = self.expr()?;
        self.consume(
            TType::LeftBrace,
            "Expected '{' after if statement condition.".into(),
        )?;
        let true_br = self.block()?;
        let mut elif_brs: Vec<(Expr, Vec<Stmt>)> = Vec::new();
        let else_br: Option<Vec<Stmt>>;

        if self.get(&[TType::Elif]) {
            loop {
                let elif_cond = self.expr()?;
                self.consume(
                    TType::LeftBrace,
                    "Expected '{' after elif statement condition.".into(),
                )?;
                elif_brs.push((elif_cond, self.block()?));
                if !self.get(&[TType::Elif]) {
                    break;
                }
            }
        }

        if self.get(&[TType::Else]) {
            self.consume(TType::LeftBrace, "Expected '{' after else keyword.".into())?;
            else_br = Some(self.block()?);
        } else {
            else_br = None;
        }

        Ok(Stmt::IfStmt(cond, true_br, elif_brs, else_br))
    }

    fn var_decl(&mut self) -> SResult {
        if let TType::Identifier(name) = self.peek().ttype {
            self.next();

            let value;
            if self.get(&[TType::Eq]) {
                value = self.expr()?;
            } else {
                value = Expr::Literal(Type::Nil);
            }

            self.consume(
                TType::Semi,
                "Expected ';' after variable declaration.".into(),
            )?;

            Ok(Stmt::VarDecl(name, value))
        } else {
            return Err(Error::new(
                self.peek().lineinfo,
                "Expected variable name after 'var'".into(),
                ErrorType::SyntaxError,
            ));
        }
    }

    fn while_stmt(&mut self) -> SResult {
        let cond = self.expr()?;
        self.consume(
            TType::LeftBrace,
            "Expected '{' after while loop condition.".into(),
        )?;
        let body = self.block()?;

        Ok(Stmt::WhileStmt(cond, body))
    }

    fn dowhile_stmt(&mut self) -> SResult {
        self.consume(TType::LeftBrace, "Expected '{' after do keyword.".into())?;
        let body = self.block()?;
        self.consume(TType::While, "Expected 'while' after do loop body.".into())?;
        let condition = self.expr()?;
        self.consume(
            TType::Semi,
            "Expected ';' after do while loop condition.".into(),
        )?;

        Ok(Stmt::Block(vec![
            Stmt::Block(body.clone()),
            Stmt::WhileStmt(condition, body.clone()),
        ]))
    }

    // expressions
    fn expr(&mut self) -> PResult {
        self.assign()
    }

    fn assign(&mut self) -> PResult {
        let expr = self.or()?;

        if self.get(&[
            TType::Eq,
            TType::PlusEq,
            TType::MinusEq,
            TType::TimesEq,
            TType::DivideEq,
            TType::PowEq,
            TType::ModEq,
        ]) {
            let eq = self.prev();
            let val = self.assign()?;

            if let Expr::Variable(var) = expr {
                if eq.ttype == TType::Eq {
                    return Ok(Expr::Assign(var, Rc::new(val)));
                } else {
                    let tok = Token {
                        ttype: match eq.ttype {
                            TType::PlusEq => TType::Plus,
                            TType::MinusEq => TType::Minus,
                            TType::TimesEq => TType::Times,
                            TType::DivideEq => TType::Divide,
                            TType::PowEq => TType::Pow,
                            TType::ModEq => TType::Mod,
                            _ => panic!(),
                        },
                        ..eq
                    };

                    return Ok(Expr::Assign(
                        var.clone(),
                        Rc::new(Expr::Binary(
                            Rc::new(Expr::Variable(var.clone())),
                            tok,
                            Rc::new(val),
                        )),
                    ));
                }
            }

            return Err(Error::new(
                eq.lineinfo,
                "Invalid assignment target.".into(),
                ErrorType::TypeError,
            ));
        }

        // if self.get(&[
        //     TType::PlusEq,
        //     TType::MinusEq,
        //     TType::TimesEq,
        //     TType::DivideEq,
        //     TType::PowEq,
        //     TType::ModEq,
        // ]) {
        //     let prev = self.prev();
        //     let val = self.assign()?;

        //     let tok = Token {
        //         ttype: match prev.ttype {
        //             TType::PlusEq => TType::Plus,
        //             TType::MinusEq => TType::Minus,
        //             TType::TimesEq => TType::Times,
        //             TType::DivideEq => TType::Divide,
        //             TType::PowEq => TType::Pow,
        //             TType::ModEq => TType::Mod,
        //             _ => panic!(),
        //         },
        //         ..prev
        //     };

        //     if let Expr::Variable(var) = expr {
        //         return Ok(Expr::Assign(var.clone(), Rc::new(Expr::Binary(Rc::new(expr), tok, Rc::new(val)))));
        //     }
        // }

        Ok(expr)
    }

    fn or(&mut self) -> PResult {
        let mut expr = self.and()?;

        while self.get(&[TType::Or]) {
            let op = self.prev();
            let right = self.and()?;
            expr = Expr::Logical(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> PResult {
        let mut expr = self.equality()?;

        while self.get(&[TType::And]) {
            let op = self.prev();
            let right = self.equality()?;
            expr = Expr::Logical(Rc::new(expr), op, Rc::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> PResult {
        let mut expr = self.comp()?;

        while self.get(&[TType::NotEq, TType::EqEq]) {
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
        if self.get(&[TType::LeftBrace]) {
            return Ok(Expr::Block(self.block()?));
        }

        if self.get(&[TType::LeftParen]) {
            let expr = self.expr()?;
            self.consume(
                TType::RightParen,
                String::from("Expected ')' after grouping expression."),
            )?;
            return Ok(Expr::Grouping(Rc::new(expr)));
        }

        self.next();

        let tok = self.prev();
        Ok(match &tok.ttype {
            TType::String(x) => Expr::Literal(Type::String(x.clone())),
            TType::Number(x) => Expr::Literal(Type::Float(*x)),
            TType::Identifier(_) => Expr::Variable(tok),
            _ => {
                return Err(Error::new(
                    tok.lineinfo,
                    format!("Unexpected token '{}'.", tok.ttype),
                    ErrorType::SyntaxError,
                ));
            }
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.check(TType::RightBrace) && self.is_valid() {
            stmts.push(self.stmt()?);
        }

        self.consume(
            TType::RightBrace,
            "Expected '}' after block expression".into(),
        )?;

        Ok(stmts)
    }

    // errors
    fn consume(&mut self, token: TType, error_message: String) -> Result<Token, Error> {
        if self.check(token) {
            return Ok(self.next());
        }

        self.synchronize();
        Err(Error::new(
            self.peek().lineinfo,
            error_message,
            ErrorType::SyntaxError,
        ))
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
            if self.check(i.clone()) {
                self.next();
                return true;
            }
        }

        false
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
