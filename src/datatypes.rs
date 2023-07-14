#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String(String),
    Float(f64),
    Number(i64),
    Boolean(bool),
}
