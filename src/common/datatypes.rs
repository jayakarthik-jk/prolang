use std::sync::Arc;

use super::functions::Function;
use super::literal::Literal;

#[derive(Debug, Clone)]
pub(crate) enum DataType {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Infinity,
    InternalUndefined,
    String(Arc<String>),
    Function(Arc<Function>),
    Return(Box<Literal>),
    Break(Box<Literal>),
    Skip(Box<Literal>),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::String(a), DataType::String(b)) => a == b,
            (DataType::Float(a), DataType::Float(b)) => a == b,
            (DataType::Integer(a), DataType::Integer(b)) => a == b,
            (DataType::Boolean(a), DataType::Boolean(b)) => a == b,
            (DataType::Infinity, DataType::Infinity) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DataType::Integer(_) => "Integer",
            DataType::Float(_) => "Float",
            DataType::Boolean(_) => "Boolean",
            DataType::Infinity => "Infinity",
            DataType::InternalUndefined => "Undefined",
            DataType::String(_) => "String",
            DataType::Function(_) => "Function",
            DataType::Return(_) => "return",
            DataType::Break(_) => "break",
            DataType::Skip(_) => "skip",
        };
        write!(f, "{}", name)
    }
}
