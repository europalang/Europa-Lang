pub struct Error {
    line: i32,
    col: i32,
    error: String
}

impl Error {
    fn new(&mut self, line: i32, col: i32, error: String) {
        self.line = line;
        self.col = col;
        self.error = error;
    }

    fn display(&self) {
        println!("[Line:{} Col:{}] {}", self.line, self.col, self.error);
    }
}