use crate::common::datatypes::DataType::*;
use crate::common::datatypes::Variable;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relational {
    Equality,
    InEquality,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl Display for Relational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Relational::Equality => "==",
            Relational::InEquality => "!=",
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
            Relational::Equality => match (a.value, b.value) {
                (String(a), String(b)) => Boolean(a == b),
                (Float(a), Float(b)) => Boolean(a == b),
                (Integer(a), Integer(b)) => Boolean(a == b),
                (Boolean(a), Boolean(b)) => Boolean(a == b),
                (Infinity, Infinity) => Boolean(true),
                _ => Boolean(false),
            },
            Relational::InEquality => match (a.value, b.value) {
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
                (Boolean(a), Boolean(b)) => Boolean(!a & b),
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
                (Boolean(a), Boolean(b)) => Boolean(a & !b),
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
