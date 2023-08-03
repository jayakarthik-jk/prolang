use std::cell::RefCell;
use std::rc::Rc;

use crate::common::datatypes::DataType::Boolean;
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
    statement: &'a AbstractSyntaxTree,
    global_block: Rc<RefCell<Block>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(statement: &'a AbstractSyntaxTree, global_block: Rc<RefCell<Block>>) -> Self {
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
        statement: &AbstractSyntaxTree,
        block: Rc<RefCell<Block>>,
    ) -> Result<Variable, CompilerError> {
        match statement {
            AbstractSyntaxTree::Literal(literal) => Ok(literal.clone()),
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                evaluate_binary_expression(left, block, right, operator)
            }
            AbstractSyntaxTree::UnaryExpression(operator, expression) => {
                evaluate_unary_expression(operator, expression, block)
            }
            AbstractSyntaxTree::Identifier(name) => match block.borrow().get_symbol(name) {
                Some(value) => Ok(value),
                None => Err(CompilerError::UndefinedVariable(name.clone())),
            },
            AbstractSyntaxTree::AssignmentExpression(name, operator, expression) => {
                evaluate_assignment_expression(name, operator, expression, block)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                Evaluator::_evaluate(expression, block)
            }
            AbstractSyntaxTree::BlockStatement(b) => evaluate_block(Rc::clone(b)),
        }
    }
}

fn evaluate_block(block: Rc<RefCell<Block>>) -> Result<Variable, CompilerError> {
    let mut result = Variable::new(Boolean(false));
    for statement in block.borrow().statements.iter() {
        result = Evaluator::_evaluate(statement, Rc::clone(&block))?;
    }
    Ok(result)
}

fn evaluate_assignment_expression(
    name: &String,
    operator: &Operator,
    expression: &AbstractSyntaxTree,
    block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    let right_hand = Evaluator::_evaluate(expression, Rc::clone(&block))?;
    let block = block.borrow();
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
        operator => Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
    }
}

fn evaluate_unary_expression(
    operator: &Operator,
    expression: &AbstractSyntaxTree,
    block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    match operator {
        ArithmeticOperator(operator) => match operator {
            Addition => Ok(Addition.evaluate_unary(Evaluator::_evaluate(expression, block)?)?),
            Subtraction => {
                Ok(Subtraction.evaluate_unary(Evaluator::_evaluate(expression, block)?)?)
            }
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::ArithmeticOperator(*operator),
            )),
        },
        LogicalOperator(operator) => match operator {
            Not => Ok(Not.evaluate_unary(Evaluator::_evaluate(expression, block)?)?),
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::LogicalOperator(*operator),
            )),
        },
        operator => Err(CompilerError::InvalidOperatorForUnaryOperation(*operator)),
    }
}

fn evaluate_binary_expression(
    left: &AbstractSyntaxTree,
    block: Rc<RefCell<Block>>,
    right: &AbstractSyntaxTree,
    operator: &Operator,
) -> Result<Variable, CompilerError> {
    let left = Evaluator::_evaluate(left, Rc::clone(&block))?;
    let right = Evaluator::_evaluate(right, block)?;
    let result = match operator {
        ArithmeticOperator(_) | RelationalOperator(_) | LogicalOperator(_) => {
            operator.evaluate(left, right)?
        }
        operator => return Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
    };
    Ok(result)
}
