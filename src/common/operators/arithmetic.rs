use std::{fmt::Display, sync::Arc};

use crate::common::literal::Literal;
use crate::common::{datatypes::DataType::*, errors::CompilerError};

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
    pub(crate) fn evaluate(&self, a: Literal, b: Literal) -> Result<Literal, CompilerError> {
        match self {
            Addition => Self::add(a, b),
            Subtraction => Self::sub(a, b),
            Multiplication => Self::mul(a, b),
            Division => Self::div(a, b),
            Modulo => Self::modulo(a, b),
            Exponentiation => Self::power(a, b),
        }
    }

    pub(crate) fn evaluate_unary(&self, variable: Literal) -> Result<Literal, CompilerError> {
        let result = match self {
            Addition => match variable.value {
                String(a) => {
                    let result: f64 = match a.parse() {
                        Ok(a) => a,
                        Err(_) => {
                            return Err(CompilerError::InvalidStringParsing(Literal::from(
                                a.clone(),
                            )))
                        }
                    };
                    if result.fract() == 0.0 {
                        Literal::from(result as i128)
                    } else {
                        Literal::from(result)
                    }
                }
                a => Literal::from(a),
            },
            Subtraction => match variable.value {
                Float(a) => Literal::from(-a),
                Integer(a) => Literal::from(-a),
                Boolean(a) => Literal::from(if a { -1 } else { 0 }),
                Infinity => variable,
                String(_) => return Err(CompilerError::InvalidUneryOperation),
                InternalUndefined => return Err(CompilerError::OperationOnUndefined),
                Function(_) => return Err(CompilerError::OperationOnFunction),
                Return(_) => return Err(CompilerError::OperationOnReturn),
                Break(_) => return Err(CompilerError::OperationOnBreak),
            },
            operator => {
                return Err(CompilerError::InvalidOperatorForUnaryOperation(
                    Operator::Arithmetic(*operator),
                ))
            }
        };
        Ok(result)
    }

    fn add(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), String(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (String(a), Float(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (String(a), Integer(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (String(a), Boolean(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (String(a), Infinity) => Literal::from(Arc::new(format!("{a}infinity"))),

            (Float(a), String(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (Float(a), Float(b)) => Literal::from(a + b),
            (Float(a), Integer(b)) => Literal::from(a + b as f64),
            (Float(a), Boolean(b)) => Literal::from(if b { a + 1.0 } else { a }),

            (Integer(a), String(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (Integer(a), Float(b)) => Literal::from(a as f64 + b),
            (Integer(a), Integer(b)) => Literal::from(a + b),
            (Integer(a), Boolean(b)) => Literal::from(if b { a + 1 } else { a }),

            (Boolean(a), String(b)) => Literal::from(Arc::new(format!("{a}{b}"))),
            (Boolean(a), Float(b)) => Literal::from(if a { b + 1.0 } else { b }),
            (Boolean(a), Integer(b)) => Literal::from(if a { b + 1 } else { b }),
            (Boolean(a), Boolean(b)) => Literal::from(match (a, b) {
                (true, true) => 2,
                (false, false) => 0,
                _ => 1,
            }),
            (Infinity, String(a)) => Literal::from(Arc::new(format!("infinity{a}"))),
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (_, Infinity) | (Infinity, _) => Literal::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn sub(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(left), right) | (right, String(left)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(left),
                    Operator::Arithmetic(Subtraction),
                    Literal::from(right),
                ))
            }

            (Float(a), Float(b)) => Literal::from(a - b),
            (Float(a), Integer(b)) => Literal::from(a - b as f64),
            (Float(a), Boolean(b)) => Literal::from(if b { a - 1.0 } else { a }),

            (Integer(a), Float(b)) => Literal::from(a as f64 - b),
            (Integer(a), Integer(b)) => Literal::from(a - b),
            (Integer(a), Boolean(b)) => Literal::from(if b { a - 1 } else { a }),
            (Boolean(a), Float(b)) => Literal::from(if a { b - 1.0 } else { b }),
            (Boolean(a), Integer(b)) => Literal::from(if a { b - 1 } else { b }),
            (Boolean(a), Boolean(b)) => Literal::from(match (a, b) {
                (true, false) => 1,
                (false, true) => -1,
                _ => 0,
            }),
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (Infinity, _) | (_, Infinity) => Literal::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn mul(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), String(b)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(a),
                    Operator::Arithmetic(Multiplication),
                    Literal::from(b),
                ))
            }
            (String(a), Float(b)) => {
                let mut result = "".to_string();
                for _ in 0..b as i128 {
                    result += &a;
                }
                Literal::from(Arc::new(result))
            }
            (String(a), Integer(b)) => {
                let mut result = "".to_string();
                for _ in 0..b {
                    result += &a;
                }
                Literal::from(Arc::new(result))
            }
            (String(a), Boolean(b)) => Literal::from(if b { a } else { Arc::new("".to_string()) }),
            (Float(a), String(b)) => {
                let mut result = "".to_string();
                for _ in 0..a as i128 {
                    result += &b;
                }
                Literal::from(Arc::new(result))
            }
            (Float(a), Float(b)) => Literal::from(a * b),
            (Float(a), Integer(b)) => Literal::from(a * b as f64),
            (Float(a), Boolean(b)) => Literal::from(if b { a } else { 0.0 }),
            (Integer(a), String(b)) => {
                let mut result = "".to_string();
                for _ in 0..a {
                    result += &b;
                }
                Literal::from(Arc::new(result))
            }
            (Integer(a), Float(b)) => Literal::from(a as f64 * b),
            (Integer(a), Integer(b)) => Literal::from(a * b),

            (Integer(a), Boolean(b)) => Literal::from(if b { a } else { 0 }),
            (Boolean(a), String(b)) => Literal::from(if a { b } else { Arc::new("".to_string()) }),
            (Boolean(a), Float(b)) => Literal::from(if a { b } else { 0.0 }),
            (Boolean(a), Integer(b)) => Literal::from(if a { b } else { 0 }),
            (Boolean(a), Boolean(b)) => Literal::from(if a == b {
                if a {
                    1
                } else {
                    0
                }
            } else {
                0
            }),
            (String(left), Infinity) | (Infinity, String(left)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(Infinity),
                    Operator::Arithmetic(Multiplication),
                    Literal::from(left),
                ))
            }
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (Infinity, _) | (_, Infinity) => Literal::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn div(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(_), _) | (_, String(_)) => return Err(CompilerError::MathUndefined),
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a / b)
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a / b as f64)
                }
            }
            (Infinity, Boolean(b)) => {
                if b {
                    Literal::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Boolean(a), Boolean(b)) => {
                if b {
                    if a {
                        Literal::from(1.0)
                    } else {
                        Literal::from(0.0)
                    }
                } else {
                    Literal::from(Infinity)
                }
            }
            (Float(a), Boolean(b)) => {
                if b {
                    Literal::from(a)
                } else {
                    Literal::from(Infinity)
                }
            }
            (Integer(a), Boolean(b)) => {
                if b {
                    Literal::from(a as f64)
                } else {
                    Literal::from(Infinity)
                }
            }
            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a as f64 / b)
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(a)
                } else {
                    Literal::from(a as f64 / b as f64)
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Division.evaluate(Literal::from(1.0), Literal::from(b))?
                } else {
                    Literal::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => Division.evaluate(
                Literal::from(if a { 1.0 } else { 0.0 }),
                Literal::from(b as f64),
            )?,
            (Float(a), Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(0.0)
                }
            }
            (Integer(a), Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(0.0)
                }
            }
            (Boolean(a), Infinity) => {
                if a {
                    Literal::from(0.0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Infinity, Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(Infinity)
                }
            }
            (Infinity, Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(a)
                }
            }
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (Infinity, Infinity) => return Err(CompilerError::MathUndefined),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn modulo(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), b) | (b, String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(a),
                    Operator::Arithmetic(Modulo),
                    Literal::from(b),
                ))
            }
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a % b)
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a % b as f64)
                }
            }
            (Infinity, Boolean(a)) => {
                if a {
                    Literal::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Integer(a), Boolean(b)) => {
                if b {
                    Modulo.evaluate(Literal::from(a), Literal::from(1))?
                } else {
                    Literal::from(Infinity)
                }
            }

            (Boolean(_), Boolean(b)) => {
                if b {
                    Literal::from(0)
                } else {
                    Literal::from(Infinity)
                }
            }

            (Float(a), Boolean(b)) => {
                if b {
                    Modulo.evaluate(Literal::from(a), Literal::from(1))?
                } else {
                    Literal::from(Infinity)
                }
            }

            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a as f64 % b)
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(Infinity)
                } else {
                    Literal::from(a % b)
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Modulo.evaluate(Literal::from(1.0), Literal::from(b))?
                } else {
                    Literal::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => {
                Modulo.evaluate(Literal::from(if a { 1 } else { 0 }), Literal::from(b))?
            }
            (Float(a), Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(a)
                }
            }
            (Integer(a), Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(a)
                }
            }
            (Boolean(a), Infinity) => {
                if a {
                    Literal::from(0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Infinity, a) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(Infinity),
                    Operator::Arithmetic(Modulo),
                    Literal::from(a),
                ))
            }
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn power(variable1: Literal, variable2: Literal) -> Result<Literal, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), b) | (b, String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(a),
                    Operator::Arithmetic(Exponentiation),
                    Literal::from(b),
                ))
            }
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(1.0)
                } else {
                    Literal::from(a.powf(b))
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(1.0)
                } else {
                    Literal::from(a.powf(b as f64))
                }
            }
            (Infinity, Boolean(b)) => {
                if b {
                    Literal::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (a, Boolean(b)) => {
                if b {
                    Exponentiation.evaluate(Literal::from(a), Literal::from(1))?
                } else {
                    Exponentiation.evaluate(Literal::from(a), Literal::from(0))?
                }
            }

            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Literal::from(1.0)
                } else {
                    Literal::from((a as f64).powf(b))
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Literal::from(1)
                } else {
                    let result = (a as f64).powf(b as f64);
                    if result.fract() == 0.0 {
                        Literal::from(result as i128)
                    } else {
                        Literal::from(result)
                    }
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Exponentiation.evaluate(Literal::from(1.0), Literal::from(b))?
                } else {
                    Literal::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => {
                Exponentiation.evaluate(Literal::from(if a { 1 } else { 0 }), Literal::from(b))?
            }
            (a, Infinity) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Literal::from(a),
                    Operator::Arithmetic(Exponentiation),
                    Literal::from(Infinity),
                ))
            }
            (Infinity, Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(Infinity)
                }
            }
            (Infinity, Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Literal::from(Infinity)
                }
            }
            (_, Return(_)) | (Return(_), _) => return Err(CompilerError::OperationOnReturn),
            (_, Break(_)) | (Break(_), _) => return Err(CompilerError::OperationOnBreak),
            (_, Function(_)) | (Function(_), _) => return Err(CompilerError::OperationOnFunction),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }
}
