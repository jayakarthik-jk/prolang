use std::{fmt::Display, sync::Arc};

use crate::common::{
    datatypes::{DataType::*, Variable},
    errors::CompilerError,
};

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
    pub fn evaluate(&self, a: Variable, b: Variable) -> Result<Variable, CompilerError> {
        match self {
            Addition => Self::add(a, b),
            Subtraction => Self::sub(a, b),
            Multiplication => Self::mul(a, b),
            Division => Self::div(a, b),
            Modulo => Self::modulo(a, b),
            Exponentiation => Self::power(a, b),
        }
    }

    pub fn evaluate_unary(&self, variable: Variable) -> Result<Variable, CompilerError> {
        let result = match self {
            Addition => match variable.value {
                String(a) => {
                    let result: f64 = match a.parse() {
                        Ok(a) => a,
                        Err(_) => {
                            return Err(CompilerError::InvalidStringParsing(Variable::from(
                                a.clone(),
                            )))
                        }
                    };
                    if result.fract() == 0.0 {
                        Variable::from(result as i128)
                    } else {
                        Variable::from(result)
                    }
                }
                a => Variable::from(a),
            },
            Subtraction => match variable.value {
                Float(a) => Variable::from(-a),
                Integer(a) => Variable::from(-a),
                Boolean(a) => Variable::from(if a { -1 } else { 0 }),
                Infinity => variable,
                String(_) => return Err(CompilerError::InvalidUneryOperation),
                InternalUndefined => return Err(CompilerError::OperationOnUndefined),
            },
            operator => {
                return Err(CompilerError::InvalidOperatorForUnaryOperation(
                    Operator::ArithmeticOperator(*operator),
                ))
            }
        };
        Ok(result)
    }

    fn add(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), String(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (String(a), Float(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (String(a), Integer(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (String(a), Boolean(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (String(a), Infinity) => Variable::from(Arc::new(format!("{a}infinity"))),

            (Float(a), String(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (Float(a), Float(b)) => Variable::from(a + b),
            (Float(a), Integer(b)) => Variable::from(a + b as f64),
            (Float(a), Boolean(b)) => Variable::from(if b { a + 1.0 } else { a }),

            (Integer(a), String(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (Integer(a), Float(b)) => Variable::from(a as f64 + b),
            (Integer(a), Integer(b)) => Variable::from(a + b),
            (Integer(a), Boolean(b)) => Variable::from(if b { a + 1 } else { a }),

            (Boolean(a), String(b)) => Variable::from(Arc::new(format!("{a}{b}"))),
            (Boolean(a), Float(b)) => Variable::from(if a { b + 1.0 } else { b }),
            (Boolean(a), Integer(b)) => Variable::from(if a { b + 1 } else { b }),
            (Boolean(a), Boolean(b)) => Variable::from(match (a, b) {
                (true, true) => 2,
                (false, false) => 0,
                _ => 1,
            }),
            (Infinity, String(a)) => Variable::from(Arc::new(format!("infinity{a}"))),
            (_, Infinity) => Variable::from(Infinity),
            (Infinity, _) => Variable::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn sub(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(left), right) | (right, String(left)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(left),
                    Operator::ArithmeticOperator(Subtraction),
                    Variable::from(right),
                ))
            }

            (Float(a), Float(b)) => Variable::from(a - b),
            (Float(a), Integer(b)) => Variable::from(a - b as f64),
            (Float(a), Boolean(b)) => Variable::from(if b { a - 1.0 } else { a }),

            (Integer(a), Float(b)) => Variable::from(a as f64 - b),
            (Integer(a), Integer(b)) => Variable::from(a - b),
            (Integer(a), Boolean(b)) => Variable::from(if b { a - 1 } else { a }),
            (Boolean(a), Float(b)) => Variable::from(if a { b - 1.0 } else { b }),
            (Boolean(a), Integer(b)) => Variable::from(if a { b - 1 } else { b }),
            (Boolean(a), Boolean(b)) => Variable::from(match (a, b) {
                (true, false) => 1,
                (false, true) => -1,
                _ => 0,
            }),
            (Infinity, _) => Variable::from(Infinity),
            (_, Infinity) => Variable::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn mul(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), String(b)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(a),
                    Operator::ArithmeticOperator(Multiplication),
                    Variable::from(b),
                ))
            }
            (String(a), Float(b)) => {
                let mut result = "".to_string();
                for _ in 0..b as i128 {
                    result += &a;
                }
                Variable::from(Arc::new(result))
            }
            (String(a), Integer(b)) => {
                let mut result = "".to_string();
                for _ in 0..b {
                    result += &a;
                }
                Variable::from(Arc::new(result))
            }
            (String(a), Boolean(b)) => Variable::from(if b { a } else { Arc::new("".to_string()) }),
            (Float(a), String(b)) => {
                let mut result = "".to_string();
                for _ in 0..a as i128 {
                    result += &b;
                }
                Variable::from(Arc::new(result))
            }
            (Float(a), Float(b)) => Variable::from(a * b),
            (Float(a), Integer(b)) => Variable::from(a * b as f64),
            (Float(a), Boolean(b)) => Variable::from(if b { a } else { 0.0 }),
            (Integer(a), String(b)) => {
                let mut result = "".to_string();
                for _ in 0..a {
                    result += &b;
                }
                Variable::from(Arc::new(result))
            }
            (Integer(a), Float(b)) => Variable::from(a as f64 * b),
            (Integer(a), Integer(b)) => Variable::from(a * b),

            (Integer(a), Boolean(b)) => Variable::from(if b { a } else { 0 }),
            (Boolean(a), String(b)) => Variable::from(if a { b } else { Arc::new("".to_string()) }),
            (Boolean(a), Float(b)) => Variable::from(if a { b } else { 0.0 }),
            (Boolean(a), Integer(b)) => Variable::from(if a { b } else { 0 }),
            (Boolean(a), Boolean(b)) => Variable::from(if a == b {
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
                    Variable::from(Infinity),
                    Operator::ArithmeticOperator(Multiplication),
                    Variable::from(left),
                ))
            }
            (Infinity, _) => Variable::from(Infinity),
            (_, Infinity) => Variable::from(Infinity),
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn div(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(_), _) | (_, String(_)) => return Err(CompilerError::MathUndefined),
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a / b)
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a / b as f64)
                }
            }
            (Infinity, Boolean(b)) => {
                if b {
                    Variable::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Boolean(a), Boolean(b)) => {
                if b {
                    if a {
                        Variable::from(1.0)
                    } else {
                        Variable::from(0.0)
                    }
                } else {
                    Variable::from(Infinity)
                }
            }
            (Float(a), Boolean(b)) => {
                if b {
                    Variable::from(a)
                } else {
                    Variable::from(Infinity)
                }
            }
            (Integer(a), Boolean(b)) => {
                if b {
                    Variable::from(a as f64)
                } else {
                    Variable::from(Infinity)
                }
            }
            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a as f64 / b)
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(a)
                } else {
                    Variable::from(a as f64 / b as f64)
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Division.evaluate(Variable::from(1.0), Variable::from(b))?
                } else {
                    Variable::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => Division.evaluate(
                Variable::from(if a { 1.0 } else { 0.0 }),
                Variable::from(b as f64),
            )?,
            (Float(a), Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(0.0)
                }
            }
            (Integer(a), Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(0.0)
                }
            }
            (Boolean(a), Infinity) => {
                if a {
                    Variable::from(0.0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Infinity, Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(Infinity)
                }
            }
            (Infinity, Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(a)
                }
            }
            (Infinity, Infinity) => return Err(CompilerError::MathUndefined),

            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn modulo(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), b) | (b, String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(a),
                    Operator::ArithmeticOperator(Modulo),
                    Variable::from(b),
                ))
            }
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a % b)
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a % b as f64)
                }
            }
            (Infinity, Boolean(a)) => {
                if a {
                    Variable::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Integer(a), Boolean(b)) => {
                if b {
                    Modulo.evaluate(Variable::from(a), Variable::from(1))?
                } else {
                    Variable::from(Infinity)
                }
            }

            (Boolean(a), Boolean(b)) => {
                if b {
                    if a {
                        Variable::from(0)
                    } else {
                        Variable::from(0)
                    }
                } else {
                    Variable::from(Infinity)
                }
            }

            (Float(a), Boolean(b)) => {
                if b {
                    Modulo.evaluate(Variable::from(a), Variable::from(1))?
                } else {
                    Variable::from(Infinity)
                }
            }

            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a as f64 % b)
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(Infinity)
                } else {
                    Variable::from(a % b)
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Modulo.evaluate(Variable::from(1.0), Variable::from(b))?
                } else {
                    Variable::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => {
                Modulo.evaluate(Variable::from(if a { 1 } else { 0 }), Variable::from(b))?
            }
            (Float(a), Infinity) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(a)
                }
            }
            (Integer(a), Infinity) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(a)
                }
            }
            (Boolean(a), Infinity) => {
                if a {
                    Variable::from(0)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (Infinity, a) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(Infinity),
                    Operator::ArithmeticOperator(Modulo),
                    Variable::from(a),
                ))
            }
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }

    fn power(variable1: Variable, variable2: Variable) -> Result<Variable, CompilerError> {
        let result = match (variable1.value, variable2.value) {
            (String(a), b) | (b, String(a)) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(a),
                    Operator::ArithmeticOperator(Exponentiation),
                    Variable::from(b),
                ))
            }
            (Float(a), Float(b)) => {
                if b == 0.0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(1.0)
                } else {
                    Variable::from(a.powf(b))
                }
            }
            (Float(a), Integer(b)) => {
                if b == 0 && a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(1.0)
                } else {
                    Variable::from(a.powf(b as f64))
                }
            }
            (Infinity, Boolean(b)) => {
                if b {
                    Variable::from(Infinity)
                } else {
                    return Err(CompilerError::MathUndefined);
                }
            }
            (a, Boolean(b)) => {
                if b {
                    Exponentiation.evaluate(Variable::from(a), Variable::from(1))?
                } else {
                    Exponentiation.evaluate(Variable::from(a), Variable::from(0))?
                }
            }

            (Integer(a), Float(b)) => {
                if b == 0.0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0.0 {
                    Variable::from(1.0)
                } else {
                    Variable::from((a as f64).powf(b))
                }
            }
            (Integer(a), Integer(b)) => {
                if b == 0 && a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else if b == 0 {
                    Variable::from(1)
                } else {
                    let result = (a as f64).powf(b as f64);
                    if result.fract() == 0.0 {
                        Variable::from(result as i128)
                    } else {
                        Variable::from(result)
                    }
                }
            }
            (Boolean(a), Float(b)) => {
                if a {
                    Exponentiation.evaluate(Variable::from(1.0), Variable::from(b))?
                } else {
                    Variable::from(0.0)
                }
            }
            (Boolean(a), Integer(b)) => {
                Exponentiation.evaluate(Variable::from(if a { 1 } else { 0 }), Variable::from(b))?
            }
            (a, Infinity) => {
                return Err(CompilerError::UnsupportedOperationBetween(
                    Variable::from(a),
                    Operator::ArithmeticOperator(Exponentiation),
                    Variable::from(Infinity),
                ))
            }
            (Infinity, Float(a)) => {
                if a == 0.0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(Infinity)
                }
            }
            (Infinity, Integer(a)) => {
                if a == 0 {
                    return Err(CompilerError::MathUndefined);
                } else {
                    Variable::from(Infinity)
                }
            }
            (_, InternalUndefined) | (InternalUndefined, _) => {
                return Err(CompilerError::OperationOnUndefined)
            }
        };
        Ok(result)
    }
}
