use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DataType {
    String(Arc<String>),
    Float(f64),
    Integer(i128),
    Boolean(bool),
    Infinity,
    InternalUndefined,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) value: DataType,
    mutability: bool,
}

impl Variable {
    pub(crate) fn is_truthy(&self) -> bool {
        match self.clone().value {
            DataType::String(a) => !a.is_empty(),
            DataType::Float(a) => a != 0.0,
            DataType::Integer(a) => a != 0,
            DataType::Boolean(a) => a,
            DataType::Infinity => true,
            DataType::InternalUndefined => false,
        }
    }

    pub(crate) fn is_mutable(&self) -> bool {
        self.mutability
    }

    // check if needed. if not remove it.

    // pub(crate) fn new(value: DataType) -> Self {
    //     Self {
    //         value,
    //         mutability: false,
    //     }
    // }

    pub(crate) fn new_mutable(value: DataType) -> Self {
        Self {
            value,
            mutability: true,
        }
    }

    pub(crate) fn as_mutable(self) -> Self {
        Self {
            value: self.value,
            mutability: true,
        }
    }
}

impl From<Arc<String>> for Variable {
    fn from(value: Arc<String>) -> Self {
        Self {
            value: DataType::String(value),
            mutability: false,
        }
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self {
            value: DataType::String(Arc::new(value)),
            mutability: false,
        }
    }
}

impl From<i128> for Variable {
    fn from(value: i128) -> Self {
        Self {
            value: DataType::Integer(value),
            mutability: false,
        }
    }
}
impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self {
            value: DataType::Boolean(value),
            mutability: false,
        }
    }
}
impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self {
            value: DataType::Float(value),
            mutability: false,
        }
    }
}
impl From<DataType> for Variable {
    fn from(value: DataType) -> Self {
        Self {
            value,
            mutability: false,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.clone().value {
            DataType::String(a) => format!("{a}"),
            DataType::Float(a) => a.to_string(),
            DataType::Integer(a) => a.to_string(),
            DataType::Boolean(a) => a.to_string(),
            DataType::Infinity => "Infinity".to_string(),
            DataType::InternalUndefined => "Undefined".to_string(),
        };
        write!(f, "{text}")
    }
}
