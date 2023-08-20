use std::fmt::Display;

use crate::common::errors::CompilerError;
use crate::common::literal::Literal;
use crate::common::operators::Arithmetic::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assingment {
    Simple,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
}

impl Display for Assingment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Assingment::Simple => "=",
            Assingment::Addition => "+=",
            Assingment::Subtraction => "-=",
            Assingment::Multiplication => "*=",
            Assingment::Division => "/=",
            Assingment::Modulo => "%=",
            Assingment::Exponentiation => "**=",
        };
        write!(f, "{}", text)
    }
}

impl Assingment {
    pub(crate) fn evaluate(&self, a: Literal, b: Literal) -> Result<Literal, CompilerError> {
        match self {
            Assingment::Simple => Err(CompilerError::InvalidAssignment),
            Assingment::Addition => Addition.evaluate(a, b),
            Assingment::Subtraction => Subtraction.evaluate(a, b),
            Assingment::Multiplication => Multiplication.evaluate(a, b),
            Assingment::Division => Division.evaluate(a, b),
            Assingment::Modulo => Modulo.evaluate(a, b),
            Assingment::Exponentiation => Exponentiation.evaluate(a, b),
        }
    }
}
