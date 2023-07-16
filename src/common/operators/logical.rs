use std::fmt::Display;

use crate::common::datatypes::DataType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Logical {
    And,
    Or,
    Not,
    Xor,
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Logical::And => "and",
            Logical::Or => "or",
            Logical::Not => "not",
            Logical::Xor => "xor",
        };
        write!(f, "{}", text)
    }
}

impl Logical {
    pub fn evaluate(&self, a: DataType, b: DataType) -> DataType {
        match self {
            Logical::And => {
                if !a.is_truthy() {
                    a
                } else {
                    b
                }
            }
            Logical::Or => {
                if a.is_truthy() {
                    a
                } else {
                    b
                }
            }
            Logical::Xor => DataType::Boolean(a.is_truthy() ^ b.is_truthy()),
            _ => DataType::Boolean(false),
        }
    }

    pub fn evaluate_unary(&self, a: DataType) -> DataType {
        match self {
            Logical::Not => match a {
                DataType::Boolean(a) => DataType::Boolean(!a),
                DataType::String(a) => DataType::Boolean(a.len() > 0),
                DataType::Float(a) => DataType::Boolean(a != 0.0),
                DataType::Integer(a) => DataType::Boolean(a != 0),
                DataType::Infinity => DataType::Boolean(false),
                DataType::NAN => DataType::Boolean(true),
            },
            _ => DataType::Boolean(false),
        }
    }
}
