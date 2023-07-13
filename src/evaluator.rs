use crate::operators::Operator::{ArithmeticOperator, LogicalOperator, RelationalOperator};
use crate::{lexer::TokenKind, parser::Expression};
pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(root: &Box<Expression>) -> Result<i32, String> {
        match root.as_ref() {
            Expression::LiteralExpression(number) if let TokenKind::LiteralToken(number) = number.kind => Ok(number),
            Expression::LiteralExpression(number) => Err(format!(
                        "Unexprected token {:?} expected Number Token",number.kind)),
            Expression::BinaryExpression(left, operator, right) => {
                let left = Evaluator::evaluate(left)?;
                let right = Evaluator::evaluate(right)?;
                match operator.kind {
                    TokenKind::OperatorToken(operator) => {
                        let result = match operator {
                            ArithmeticOperator(_) | RelationalOperator(_) | LogicalOperator(_) => operator.evaluate(left, right),
                            _ => {
                                return Err("invalid operator".to_string())
                            }
                        };
                        Ok(result)
                    }
                    kind => Err(format!(
                        "Unexprected token {:?} expected Operator Token",
                        kind
                    )),
                }
            }
            Expression::ParanthesizedExpression(expression) => Evaluator::evaluate(expression),
        }
    }
}
