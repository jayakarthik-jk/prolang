use crate::{datatypes::DataType, errors::CompilerError};

use super::Operator;

use self::Arithmetic::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arithmetic {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

impl Arithmetic {
    pub fn evaluate(&self, a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        match self {
            Addition => match (a, b) {
                (DataType::String(a), DataType::String(b)) => Ok(DataType::String(a + &b)),
                (DataType::String(a), DataType::Float(b)) => {
                    Ok(DataType::String(a + &b.to_string()))
                }
                (DataType::String(a), DataType::Number(b)) => {
                    Ok(DataType::String(a + &b.to_string()))
                }
                (DataType::String(a), DataType::Boolean(b)) => {
                    Ok(DataType::String(a + &b.to_string()))
                }

                (DataType::Float(a), DataType::String(b)) => {
                    Ok(DataType::String(a.to_string() + &b))
                }
                (DataType::Float(a), DataType::Float(b)) => Ok(DataType::Float(a + b)),
                (DataType::Float(a), DataType::Number(b)) => Ok(DataType::Float(a + b as f64)),
                (DataType::Float(a), DataType::Boolean(b)) => {
                    Ok(DataType::Float(if b { a + 1.0 } else { a }))
                }
                (DataType::Number(a), DataType::String(b)) => {
                    Ok(DataType::String(a.to_string() + &b))
                }
                (DataType::Number(a), DataType::Float(b)) => Ok(DataType::Float(a as f64 + b)),
                (DataType::Number(a), DataType::Number(b)) => Ok(DataType::Number(a + b)),
                (DataType::Number(a), DataType::Boolean(b)) => {
                    Ok(DataType::Number(if b { a + 1 } else { a }))
                }
                (DataType::Boolean(a), DataType::String(b)) => {
                    Ok(DataType::String(a.to_string() + &b))
                }
                (DataType::Boolean(a), DataType::Float(b)) => {
                    Ok(DataType::Float(if a { b + 1.0 } else { b }))
                }
                (DataType::Boolean(a), DataType::Number(b)) => {
                    Ok(DataType::Number(if a { b + 1 } else { b }))
                }
                (DataType::Boolean(a), DataType::Boolean(b)) => {
                    Ok(DataType::Number(match (a, b) {
                        (true, true) => 2,
                        (false, false) => 0,
                        _ => 1,
                    }))
                }
            },
            Subtraction => match (a, b) {
                (DataType::String(a), b) => Err(CompilerError::InvalidBinaryOperationError(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Subtraction),
                    b,
                )),
                (a, DataType::String(b)) => Err(CompilerError::InvalidBinaryOperationError(
                    a,
                    Operator::ArithmeticOperator(Subtraction),
                    DataType::String(b),
                )),

                (DataType::Float(a), DataType::Float(b)) => Ok(DataType::Float(a - b)),
                (DataType::Float(a), DataType::Number(b)) => Ok(DataType::Float(a - b as f64)),
                (DataType::Float(a), DataType::Boolean(b)) => {
                    Ok(DataType::Float(if b { a + 1.0 } else { a }))
                }

                (DataType::Number(a), DataType::Float(b)) => Ok(DataType::Float(a as f64 - b)),
                (DataType::Number(a), DataType::Number(b)) => Ok(DataType::Number(a - b)),
                (DataType::Number(a), DataType::Boolean(b)) => {
                    Ok(DataType::Number(if b { a + 1 } else { a }))
                }
                (DataType::Boolean(a), DataType::Float(b)) => {
                    Ok(DataType::Float(if a { b + 1.0 } else { b }))
                }
                (DataType::Boolean(a), DataType::Number(b)) => {
                    Ok(DataType::Number(if a { b + 1 } else { b }))
                }
                (DataType::Boolean(a), DataType::Boolean(b)) => {
                    Ok(DataType::Number(match (a, b) {
                        (true, false) => 1,
                        (false, true) => -1,
                        _ => 0,
                    }))
                }
            },
            Multiplication => match (a, b) {
                (DataType::String(a), DataType::String(b)) => {
                    Err(CompilerError::InvalidBinaryOperationError(
                        DataType::String(a),
                        Operator::ArithmeticOperator(Multiplication),
                        DataType::String(b),
                    ))
                }
                (DataType::String(a), DataType::Float(b)) => {
                    let mut result = String::new();
                    for _ in 0..b as i64 {
                        result += &a;
                    }
                    Ok(DataType::String(result))
                }
                (DataType::String(a), DataType::Number(b)) => {
                    let mut result = String::new();
                    for _ in 0..b {
                        result += &a;
                    }
                    Ok(DataType::String(result))
                }
                (DataType::String(a), DataType::Boolean(b)) => {
                    Ok(DataType::String(if b { a } else { String::new() }))
                }
                (DataType::Float(a), DataType::String(b)) => {
                    let mut result = String::new();
                    for _ in 0..a as i64 {
                        result += &b;
                    }
                    Ok(DataType::String(result))
                }
                (DataType::Float(a), DataType::Float(b)) => Ok(DataType::Float(a * b)),
                (DataType::Float(a), DataType::Number(b)) => Ok(DataType::Float(a * b as f64)),
                (DataType::Float(a), DataType::Boolean(b)) => {
                    Ok(DataType::Float(if b { a } else { 0.0 }))
                }
                (DataType::Number(a), DataType::String(b)) => {
                    let mut result = String::new();
                    for _ in 0..a {
                        result += &b;
                    }
                    Ok(DataType::String(result))
                }
                (DataType::Number(a), DataType::Float(b)) => Ok(DataType::Float(a as f64 * b)),
                (DataType::Number(a), DataType::Number(b)) => Ok(DataType::Number(a * b)),

                (DataType::Number(a), DataType::Boolean(b)) => {
                    Ok(DataType::Number(if b { a } else { 0 }))
                }
                (DataType::Boolean(a), DataType::String(b)) => {
                    Ok(DataType::String(if a { b } else { String::new() }))
                }
                (DataType::Boolean(a), DataType::Float(b)) => {
                    Ok(DataType::Float(if a { b } else { 0.0 }))
                }
                (DataType::Boolean(a), DataType::Number(b)) => {
                    Ok(DataType::Number(if a { b } else { 0 }))
                }
                (DataType::Boolean(a), DataType::Boolean(b)) => {
                    Ok(DataType::Boolean(if a == b { a } else { false }))
                }
            },
            Division => match (a, b) {
                (DataType::String(a), b) => Err(CompilerError::InvalidBinaryOperationError(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Division),
                    b,
                )),
                (a, DataType::String(b)) => Err(CompilerError::InvalidBinaryOperationError(
                    a,
                    Operator::ArithmeticOperator(Division),
                    DataType::String(b),
                )),
                (DataType::Float(a), DataType::Float(b)) => Ok(if b == 0.0 {
                    todo!()
                } else {
                    DataType::Float(a / b)
                }),
                (DataType::Float(a), DataType::Number(b)) => Ok(if b == 0 {
                    todo!()
                } else {
                    DataType::Float(a / b as f64)
                }),
                (a, DataType::Boolean(b)) => Ok(if b { a } else { todo!() }),
                (DataType::Number(a), DataType::Float(b)) => Ok(if b == 0.0 {
                    todo!()
                } else {
                    DataType::Float(a as f64 / b)
                }),
                (DataType::Number(a), DataType::Number(b)) => Ok(if b == 0 {
                    todo!()
                } else {
                    DataType::Float(a as f64 / b as f64)
                }),
                (DataType::Boolean(a), DataType::Float(b)) => Ok(if a {
                    Modulo.evaluate(DataType::Float(1.0), DataType::Float(b))?
                } else {
                    DataType::Float(0.0)
                }),
                (DataType::Boolean(a), DataType::Number(b)) => Ok(if a {
                    Modulo.evaluate(DataType::Float(1.0), DataType::Number(b))?
                } else {
                    DataType::Number(0)
                }),
            },
            Modulo => match (a, b) {
                (DataType::String(a), b) => Err(CompilerError::InvalidBinaryOperationError(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Modulo),
                    b,
                )),
                (a, DataType::String(b)) => Err(CompilerError::InvalidBinaryOperationError(
                    a,
                    Operator::ArithmeticOperator(Modulo),
                    DataType::String(b),
                )),
                (DataType::Float(a), DataType::Float(b)) => Ok(if b == 0.0 {
                    todo!()
                } else {
                    DataType::Float(a % b)
                }),
                (DataType::Float(a), DataType::Number(b)) => Ok(if b == 0 {
                    todo!()
                } else {
                    DataType::Float(a % b as f64)
                }),
                (DataType::Float(a), DataType::Boolean(b)) => Ok(if b {
                    Modulo.evaluate(DataType::Float(a), DataType::Float(1.0))?
                } else {
                    todo!()
                }),
                (DataType::Number(a), DataType::Float(b)) => Ok(DataType::Float(a as f64 % b)),
                (DataType::Number(a), DataType::Number(b)) => Ok(DataType::Number(a % b)),
                (DataType::Number(_), DataType::Boolean(b)) => {
                    Ok(if b { DataType::Number(0) } else { todo!() })
                }
                (DataType::Boolean(a), DataType::Float(b)) => Ok(if a {
                    Modulo.evaluate(DataType::Float(1.0), DataType::Float(b))?
                } else {
                    DataType::Float(0.0)
                }),
                (DataType::Boolean(a), DataType::Number(b)) => Ok(if a {
                    Modulo.evaluate(DataType::Number(1), DataType::Number(b))?
                } else {
                    DataType::Number(0)
                }),
                (DataType::Boolean(_), DataType::Boolean(b)) => {
                    Ok(DataType::Number(if b { 0 } else { todo!() }))
                }
            },
        }
    }
}
