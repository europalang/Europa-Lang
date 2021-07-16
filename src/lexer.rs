use crate::error::{Error, ErrorType, LineInfo};
use crate::token::{TType, Token};

use std::char::from_u32 as char_from_u32;
use maplit::hashmap;
use std::collections::HashMap;

pub struct Lexer {
    code: String,
    chars: Vec<char>,
    i: usize, // index
    curr_type: String,
    info: LineInfo,
    tokens: Vec<Token>,
    keywords: HashMap<String, TType>,
    operators: HashMap<String, TType>,
    separators: HashMap<String, TType>,
    group1: HashMap<String, TType>,
}

impl Lexer {
    pub fn new(code: &String) -> Self {
        Self {
            code: code.to_string(),
            chars: code.chars().collect(),
            i: 0,
            curr_type: String::from(""),
            info: LineInfo::new(1, 0),
            tokens: vec![],

            // Any length keyword
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

            // Any length string, will group
            operators: hashmap! {
                "+".into() => TType::Plus,
                "+=".into() => TType::PlusEq,
                "-".into() => TType::Minus,
                "-=".into() => TType::MinusEq,
                "*".into() => TType::Times,
                "*=".into() => TType::TimesEq,
                "/".into() => TType::Divide,
                "/=".into() => TType::DivideEq,
                "%".into() => TType::Mod,
                "%=".into() => TType::ModEq,
                "**".into() => TType::Pow,
                "**=".into() => TType::PowEq,
                "!".into() => TType::Not,
                "!=".into() => TType::NotEq,
                "=".into() => TType::Eq,
                "==".into() => TType::EqEq,
                ">".into() => TType::Greater,
                ">=".into() => TType::GreaterEq,
                "<".into() => TType::Less,
                "<=".into() => TType::LessEq,
            },

            // Single character, does not group
            separators: hashmap! {
                "(".into() => TType::LeftParen,
                ")".into() => TType::RightParen,
                "[".into() => TType::LeftBrack,
                "]".into() => TType::RightBrack,
                ".".into() => TType::Dot,
                ",".into() => TType::Comma,
                ";".into() => TType::Semi,
            },

            // Any length, will group
            group1: hashmap! {
                "{".into() => TType::LeftBrace,
                "}".into() => TType::RightBrace,
                "{{".into() => TType::LeftS,
                "}}".into() => TType::RightS,
            }
        }
    }

    fn add_token(&mut self, str: &String) -> Result<(), Error> {
        // Check if a keyword
        if self.keywords.contains_key(str) {
            Ok(self.tokens.push(Token {
                ttype: self.keywords[str].clone(),
                lineinfo: self.info
            }))

        // Check if an operator
        } else if self.operators.contains_key(str) {
            Ok(self.tokens.push(Token {
                ttype: self.operators[str].clone(),
                lineinfo: self.info
            }))

        // Check if a separator
        } else if self.separators.contains_key(str) {
            Ok(self.tokens.push(Token {
                ttype: self.separators[str].clone(),
                lineinfo: self.info
            }))

        // Check if in group1
        } else if self.group1.contains_key(str) {
            Ok(self.tokens.push(Token {
                ttype: self.group1[str].clone(),
                lineinfo: self.info
            }))

        // Variables, Numbers
        } else {
            Ok(self.tokens.push(Token {
                // Check if str is a number
                ttype: match str.parse::<f32>() {
                    Ok(num) => TType::Number(num),

                    // Variable
                    Err(_) => match str.len() {
                        0 => return Ok(()), // Empty
                        _ => TType::Identifier(str.to_string())
                    }
                },
                lineinfo: self.info
            }))
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Error> {
        let mut curr_str = String::from("");

        while self.i < self.code.len() {
            let char = self.peek();
            self.next();

            match char {
                ' '|'\r'|'\t' => {
                    self.add_token(&curr_str).unwrap();
                    curr_str = String::from("");
                    self.curr_type = String::from("");
                }
                '\n' => self.newline(),

                // Separators, will push a token immediately
                '('|')'|'['|']'|'.'|','|';' => {
                    self.add_token(&curr_str).unwrap();
                    curr_str = String::from("");
                    self.add_token(&char.to_string()).unwrap();
                    self.curr_type = String::from("");
                },

                // Groupers, will group with each other
                '{'|'}' => {
                    if &self.curr_type[..] != "group1" {
                        match self.add_token(&curr_str) {
                            Ok(_) => curr_str = char.to_string(),
                            Err(e) => return Err(e)
                        };
                        self.curr_type = String::from("group1");
                    }
                }
                
                // Operators, will group with each other
                '+'|'-'|'*'|'%'|'='|'!'|'>'|'<' => {
                    if &self.curr_type[..] != "operator" {
                        match self.add_token(&curr_str) {
                            Ok(_) => curr_str = char.to_string(),
                            Err(e) => return Err(e)
                        };
                        self.curr_type = String::from("operator");
                    }
                },
                
                // Division or comment
                '/' => {
                    if self.get('=') {
                        self.append_token(&TType::DivideEq)
                    } else if self.get('/') {
                        while self.peek() != '\n' && self.is_valid() {
                            self.next();
                        }
                    } else if self.get('*') {
                        while self.is_valid() && (self.peek() != '*' && self.peek_n(1) != '/') {
                            if self.peek() == '\n' { self.newline() };
                            self.next();
                        }

                        if !self.is_valid() {
                            return Err(Error::new(
                                self.info,
                                String::from("Unterminated multiline comment."),
                                ErrorType::SyntaxError,
                            ));
                        }

                        self.next(); self.next(); // End of multiline "*/"
                    } else {
                        self.append_token(&TType::Divide)
                    }
                }
                
                // Strings, will loop until end of string
                '"' | '\'' => {
                    // Enter previous token
                    self.add_token(&curr_str).unwrap();
                    curr_str = String::from("");
                    self.curr_type = String::from("");

                    
                    let str_type = char; // " or '
                    let mut str = String::new();

                    while self.is_valid() && self.peek() != str_type {
                        if self.peek() == '\n' { self.newline() };
                        
                        // Escape characters
                        if self.peek() == '\\' {
                            self.next();
                            let ch = self.peek();
                            self.next();

                            str.push(match ch {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                'a' => '\x07',
                                'b' => '\x08',
                                'e' => '\x1b',
                                'f' => '\x0c',
                                'v' => '\x0b',
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
                        return Err(Error::new(
                            self.info,
                            String::from("Unterminated string."),
                            ErrorType::SyntaxError,
                        ));
                    }

                    self.next(); // "

                    self.tokens.push(Token {
                        ttype: TType::String(str),
                        lineinfo: self.info,
                    });
                },
                
                // Add char to current string
                _ => curr_str.push(char)
            };
        }
        
        // Add final token and EOF
        match self.add_token(&curr_str) {
            Ok(_) => (), Err(e) => return Err(e)
        }
        self.append_token(&TType::EOF);

        Ok(self.tokens.clone())
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
    fn append_token(&mut self, token: &TType) {
        self.tokens.push(Token {
            ttype: token.clone(),
            lineinfo: self.info,
        });
    }

    // lookahead
    fn get(&mut self, char: char) -> bool {
        if self.peek() != char { return false };
        self.i += 1;
        true
    }

    fn peek(&self) -> char {
        if !self.is_valid() { return '\0' };
        self.chars[self.i]
    }

    fn peek_n(&self, n: usize) -> char {
        if self.i + n >= self.chars.len() { return '\0' };
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
