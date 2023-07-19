use std::{fmt::Display, sync::Arc};

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String(Arc<String>),
    Float(f64),
    Integer(i128),
    Boolean(bool),
    Infinity,
    InternalUndefined,
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub value: DataType,
    mutability: bool,
    nullability: bool,
}

impl Variable {
    pub fn is_truthy(&self) -> bool {
        match self.clone().value {
            DataType::String(a) => !a.is_empty(),
            DataType::Float(a) => a != 0.0,
            DataType::Integer(a) => a != 0,
            DataType::Boolean(a) => a,
            DataType::Infinity => true,
            DataType::InternalUndefined => false,
            DataType::Null => false,
        }
    }
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
    pub fn is_mutable(&self) -> bool {
        self.mutability
    }
    pub fn is_nullable(&self) -> bool {
        self.nullability
    }
    pub fn set_mutable(&mut self, mutability: bool) {
        self.mutability = mutability;
    }
    pub fn set_nullable(&mut self, nullability: bool) {
        self.nullability = nullability;
    }
    pub fn new(value: DataType) -> Self {
        Self {
            value,
            mutability: false,
            nullability: false,
        }
    }
    pub fn new_mutable(value: DataType) -> Self {
        Self {
            value,
            mutability: true,
            nullability: false,
        }
    }
    pub fn new_nullable(value: DataType) -> Self {
        Self {
            value,
            mutability: true,
            nullability: true,
        }
    }
    pub fn as_nullable(self) -> Self {
        Self {
            value: self.value.clone(),
            mutability: self.mutability,
            nullability: true,
        }
    }
    pub fn as_mutable(self) -> Self {
        Self {
            value: self.value.clone(),
            mutability: true,
            nullability: self.nullability,
        }
    }
}

impl From<Arc<String>> for Variable {
    fn from(value: Arc<String>) -> Self {
        Self {
            value: DataType::String(value),
            mutability: false,
            nullability: false,
        }
    }
}
impl From<i128> for Variable {
    fn from(value: i128) -> Self {
        Self {
            value: DataType::Integer(value),
            mutability: false,
            nullability: false,
        }
    }
}
impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self {
            value: DataType::Boolean(value),
            mutability: false,
            nullability: false,
        }
    }
}
impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self {
            value: DataType::Float(value),
            mutability: false,
            nullability: false,
        }
    }
}
impl From<DataType> for Variable {
    fn from(value: DataType) -> Self {
        Self {
            value,
            mutability: false,
            nullability: false,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.clone().value {
            DataType::String(a) => format!("'{a}'"),
            DataType::Float(a) => a.to_string(),
            DataType::Integer(a) => a.to_string(),
            DataType::Boolean(a) => a.to_string(),
            DataType::Infinity => "Infinity".to_string(),
            DataType::InternalUndefined => "Undefined".to_string(),
            DataType::Null => "Null".to_string(),
        };
        write!(f, "{text}")
    }
}
