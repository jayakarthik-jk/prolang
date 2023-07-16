use std::fmt::Display;

use crate::common::{datatypes::DataType, errors::CompilerError};

use self::Arithmetic::*;

use super::Operator;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arithmetic {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
}

impl Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Addition => write!(f, "+"),
            Subtraction => write!(f, "-"),
            Multiplication => write!(f, "*"),
            Division => write!(f, "/"),
            Modulo => write!(f, "%"),
            Exponentiation => write!(f, "**"),
        }
    }
}

impl Arithmetic {
    pub fn evaluate(&self, a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        match self {
            Addition => Self::add(a, b),
            Subtraction => Self::sub(a, b),
            Multiplication => Self::mul(a, b),
            Division => Self::div(a, b),
            Modulo => Self::modulo(a, b),
            Exponentiation => Self::power(a, b),
        }
    }

    pub fn evaluate_unary(&self, value: DataType) -> Result<DataType, CompilerError> {
        let result = match self {
            Addition => match value {
                DataType::String(a) => {
                    let result: f64 = match a.parse() {
                        Ok(a) => a,
                        Err(_) => {
                            return Err(CompilerError::InvalidStringParsing(DataType::String(
                                a.clone(),
                            )))
                        }
                    };
                    if result.fract() == 0.0 {
                        DataType::Integer(result as i128)
                    } else {
                        DataType::Float(result)
                    }
                }
                a => a,
            },
            Subtraction => match value {
                DataType::Float(a) => DataType::Float(-a),
                DataType::Integer(a) => DataType::Integer(-a),
                DataType::Boolean(a) => DataType::Integer(if a { -1 } else { 0 }),
                DataType::Infinity => DataType::Infinity,
                DataType::String(_) => return Err(CompilerError::InvalidUneryOperation),
                DataType::InternalUndefined => return Err(CompilerError::OperationOnUndefined),
            },
            operator => {
                return Err(CompilerError::INvalidOperatorForUnaryOperation(
                    Operator::ArithmeticOperator(*operator),
                ))
            }
        };
        Ok(result)
    }

    fn add(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(a), DataType::String(b)) => DataType::String(a + &b),
            (DataType::String(a), DataType::Float(b)) => DataType::String(a + &b.to_string()),
            (DataType::String(a), DataType::Integer(b)) => DataType::String(a + &b.to_string()),
            (DataType::String(a), DataType::Boolean(b)) => DataType::String(a + &b.to_string()),
            (DataType::String(a), DataType::Infinity) => DataType::String(a + "Infinity"),

            (DataType::Float(a), DataType::String(b)) => DataType::String(a.to_string() + &b),
            (DataType::Float(a), DataType::Float(b)) => DataType::Float(a + b),
            (DataType::Float(a), DataType::Integer(b)) => DataType::Float(a + b as f64),
            (DataType::Float(a), DataType::Boolean(b)) => {
                DataType::Float(if b { a + 1.0 } else { a })
            }

