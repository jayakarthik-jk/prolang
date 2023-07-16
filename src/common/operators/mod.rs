use std::fmt::Display;

use crate::common::datatypes::DataType;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical;
use crate::common::operators::relational::Relational;

pub mod arithmetic;
pub mod assignment;
pub mod logical;
pub mod relational;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    ArithmeticOperator(Arithmetic),
    RelationalOperator(Relational),
    AssingmentOperator(Assingment),
    LogicalOperator(Logical),
}

impl Operator {
    pub fn evaluate(&self, a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        match self {
            Operator::ArithmeticOperator(arithmetic) => Ok(arithmetic.evaluate(a, b)),
            Operator::RelationalOperator(relational) => Ok(relational.evaluate(a, b)),
            Operator::LogicalOperator(logical) => Ok(logical.evaluate(a, b)),
            Operator::AssingmentOperator(_) => todo!(),
        }
    }
    pub fn get_binary_precedence(&self) -> u8 {
        match self {
            Operator::ArithmeticOperator(operator) => match operator {
                Arithmetic::Addition => 6,
                Arithmetic::Subtraction => 6,
                Arithmetic::Multiplication => 7,
                Arithmetic::Division => 7,
                Arithmetic::Modulo => 7,
                Arithmetic::Exponentiation => 8,
            },
            Operator::RelationalOperator(operator) => match operator {
                Relational::Equals => 4,
                Relational::NotEquals => 4,
                Relational::LessThan => 5,
                Relational::LessThanOrEquals => 5,
                Relational::GreaterThan => 5,
                Relational::GreaterThanOrEquals => 5,
            },
            Operator::AssingmentOperator(operator) => match operator {
                Assingment::SimpleAssignment => 0,
                Assingment::AdditionAssignment => 0,
                Assingment::SubtractionAssignment => 0,
                Assingment::MultiplicationAssignment => 0,
                Assingment::DivisionAssignment => 0,
                Assingment::ModuloAssignment => 0,
                Assingment::ExponentiationAssignment => 0,
            },
            Operator::LogicalOperator(operator) => match operator {
                Logical::And => 3,
                Logical::Or => 3,
                Logical::Not => 8,
                Logical::Xor => 1,
            },
        }
    }
    pub fn get_unery_precedence(&self) -> u8 {
        match self {
            Operator::ArithmeticOperator(operator) => match operator {
                Arithmetic::Addition => 9,
                Arithmetic::Subtraction => 9,
                _ => 0,
            },
            Operator::LogicalOperator(operator) => match operator {
                Logical::Not => 9,
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::ArithmeticOperator(operator) => write!(f, "{}", operator),
            Operator::RelationalOperator(operator) => write!(f, "{}", operator),
            Operator::AssingmentOperator(operator) => write!(f, "{}", operator),
            Operator::LogicalOperator(operator) => write!(f, "{}", operator),
        }
    }
}
