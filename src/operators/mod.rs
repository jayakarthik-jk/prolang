use crate::{datatypes::DataType, errors::CompilerError};

use self::{
    arithmetic::Arithmetic, assignment::Assingment, logical::Logical, relational::Relational,
};

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
            Operator::ArithmeticOperator(arithmetic) => arithmetic.evaluate(a, b),
            _ => {
                todo!("not implemented")
            }
        }
    }
    pub fn get_binary_precedence(&self) -> u8 {
        match self {
            Operator::ArithmeticOperator(operator) => match operator {
                Arithmetic::Addition => 4,
                Arithmetic::Subtraction => 4,
                Arithmetic::Multiplication => 5,
                Arithmetic::Division => 5,
                Arithmetic::Modulo => 5,
            },
            Operator::RelationalOperator(_) => 3,
            Operator::AssingmentOperator(_) => 2,
            Operator::LogicalOperator(_) => 1,
        }
    }
    pub fn get_unery_precedence(&self) -> u8 {
        match self {
            Operator::ArithmeticOperator(operator) => match operator {
                Arithmetic::Addition => 5,
                Arithmetic::Subtraction => 5,
                // Arithmetic::Multiplication => 4,
                // Arithmetic::Division => 4,
                // Arithmetic::Modulo => 4,
                _ => 0,
            },
            // Operator::RelationalOperator(_) => 3,
            // Operator::AssingmentOperator(_) => 2,
            // Operator::LogicalOperator(_) => 1,
            _ => 0,
        }
    }
}
