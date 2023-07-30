use std::rc::Rc;

use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical::Not;
use crate::common::operators::Operator::{
    self, ArithmeticOperator, LogicalOperator, RelationalOperator,
};
use crate::syntax_analysis::ast::AbstractSyntaxTree;
use crate::syntax_analysis::block::Block;

pub struct Evaluator<'a> {
    statement: &'a Box<AbstractSyntaxTree>,
    global_block: Rc<Block>,
}

impl<'a> Evaluator<'a> {
    pub fn new(statement: &'a Box<AbstractSyntaxTree>, global_block: Rc<Block>) -> Self {
        Self {
            statement,
            global_block,
        }
    }

    pub fn evaluate(&self) -> Result<(), CompilerError> {
        Evaluator::_evaluate(self.statement, Rc::clone(&self.global_block))?;
        Ok(())
    }

    fn _evaluate(
        statement: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<Variable, CompilerError> {
        match statement.as_ref() {
            AbstractSyntaxTree::Literal(literal) => Ok(literal.clone()),
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                evaluate_binary_expression(left, block, right, operator)
            }
            AbstractSyntaxTree::UnaryExpression(operator, expression) => {
                evaluate_unary_expression(operator, expression, block)
            }
            AbstractSyntaxTree::Identifier(name) => match block.get_symbol(name) {
                Some(value) => Ok(value.clone()),
                None => Err(CompilerError::UndefinedVariable(name.clone())),
            },
            AbstractSyntaxTree::AssignmentExpression(name, operator, expression) => {
                evaluate_assignment_expression(expression, block, operator, name)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                Evaluator::_evaluate(expression, block)
            }
            AbstractSyntaxTree::BlockStatement(_) => todo!(),
        }
    }
}

fn evaluate_block(block: Rc<Block>) -> Result<(), CompilerError> {
    for statement in block.statements.iter() {
        Evaluator::_evaluate(statement, Rc::clone(&block))?;
    }
    Ok(())
}

fn evaluate_assignment_expression(
    expression: &Box<AbstractSyntaxTree>,
    block: Rc<Block>,
    operator: &Operator,
    name: &String,
) -> Result<Variable, CompilerError> {
    let right_hand = Evaluator::_evaluate(expression, Rc::clone(&block))?;

    match operator {
        Operator::AssignmentOperator(assigmnent) => match assigmnent {
            Assingment::SimpleAssignment => {
                if let Some(old_variable) = block.get_symbol(name) {
                    if old_variable.is_mutable() {
                        block.add_symbol(name.to_string(), right_hand.clone().as_mutable());
                    } else {
                        return Err(CompilerError::ImmutableVariable(name.to_string()));
                    }
                } else {
                    block.add_symbol(name.to_string(), right_hand.clone());
                }
                Ok(right_hand)
            }
            assignment_operator => {
                if let Some(old_variable) = block.get_symbol(name) {
                    if old_variable.is_mutable() {
                        let result = assignment_operator.evaluate(old_variable, right_hand)?;
                        block.add_symbol(name.clone(), result.clone().as_mutable());
                        Ok(result)
                    } else {
                        Err(CompilerError::ImmutableVariable(name.clone()))
                    }
                } else {
                    Err(CompilerError::UndefinedVariable(name.clone()))
                }
            }
        },
        operator => Err(CompilerError::InvalidOperatorForBinaryOperation(
            operator.clone(),
        )),
    }
}

fn evaluate_unary_expression(
    operator: &Operator,
    expression: &Box<AbstractSyntaxTree>,
    block: Rc<Block>,
) -> Result<Variable, CompilerError> {
    match operator {
        ArithmeticOperator(operator) => match operator {
            Addition => Ok(Addition.evaluate_unary(Evaluator::_evaluate(expression, block)?)?),
            Subtraction => {
                Ok(Subtraction.evaluate_unary(Evaluator::_evaluate(expression, block)?)?)
            }
            operation => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::ArithmeticOperator(operation.clone()),
            )),
        },
        LogicalOperator(operator) => match operator {
            Not => Ok(Not.evaluate_unary(Evaluator::_evaluate(expression, block)?)?),
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::LogicalOperator(operator.clone()),
            )),
        },
        operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
            operator.clone(),
        )),
    }
}

fn evaluate_binary_expression(
    left: &Box<AbstractSyntaxTree>,
    block: Rc<Block>,
    right: &Box<AbstractSyntaxTree>,
    operator: &Operator,
) -> Result<Variable, CompilerError> {
    let left = Evaluator::_evaluate(left, Rc::clone(&block))?;
    let right = Evaluator::_evaluate(right, block)?;
    let result = match operator {
        ArithmeticOperator(_) | RelationalOperator(_) | LogicalOperator(_) => {
            operator.evaluate(left, right)?
        }
        operator => {
            return Err(CompilerError::InvalidOperatorForBinaryOperation(
                operator.clone(),
            ))
        }
    };
    Ok(result)
}
