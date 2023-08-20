use std::fmt::Display;

use crate::common::datatypes::DataType::*;
use crate::common::errors::CompilerError;
use crate::common::literal::Literal;

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
    pub(crate) fn evaluate(&self, a: Literal, b: Literal) -> Literal {
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
            Logical::Xor => Literal::from(a.is_truthy() ^ b.is_truthy()),
            _ => Literal::from(false),
        }
    }

    pub(crate) fn evaluate_unary(&self, variable: Literal) -> Result<Literal, CompilerError> {
        let result = match self {
            Logical::Not => match variable.value {
                Boolean(value) => Literal::from(!value),
                String(value) => Literal::from(value.len() > 0),
                Float(value) => Literal::from(value != 0.0),
                Integer(value) => Literal::from(value != 0),
                Infinity => Literal::from(Infinity),
                InternalUndefined => return Err(CompilerError::OperationOnUndefined),
                Function(_) => return Err(CompilerError::OperationOnFunction),
            },
            _ => Literal::from(false),
        };
        Ok(result)
    }
}
