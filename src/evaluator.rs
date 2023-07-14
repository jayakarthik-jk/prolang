use crate::datatypes::DataType;
use crate::errors::CompilerError;
use crate::expressions::SyntaxExpression;
use crate::lexer::TokenKind;
use crate::operators::arithmetic::Arithmetic::*;
use crate::operators::Operator;
use crate::operators::Operator::{ArithmeticOperator, LogicalOperator, RelationalOperator};

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(root: &Box<SyntaxExpression>) -> Result<DataType, CompilerError> {
        match root.as_ref() {
            SyntaxExpression::LiteralExpression(number) => {
                if let TokenKind::LiteralToken(number) = &number.kind {
                    Ok(number.clone())
                } else {
                    Err(CompilerError::UnExpectedTokenError(
                        number.kind.clone(),
                        TokenKind::LiteralToken(DataType::Number(0)),
                    ))
                }
            }
            SyntaxExpression::BinaryExpression(left, operator, right) => {
                let left = Evaluator::evaluate(left)?;
                let right = Evaluator::evaluate(right)?;
                match &operator.kind {
                    TokenKind::OperatorToken(operator) => {
                        let result = match operator {
                            ArithmeticOperator(_) | RelationalOperator(_) | LogicalOperator(_) => {
                                operator.evaluate(left, right)?
                            }
                            operator => {
                                return Err(CompilerError::InvalidBinaryOperationError(
                                    left,
                                    operator.clone(),
                                    right,
                                ))
                            }
                        };
                        Ok(result)
                    }
                    kind => Err(CompilerError::UnExpectedOperatorTokenError(kind.clone())),
                }
            }
            SyntaxExpression::UnaryExpression(operator_token, expression) => {
                if let TokenKind::OperatorToken(ArithmeticOperator(operator)) = operator_token.kind
                {
                    match operator {
                        Addition => Evaluator::evaluate(expression),
                        Subtraction => Ok(Operator::ArithmeticOperator(Multiplication)
                            .evaluate(DataType::Float(-1f64), Evaluator::evaluate(expression)?)?),
                        _ => Err(CompilerError::InvalidUnaryOperationError(
                            operator_token.kind.clone(),
                        )),
                    }
                } else {
                    Err(CompilerError::InvalidUnaryOperationError(
                        operator_token.kind.clone(),
                    ))
                }
            }
        }
    }
}
