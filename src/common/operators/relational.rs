use std::fmt::Display;
use crate::common::datatypes::Variable;
use crate::common::datatypes::DataType::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relational {
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl Display for Relational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Relational::Equals => "==",
            Relational::NotEquals => "!=",
            Relational::LessThan => "<",
            Relational::LessThanOrEquals => "<=",
            Relational::GreaterThan => ">",
            Relational::GreaterThanOrEquals => ">=",
        };
        write!(f, "{}", text)
    }
}

impl Relational {
    pub fn evaluate(&self, a: Variable, b: Variable) -> Variable {
        let result = match self {
            Relational::Equals => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a == b),
                (Float(a), Float(b)) => Boolean(a == b),
                (Integer(a), Integer(b)) => Boolean(a == b),
                (Boolean(a), Boolean(b)) => Boolean(a == b),
                (Infinity, Infinity) => Boolean(true),
                _ => Boolean(false),
            },
            Relational::NotEquals => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a != b),
                (Float(a), Float(b)) => Boolean(a != b),
                (Integer(a), Integer(b)) => Boolean(a != b),
                (Boolean(a), Boolean(b)) => Boolean(a != b),
                (Infinity, Infinity) => Boolean(false),
                _ => Boolean(true),
            },
            Relational::LessThan => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a < b),
                (Float(a), Float(b)) => Boolean(a < b),
                (Integer(a), Integer(b)) => Boolean(a < b),
                (Boolean(a), Boolean(b)) => Boolean(a < b),
                (Infinity, Infinity) => Boolean(false),
                _ => Boolean(false),
            },
            Relational::LessThanOrEquals => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a <= b),
                (Float(a), Float(b)) => Boolean(a <= b),
                (Integer(a), Integer(b)) => Boolean(a <= b),
                (Boolean(a), Boolean(b)) => Boolean(a <= b),
                (Infinity, Infinity) => Boolean(true),
                _ => Boolean(false),
            },
            Relational::GreaterThan => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a > b),
                (Float(a), Float(b)) => Boolean(a > b),
                (Integer(a), Integer(b)) => Boolean(a > b),
                (Boolean(a), Boolean(b)) => Boolean(a > b),
                (Infinity, Infinity) => Boolean(false),
                _ => Boolean(false),
            },
            Relational::GreaterThanOrEquals => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a >= b),
                (Float(a), Float(b)) => Boolean(a >= b),
                (Integer(a), Integer(b)) => Boolean(a >= b),
                (Boolean(a), Boolean(b)) => Boolean(a >= b),
                (Infinity, Infinity) => Boolean(true),
                _ => Boolean(false),
            },
        };

        Variable::from(result)
    }
}
