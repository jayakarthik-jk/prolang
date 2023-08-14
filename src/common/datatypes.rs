use std::sync::Arc;

use super::functions::Function;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DataType {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Infinity,
    InternalUndefined,
    String(Arc<String>),
    Function(Arc<Function>),
}
