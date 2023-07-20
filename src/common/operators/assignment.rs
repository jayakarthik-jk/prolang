use std::fmt::Display;

use crate::common::operators::Arithmetic::*;
use crate::common::{datatypes::Variable, errors::CompilerError};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assingment {
    SimpleAssignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModuloAssignment,
    ExponentiationAssignment,
}

impl Display for Assingment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Assingment::SimpleAssignment => "=",
            Assingment::AdditionAssignment => "+=",
            Assingment::SubtractionAssignment => "-=",
            Assingment::MultiplicationAssignment => "*=",
            Assingment::DivisionAssignment => "/=",
            Assingment::ModuloAssignment => "%=",
            Assingment::ExponentiationAssignment => "**=",
        };
        write!(f, "{}", text)
    }
}

impl Assingment {
    pub fn evaluate(&self, a: Variable, b: Variable) -> Result<Variable, CompilerError> {
        match self {
            Assingment::SimpleAssignment => return Err(CompilerError::InvalidAssignment),
            Assingment::AdditionAssignment => Addition.evaluate(a, b),
            Assingment::SubtractionAssignment => Subtraction.evaluate(a, b),
            Assingment::MultiplicationAssignment => Multiplication.evaluate(a, b),
            Assingment::DivisionAssignment => Division.evaluate(a, b),
            Assingment::ModuloAssignment => Modulo.evaluate(a, b),
            Assingment::ExponentiationAssignment => Exponentiation.evaluate(a, b),
        }
    }
}