            (DataType::Integer(a), DataType::String(b)) => DataType::String(a.to_string() + &b),
            (DataType::Integer(a), DataType::Float(b)) => DataType::Float(a as f64 + b),
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a + b),
            (DataType::Integer(a), DataType::Boolean(b)) => {
                DataType::Integer(if b { a + 1 } else { a })
            }

            (DataType::Boolean(a), DataType::String(b)) => DataType::String(a.to_string() + &b),
            (DataType::Boolean(a), DataType::Float(b)) => {
                DataType::Float(if a { b + 1.0 } else { b })
            }
            (DataType::Boolean(a), DataType::Integer(b)) => {
                DataType::Integer(if a { b + 1 } else { b })
            }
            (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Integer(match (a, b) {
                (true, true) => 2,
                (false, false) => 0,
                _ => 1,
            }),
            (DataType::Infinity, DataType::String(a)) => {
                DataType::String("Infinity".to_string() + &a)
            }
            (_, DataType::Infinity) | (DataType::Infinity, _) => DataType::Infinity,
            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn sub(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(a), b) | (b, DataType::String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Subtraction),
                    b,
                ))
            }

            (DataType::Float(a), DataType::Float(b)) => DataType::Float(a - b),
            (DataType::Float(a), DataType::Integer(b)) => DataType::Float(a - b as f64),
            (DataType::Float(a), DataType::Boolean(b)) => {
                DataType::Float(if b { a - 1.0 } else { a })
            }

            (DataType::Integer(a), DataType::Float(b)) => DataType::Float(a as f64 - b),
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a - b),
            (DataType::Integer(a), DataType::Boolean(b)) => {
                DataType::Integer(if b { a - 1 } else { a })
            }
            (DataType::Boolean(a), DataType::Float(b)) => {
                DataType::Float(if a { b - 1.0 } else { b })
            }
            (DataType::Boolean(a), DataType::Integer(b)) => {
                DataType::Integer(if a { b - 1 } else { b })
            }
            (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Integer(match (a, b) {
                (true, false) => 1,
                (false, true) => -1,
                _ => 0,
            }),
            (_, DataType::Infinity) | (DataType::Infinity, _) => DataType::Infinity,
            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn mul(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(a), DataType::String(b)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Multiplication),
                    DataType::String(b),
                ))
            }
            (DataType::String(a), DataType::Float(b)) => {
                let mut result = String::new();
                for _ in 0..b as i128 {
                    result += &a;
                }
                DataType::String(result)
            }
            (DataType::String(a), DataType::Integer(b)) => {
                let mut result = String::new();
                for _ in 0..b {
                    result += &a;
                }
                DataType::String(result)
            }
            (DataType::String(a), DataType::Boolean(b)) => {
                DataType::String(if b { a } else { String::new() })
            }
            (DataType::Float(a), DataType::String(b)) => {
                let mut result = String::new();
                for _ in 0..a as i128 {
                    result += &b;
                }
                DataType::String(result)
            }
            (DataType::Float(a), DataType::Float(b)) => DataType::Float(a * b),
            (DataType::Float(a), DataType::Integer(b)) => DataType::Float(a * b as f64),
            (DataType::Float(a), DataType::Boolean(b)) => DataType::Float(if b { a } else { 0.0 }),
            (DataType::Integer(a), DataType::String(b)) => {
                let mut result = String::new();
                for _ in 0..a {
                    result += &b;
                }
                DataType::String(result)
            }
            (DataType::Integer(a), DataType::Float(b)) => DataType::Float(a as f64 * b),
            (DataType::Integer(a), DataType::Integer(b)) => DataType::Integer(a * b),

            (DataType::Integer(a), DataType::Boolean(b)) => {
                DataType::Integer(if b { a } else { 0 })
            }
            (DataType::Boolean(a), DataType::String(b)) => {
                DataType::String(if a { b } else { String::new() })
            }
            (DataType::Boolean(a), DataType::Float(b)) => DataType::Float(if a { b } else { 0.0 }),
            (DataType::Boolean(a), DataType::Integer(b)) => {
                DataType::Integer(if a { b } else { 0 })
            }
            (DataType::Boolean(a), DataType::Boolean(b)) => DataType::Integer(if a == b {
                if a {
                    1
                } else {
                    0
                }
            } else {
                0
            }),
            (DataType::String(a), DataType::Infinity)
            | (DataType::Infinity, DataType::String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::Infinity,
                    Operator::ArithmeticOperator(Multiplication),
                    DataType::String(a),
                ))
            }
            (_, DataType::Infinity) | (DataType::Infinity, _) => DataType::Infinity,
            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn div(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(_), _) | (_, DataType::String(_)) => {
                return Err(CompilerError::MathUndefined)
            }
            (DataType::Float(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a / b)
                }
            }
            (DataType::Float(a), DataType::Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a / b as f64)
                }
            }
            (DataType::Infinity, DataType::Boolean(b)) => {
                if b {
                    DataType::Infinity
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (DataType::Boolean(a), DataType::Boolean(b)) => {
                if b {
                    if a {
                        DataType::Float(1.0)
                    } else {
                        DataType::Float(0.0)
                    }
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Float(a), DataType::Boolean(b)) => {
                if b {
                    DataType::Float(a)
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Integer(a), DataType::Boolean(b)) => {
                if b {
                    DataType::Float(a as f64)
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Integer(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a as f64 / b)
                }
            }
            (DataType::Integer(a), DataType::Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a as f64 / b as f64)
                }
            }
            (DataType::Boolean(a), DataType::Float(b)) => {
                if a {
                    Division.evaluate(DataType::Float(1.0), DataType::Float(b))?
                } else {
                    DataType::Float(0.0)
                }
            }
            (DataType::Boolean(a), DataType::Integer(b)) => Division.evaluate(
                DataType::Float(if a { 1.0 } else { 0.0 }),
                DataType::Float(b as f64),
            )?,
            (DataType::Float(a), DataType::Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Float(0.0)
                }
            }
            (DataType::Integer(a), DataType::Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Float(0.0)
                }
            }
            (DataType::Boolean(a), DataType::Infinity) => {
                if a {
                    DataType::Float(0.0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (DataType::Infinity, DataType::Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Infinity, DataType::Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Infinity, DataType::Infinity) => return Err(CompilerError::MathUndefined),

            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn modulo(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(a), b) | (b, DataType::String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Modulo),
                    b,
                ))
            }
            (DataType::Float(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a % b)
                }
            }
            (DataType::Float(a), DataType::Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a % b as f64)
                }
            }
            (DataType::Infinity, DataType::Boolean(a)) => {
                if a {
                    DataType::Infinity
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (DataType::Integer(a), DataType::Boolean(b)) => {
                if b {
                    Modulo.evaluate(DataType::Integer(a), DataType::Integer(1))?
                } else {
                    DataType::Infinity
                }
            }

            (DataType::Boolean(a), DataType::Boolean(b)) => {
                if b {
                    if a {
                        DataType::Integer(0)
                    } else {
                        DataType::Integer(0)
                    }
                } else {
                    DataType::Infinity
                }
            }

            (DataType::Float(a), DataType::Boolean(b)) => {
                if b {
                    Modulo.evaluate(DataType::Float(a), DataType::Integer(1))?
                } else {
                    DataType::Infinity
                }
            }

            (DataType::Integer(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Infinity
                } else {
                    DataType::Float(a as f64 % b)
                }
            }
            (DataType::Integer(a), DataType::Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Infinity
                } else {
                    DataType::Integer(a % b)
                }
            }
            (DataType::Boolean(a), DataType::Float(b)) => {
                if a {
                    Modulo.evaluate(DataType::Float(1.0), DataType::Float(b))?
                } else {
                    DataType::Float(0.0)
                }
            }
            (DataType::Boolean(a), DataType::Integer(b)) => Modulo.evaluate(
                DataType::Integer(if a { 1 } else { 0 }),
                DataType::Integer(b),
            )?,
            (DataType::Float(a), DataType::Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Float(a)
                }
            }
            (DataType::Integer(a), DataType::Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Integer(a)
                }
            }
            (DataType::Boolean(a), DataType::Infinity) => {
                if a {
                    DataType::Integer(0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (DataType::Infinity, a) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::Infinity,
                    Operator::ArithmeticOperator(Modulo),
                    a,
                ))
            }
            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn power(a: DataType, b: DataType) -> Result<DataType, CompilerError> {
        let result = match (a, b) {
            (DataType::String(a), b) | (b, DataType::String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    DataType::String(a),
                    Operator::ArithmeticOperator(Exponentiation),
                    b,
                ))
            }
            (DataType::Float(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Float(1.0)
                } else {
                    DataType::Float(a.powf(b))
                }
            }
            (DataType::Float(a), DataType::Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Float(1.0)
                } else {
                    DataType::Float(a.powf(b as f64))
                }
            }
            (DataType::Infinity, DataType::Boolean(b)) => {
                if b {
                    DataType::Infinity
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (a, DataType::Boolean(b)) => {
                if b {
                    Exponentiation.evaluate(a, DataType::Integer(1))?
                } else {
                    Exponentiation.evaluate(a, DataType::Integer(0))?
                }
            }

            (DataType::Integer(a), DataType::Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    DataType::Float(1.0)
                } else {
                    DataType::Float((a as f64).powf(b))
                }
            }
            (DataType::Integer(a), DataType::Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    DataType::Integer(1)
                } else {
                    let result = (a as f64).powf(b as f64);
                    if result.fract() == 0.0 {
                        DataType::Integer(result as i128)
                    } else {
                        DataType::Integer(result as i128)
                    }
                }
            }
            (DataType::Boolean(a), DataType::Float(b)) => {
                if a {
                    Exponentiation.evaluate(DataType::Float(1.0), DataType::Float(b))?
                } else {
                    DataType::Float(0.0)
                }
            }
            (DataType::Boolean(a), DataType::Integer(b)) => Exponentiation.evaluate(
                DataType::Integer(if a { 1 } else { 0 }),
                DataType::Integer(b),
            )?,
            (a, DataType::Infinity) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    a,
                    Operator::ArithmeticOperator(Exponentiation),
                    DataType::Infinity,
                ))
            }
            (DataType::Infinity, DataType::Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Infinity
                }
            }
            (DataType::Infinity, DataType::Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    DataType::Infinity
                }
            }
            (_, DataType::InternalUndefined) | (DataType::InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }
}
