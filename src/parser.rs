use std::rc::Rc;

use crate::error::{Error, ErrorNote, ErrorType, LineInfo};
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
            let (cond, if_br, elif_brs, else_br) = self.if_stmt()?;
            return Ok(Stmt::IfStmt(cond, if_br, elif_brs, else_br));
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
        if self.get(&[TType::For]) {
            return self.for_stmt();
        }
        if self.get(&[TType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        if self.get(&[TType::Break, TType::Continue, TType::Return]) {
            return self.controlflow_stmt();
        }
        if self.get(&[TType::Fn]) {
            return self.fn_stmt();
        }
        if self.get(&[TType::Use]) {
            return self.use_stmt();
        }

        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> SResult {
        let expr = self.expr()?;

        if !self.get(&[TType::Semi]) {
            let semi = self.peek();
            if semi.ttype != TType::RightBrace {
                return Err(Error::new_n(
                    self.prev().lineinfo,
                    "Expected ';' after statement.".into(),
                    ErrorType::SyntaxError,
                    vec![ErrorNote::Note(
                        "The ';' is optional only if it is the last statement of a block.".into(),
                    )],
                ));
            }
        }

        Ok(Stmt::ExprStmt(expr))
    }

    fn if_stmt(
        &mut self,
    ) -> Result<(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>), Error> {
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

        Ok((cond, true_br, elif_brs, else_br))
    }

    fn var_decl(&mut self) -> SResult {
        let mut vars = Vec::new();

        loop {
            if let TType::Identifier(name) = self.next().ttype {
                let value = if self.get(&[TType::Eq]) {
                    self.expr()?
                } else {
                    Expr::Literal(Type::Nil)
                };

                vars.push((name, value));

                match self.next().ttype {
                    TType::Semi => break,
                    TType::Comma => continue,
                    _ => {
                        return Err(Error::new(
                            self.prev().lineinfo,
                            "Expected ',' or ';' after variable declaration.".into(),
                            ErrorType::SyntaxError,
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    self.prev().lineinfo,
                    "Expected variable name".into(),
                    ErrorType::SyntaxError,
                ));
            }
        }

        Ok(Stmt::VarDecl(vars))
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

    fn for_stmt(&mut self) -> SResult {
        let parens: Option<LineInfo>;

        if self.get(&[TType::LeftParen]) {
            parens = Some(self.prev().lineinfo);
        } else {
            parens = None;
        }

        let name = self.next();
        if !matches!(name.ttype, TType::Identifier(_)) {
            return Err(Error::new(
                name.lineinfo,
                "Expected variable name after 'for' keyword".into(),
                ErrorType::SyntaxError,
            ));
        }

        self.consume(TType::In, "Expected 'in' after for loop variable.".into())?;

        let val = self.expr()?;

        match parens {
            Some(lineinfo) => {
                self.consume_n(
                    TType::RightParen,
                    "Expected ')' after for expression.".into(),
                    vec![ErrorNote::Expect(
                        lineinfo,
                        "Expected ')' to match this.".into(),
                    )],
                )?;
            }
            _ => {}
        }

        self.consume(TType::LeftBrace, "Expected '{' after for expression".into())?;
        let block = self.block()?;

        Ok(Stmt::Block(vec![Stmt::ForStmt(name, val, block)]))
    }

    fn controlflow_stmt(&mut self) -> SResult {
        let tok = self.prev();
        let stype: String;

        let out = match tok.ttype {
            TType::Break => {
                stype = "break keyword".into();
                Stmt::Break(tok)
            }
            TType::Continue => {
                stype = "continue keyword".into();
                Stmt::Continue(tok)
            }
            TType::Return => {
                stype = "return keyword".into();
                let val;

                if !self.check(TType::Semi) {
                    val = Some(self.expr()?);
                } else {
                    val = None;
                }

                Stmt::Return(tok, val)
            }
            _ => panic!(),
        };

        self.consume(TType::Semi, format!("Expected ';' after {}", stype).into())?;
        Ok(out)
    }

    fn fn_stmt(&mut self) -> SResult {
        let name = self.peek();

        if let TType::Identifier(_) = name.ttype {
            self.next();
            let (params, block) = self.finish_fn("function name".into())?;

            return Ok(Stmt::Function(name, params, block));
        } else {
            return Err(Error::new(
                name.lineinfo,
                "Expected function name after 'fn' keyword.".into(),
                ErrorType::SyntaxError,
            ));
        }
    }

    fn use_stmt(&mut self) -> SResult {
        let name = self.next();

        let out = if matches!(name.ttype, TType::Identifier(_)) {
            Stmt::UseStmt(name)
        } // todo: string syntax???
        else {
            return Err(Error::new(
                name.lineinfo,
                "Expected an identifier or a string after use statement.".into(),
                ErrorType::SyntaxError,
            ));
        };

        self.consume(TType::Semi, "Expected ';' after use statement.".into())?;

        Ok(out)
    }

    // expressions
    fn expr(&mut self) -> PResult {
        self.range()
    }

    fn range(&mut self) -> PResult {
        let mut expr = self.ternary()?;

        if self.get(&[TType::DotDot, TType::DotEq]) {
            let tok = self.prev();
            let right = self.expr()?;

            let inclusive = match tok.ttype {
                TType::DotDot => false,
                TType::DotEq => true,
                _ => panic!(),
            };

            expr = Expr::Range(Rc::new(expr), tok, Rc::new(right), inclusive);
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> PResult {
        let mut expr = self.assign()?;

        if self.get(&[TType::Question]) {
            let true_br = self.expr()?;
            self.consume(
                TType::Colon,
                "Expected ':' after ternary if then expression.".into(),
            )?;
            let else_br = self.ternary()?;

            expr = Expr::Ternary(Rc::new(expr), Rc::new(true_br), Rc::new(else_br));
        }

        Ok(expr)
    }

    fn assign(&mut self) -> PResult {
        let expr = self.or()?;

        // Set equal
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
            let val = self.expr()?;

            let tok = if eq.ttype == TType::Eq {
                None
            } else {
                Some(Token {
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
                })
            };

            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assign(
                    var.clone(),
                    if let Some(t) = tok {
                        Rc::new(Expr::Binary(
                            Rc::new(Expr::Variable(var.clone())),
                            t,
                            Rc::new(val),
                        ))
                    } else {
                        Rc::new(val)
                    },
                ));
            } else if let Expr::Get(ref var, ref brack, ref i) = expr {
                // var[idx] = val
                return Ok(Expr::Set(
                    var.clone(),
                    brack.clone(),
                    i.clone(),
                    if let Some(t) = tok {
                        Rc::new(Expr::Binary(Rc::new(expr.clone()), t, Rc::new(val)))
                    } else {
                        Rc::new(val)
                    },
                ));
            }

            return Err(Error::new(
                eq.lineinfo,
                "Only variables and indices of arrays or maps can be assigned to.".into(),
                ErrorType::TypeError,
            ));
        }

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

        self.call()
    }

    fn call(&mut self) -> PResult {
        let mut expr = self.primary()?;

        loop {
            if self.get(&[TType::LeftParen]) {
                expr = self.finish_call(&mut expr)?;
            } else if self.get(&[TType::LeftBrack]) {
                let tok = self.prev();
                let val = self.expr()?;
                self.consume_n(
                    TType::RightBrack,
                    "Expected ']' after accessor value.".into(),
                    vec![ErrorNote::Expect(
                        tok.lineinfo,
                        "Expected the ']' to match this.".into(),
                    )],
                )?;
                expr = Expr::Get(Rc::new(expr), tok, Rc::new(val));
            } else {
                break;
            }
        }

        Ok(expr)
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
        if self.get(&[TType::LeftBrack]) {
            return self.array();
        }
        if self.get(&[TType::LeftBBrace]) {
            return self.map();
        }

        // 'statement-like'
        if self.get(&[TType::LeftBrace]) {
            return Ok(Expr::Block(self.block()?));
        }

        if self.get(&[TType::If]) {
            let (cond, if_br, elif_brs, else_br) = self.if_stmt()?;
            return Ok(Expr::IfExpr(Rc::new(cond), if_br, elif_brs, else_br));
        }

        if self.get(&[TType::LeftParen]) {
            let tok = self.prev();
            let expr = self.expr()?;
            self.consume_n(
                TType::RightParen,
                String::from("Expected ')' after grouping expression."),
                vec![ErrorNote::Expect(
                    tok.lineinfo,
                    "Expected the ')' to match this.".into(),
                )],
            )?;
            return Ok(Expr::Grouping(Rc::new(expr)));
        }

        // literal-like
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
        let tok = self.prev();
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.check(TType::RightBrace) && self.is_valid() {
            stmts.push(self.stmt()?);
        }

        self.consume_n(
            TType::RightBrace,
            "Expected '}' after block expression".into(),
            vec![ErrorNote::Expect(
                tok.lineinfo,
                "Expected '}' to match this.".into(),
            )],
        )?;

        Ok(stmts)
    }

    fn array(&mut self) -> PResult {
        let mut vals = Vec::new();

        while self.peek().ttype != TType::RightBrack {
            vals.push(self.expr()?);

            if !self.get(&[TType::Comma]) && self.peek().ttype != TType::RightBrack {
                return Err(Error::new(
                    self.peek().lineinfo,
                    "Expected ',' after array value".into(),
                    ErrorType::SyntaxError,
                ));
            }
        }

        self.next();

        Ok(Expr::Array(vals))
    }

    fn map(&mut self) -> PResult {
        // todo: enum
        // todo: identifiers
        let mut vals = Vec::new();

        while self.peek().ttype != TType::RightBBrace {
            let key = self.expr()?;
            self.consume(TType::Colon, "Expected ':' after key.".into())?;
            let value = self.expr()?;
            vals.push((key, value));

            if !self.get(&[TType::Comma]) && self.peek().ttype != TType::RightBBrace {
                return Err(Error::new(
                    self.peek().lineinfo,
                    "Expected ',' after map value".into(),
                    ErrorType::SyntaxError,
                ));
            }
        }

        self.next();

        Ok(Expr::Map(vals))
    }

    // util
    fn finish_call(&mut self, expr: &mut Expr) -> PResult {
        let mut args: Vec<Expr> = Vec::new();
        if !self.check(TType::RightParen) {
            loop {
                args.push(self.expr()?);

                if !self.get(&[TType::Comma]) {
                    break;
                }
            }
        }

        let tok = self.consume(TType::RightParen, "Expected ')' after arguments.".into())?;
        Ok(Expr::Call(Rc::new(expr.clone()), tok, args))
    }

    fn finish_fn(&mut self, kind: String) -> Result<(Vec<Token>, Vec<Stmt>), Error> {
        let lineinfo = self
            .consume(
                TType::LeftParen,
                format!("Expected '(' after {}", kind).into(),
            )?
            .lineinfo;

        let mut params: Vec<Token> = Vec::new();
        if !self.check(TType::RightParen) {
            loop {
                let tok = self.peek();

                if let TType::Identifier(_) = tok.ttype {
                    self.next();
                    params.push(tok);
                } else {
                    return Err(Error::new(
                        tok.lineinfo,
                        "Expected paramater name.".into(),
                        ErrorType::SyntaxError,
                    ));
                }

                if !self.get(&[TType::Comma]) {
                    break;
                }
            }
        }

        self.consume_n(
            TType::RightParen,
            "Expected ')' after function paramaters.".into(),
            vec![ErrorNote::Expect(
                lineinfo,
                "Expected ')' to match this.".into(),
            )],
        )?;
        self.consume(TType::LeftBrace, "Expected '{' after ')'".into())?;

        let body = self.block()?;

        Ok((params, body))
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

    fn consume_n(
        &mut self,
        token: TType,
        error_message: String,
        notes: Vec<ErrorNote>,
    ) -> Result<Token, Error> {
        if self.check(token) {
            return Ok(self.next());
        }

        self.synchronize();
        Err(Error::new_n(
            self.peek().lineinfo,
            error_message,
            ErrorType::SyntaxError,
            notes,
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

    /// consume if the current token is in `tokens`
    fn get(&mut self, tokens: &[TType]) -> bool {
        for i in tokens.iter() {
            if self.check(i.clone()) {
                self.next();
                return true;
            }
        }

        false
    }

    /// check if the current token is `token`
    fn check(&self, token: TType) -> bool {
        if !self.is_valid() {
            return false;
        }

        self.peek().ttype == token
    }

    /// get the current token
    fn peek(&self) -> Token {
        self.tokens[self.i].clone()
    }

    /// get the previous token
    fn prev(&self) -> Token {
        self.tokens[self.i - 1].clone()
    }

    /// consume the current token
    fn next(&mut self) -> Token {
        if self.is_valid() {
            self.i += 1;
        }

        self.prev()
    }

    /// check if the current token is EOF
    fn is_valid(&self) -> bool {
        self.peek().ttype != TType::EOF
    }
}
