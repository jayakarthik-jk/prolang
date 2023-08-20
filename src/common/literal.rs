use std::{fmt::Display, sync::Arc};

use super::datatypes::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub(crate) value: DataType,
    mutability: bool,
}

impl Literal {
    pub(crate) fn new(value: DataType, mutability: bool) -> Self {
        Self { value, mutability }
    }

    pub(crate) fn is_truthy(&self) -> bool {
        match self.clone().value {
            DataType::String(a) => !a.is_empty(),
            DataType::Float(a) => a != 0.0,
            DataType::Integer(a) => a != 0,
            DataType::Boolean(a) => a,
            DataType::Infinity => true,
            DataType::InternalUndefined => false,
            DataType::Function(_) => true,
            DataType::Return(_) => false,
        }
    }

    pub(crate) fn is_mutable(&self) -> bool {
        self.mutability
    }

    pub(crate) fn new_mutable(value: DataType) -> Self {
        Self {
            value,
            mutability: true,
        }
    }

    pub(crate) fn to_mutable(&self) -> Self {
        Self {
            value: self.value.clone(),
            mutability: true,
        }
    }
}

impl From<Arc<String>> for Literal {
    fn from(value: Arc<String>) -> Self {
        Self {
            value: DataType::String(value),
            mutability: false,
        }
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self {
            value: DataType::String(Arc::new(value)),
            mutability: false,
        }
    }
}

impl From<i128> for Literal {
    fn from(value: i128) -> Self {
        Self {
            value: DataType::Integer(value),
            mutability: false,
        }
    }
}
impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self {
            value: DataType::Boolean(value),
            mutability: false,
        }
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self {
            value: DataType::Float(value),
            mutability: false,
        }
    }
}
impl From<DataType> for Literal {
    fn from(value: DataType) -> Self {
        Self {
            value,
            mutability: false,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.clone().value {
            DataType::String(a) => a.to_string(),
            DataType::Float(a) => a.to_string(),
            DataType::Integer(a) => a.to_string(),
            DataType::Boolean(a) => a.to_string(),
            DataType::Infinity => "Infinity".to_string(),
            DataType::InternalUndefined => "Undefined".to_string(),
            DataType::Function(_) => "Function".to_string(),
            DataType::Return(_) => "Return".to_string(),
        };
        write!(f, "{text}")
    }
}
