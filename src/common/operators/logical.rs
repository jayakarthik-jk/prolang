use std::fmt::Display;

use crate::common::datatypes::DataType::*;
use crate::common::errors::CompilerError;
use crate::common::variables::Variable;

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
    pub(crate) fn evaluate(&self, a: Variable, b: Variable) -> Variable {
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

    pub(crate) fn evaluate_unary(&self, variable: Variable) -> Result<Variable, CompilerError> {
        let result = match self {
            Logical::Not => match variable.value {
                Boolean(value) => Variable::from(!value),
                String(value) => Variable::from(value.len() > 0),
                Float(value) => Variable::from(value != 0.0),
                Integer(value) => Variable::from(value != 0),
                Infinity => Variable::from(Infinity),
                InternalUndefined => return Err(CompilerError::OperationOnUndefined),
                Function(_) => return Err(CompilerError::OperationOnFunction),
            },
            _ => Variable::from(false),
        };
        Ok(result)
    }
}
