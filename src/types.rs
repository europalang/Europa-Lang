#[derive(Debug, Clone)]
pub enum Type {
    Number(i32),
    String(String),
    Bool(bool),
    Array(Vec<Type>),
    Nil
}