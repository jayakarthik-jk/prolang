use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    Infinity,
    NAN,
}

impl DataType {
    pub fn is_truthy(&self) -> bool {
        match self {
            DataType::String(a) => !a.is_empty(),
            DataType::Float(a) => *a != 0.0,
            DataType::Integer(a) => *a != 0,
            DataType::Boolean(a) => *a,
            DataType::Infinity => true,
            DataType::NAN => false,
        }
    }
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            DataType::String(a) => a.to_string(),
            DataType::Float(a) => a.to_string(),
            DataType::Integer(a) => a.to_string(),
            DataType::Boolean(a) => a.to_string(),
            DataType::Infinity => "Infinity".to_string(),
            DataType::NAN => "NAN (Not a Number)".to_string(),
        };
        write!(f, "{text}")
    }
}
