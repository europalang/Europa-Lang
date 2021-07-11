pub struct Error {
    line: i32,
    col: i32,
    error: String
}

impl Error {
    pub fn new(line: i32, col: i32, error: String) -> Self {
        Self {
            line, col, error
        }
    }

    pub fn display(&self) {
        println!("[Line:{} Col:{}] {}", self.line, self.col, self.error);
    }
}