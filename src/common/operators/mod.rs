use std::fmt::Display;

use crate::common::errors::CompilerError;
use crate::common::literal::Literal;
use crate::common::operators::arithmetic::Arithmetic;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical;
use crate::common::operators::relational::Relational;

pub(crate) mod arithmetic;
pub(crate) mod assignment;
pub(crate) mod logical;
pub(crate) mod relational;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Arithmetic(Arithmetic),
    Relational(Relational),
    Assignment(Assingment),
    Logical(Logical),
}

impl Operator {
    pub(crate) fn evaluate(&self, a: Literal, b: Literal) -> Result<Literal, CompilerError> {
        match self {
            Operator::Arithmetic(arithmetic) => arithmetic.evaluate(a, b),
            Operator::Relational(relational) => Ok(relational.evaluate(a, b)),
            Operator::Logical(logical) => Ok(logical.evaluate(a, b)?),
            Operator::Assignment(_) => todo!(),
        }
    }
    pub(crate) fn get_binary_precedence(&self) -> u8 {
        match self {
            Operator::Arithmetic(operator) => match operator {
                Arithmetic::Addition => 6,
                Arithmetic::Subtraction => 6,
                Arithmetic::Multiplication => 7,
                Arithmetic::Division => 7,
                Arithmetic::Modulo => 7,
                Arithmetic::Exponentiation => 8,
            },
            Operator::Relational(operator) => match operator {
                Relational::Equality => 4,
                Relational::InEquality => 4,
                Relational::LessThan => 5,
                Relational::LessThanOrEquals => 5,
                Relational::GreaterThan => 5,
                Relational::GreaterThanOrEquals => 5,
            },
            Operator::Assignment(operator) => match operator {
                Assingment::Simple => 0,
                Assingment::Addition => 0,
                Assingment::Subtraction => 0,
                Assingment::Multiplication => 0,
                Assingment::Division => 0,
                Assingment::Modulo => 0,
                Assingment::Exponentiation => 0,
            },
            Operator::Logical(operator) => match operator {
                Logical::And => 3,
                Logical::Or => 3,
                Logical::Not => 8,
                Logical::Xor => 1,
            },
        }
    }
    pub(crate) fn get_unery_precedence(&self) -> u8 {
        match self {
            Operator::Arithmetic(operator) => match operator {
                Arithmetic::Addition => 9,
                Arithmetic::Subtraction => 9,
                _ => 0,
            },
            Operator::Logical(Logical::Not) => 9,
            _ => 0,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Arithmetic(operator) => write!(f, "{}", operator),
            Operator::Relational(operator) => write!(f, "{}", operator),
            Operator::Assignment(operator) => write!(f, "{}", operator),
            Operator::Logical(operator) => write!(f, "{}", operator),
        }
    }
}
