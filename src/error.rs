#[derive(Debug, Clone, Copy)]
pub struct LineInfo {
    pub line: i32,
    pub col: i32
}

impl LineInfo {
    pub fn new(line: i32, col: i32) -> Self {
        Self {
            line, col
        }
    }
}


pub struct Error {
    info: LineInfo,
    error: String
}

impl Error {
    pub fn new(info: LineInfo, error: String) -> Self {
        Self {
            info,
            error
        }
    }

    pub fn display(&self) {
        println!("[Line:{} Col:{}] {}", self.info.line, self.info.col, self.error);
    }
}