use std::fmt::Display;

use crate::common::{datatypes::Variable, errors::CompilerError};
use crate::common::datatypes::DataType::*;

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
    pub fn evaluate(&self, a: Variable, b: Variable) -> Variable {
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
            Logical::Xor => Variable::from(a.is_truthy() ^ b.is_truthy()),
            _ => Variable::from(false),
        }
    }

    pub fn evaluate_unary(&self, a: Variable) -> Result<Variable, CompilerError> {
        let result = match self {
            Logical::Not => match a.value {
                Boolean(a) => Variable::from(!a),
                String(a) => Variable::from(a.len() > 0),
                Float(a) => Variable::from(a != 0.0),
                Integer(a) => Variable::from(a != 0),
                Infinity => Variable::from(Infinity),
                InternalUndefined => return Err(CompilerError::OperationOnUndefined),
            },
            _ => Variable::from(false),
        };
        Ok(result)
    }
}
