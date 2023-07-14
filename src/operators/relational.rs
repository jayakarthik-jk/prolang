use crate::datatypes::DataType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relational {
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl Relational {
    pub fn evaluate(&self, a: DataType, b: DataType) -> DataType {
        match self {
            Relational::Equals => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a == b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a == b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a == b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a == b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(true),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(true),
                _ => DataType::Boolean(false),
            },
            Relational::NotEquals => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a != b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a != b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a != b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a != b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(false),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(false),
                _ => DataType::Boolean(true),
            },
            Relational::LessThan => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a < b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a < b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a < b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a < b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(false),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(false),
                _ => DataType::Boolean(false),
            },
            Relational::LessThanOrEquals => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a <= b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a <= b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a <= b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a <= b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(true),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(false),
                _ => DataType::Boolean(false),
            },
            Relational::GreaterThan => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a > b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a > b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a > b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a > b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(false),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(false),
                _ => DataType::Boolean(false),
            },
            Relational::GreaterThanOrEquals => match (a, b) {
                (DataType::String(a), DataType::String(b)) => DataType::Boolean(a >= b),
                (DataType::Float(a), DataType::Float(b)) => DataType::Boolean(a >= b),
                (DataType::Integer(a), DataType::Integer(b)) => DataType::Boolean(a >= b),
                (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Boolean(a >= b),
                (DataType::Infinity, DataType::Infinity) => DataType::Boolean(true),
                (DataType::NAN, DataType::NAN) => DataType::Boolean(false),
                _ => DataType::Boolean(false),
            },
        }
    }
}
