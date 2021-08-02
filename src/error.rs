use crate::types::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineInfo {
    pub line: i32,
    pub col: i32,
}

impl LineInfo {
    pub fn new(line: i32, col: i32) -> Self {
        Self { line, col }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorType {
    MathError,
    TypeError,
    SyntaxError,
    ReferenceError,
    
    Break,
    Continue,
    Return(Type),
}

#[derive(Clone, Debug)]
pub struct Error {
    info: LineInfo,
    pub error_type: ErrorType,
    error: String,
}

impl Error {
    pub fn new(info: LineInfo, error: String, error_type: ErrorType) -> Self {
        Self {
            info,
            error,
            error_type,
        }
    }

    pub fn display(&self, code: &String) {
        let line = self.info.line as usize;
        let col = self.info.col as usize;
        
        let message = format!("[\x1b[1m{}\x1b[0m:\x1b[1m{}\x1b[0m] \x1b[1m{:?}\x1b[0m: \x1b[31m\x1b[1m{}\x1b[0m", line, col, self.error_type, self.error);
        let gutter = format!("\x1b[1m{}\x1b[0m | ", line);
        let editor = format!("{}{}", gutter, code.split('\n').collect::<Vec<&str>>()[line - 1]);

        eprintln!(
            "{msg}
{edt}
\x1b[94m{arrow:->length$} here\x1b[0m",
            msg = message,
            edt = editor,
            length = col + gutter.len() - 8, // - \x1b[1m\x1b[0m
            arrow = '^',
        );
    }
}
