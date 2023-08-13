use std::cell::RefCell;
use std::rc::Rc;

use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical::Not;
use crate::common::operators::Operator;
use crate::common::operators::Operator::*;
use crate::lexing::symbols::Symbol;
use crate::parsing::ast::AbstractSyntaxTree;
use crate::parsing::block::Block;
use crate::parsing::seperated_statements::SeperatedStatements;

use super::global::GlobalProperties;

pub(crate) struct Evaluator<'a> {
    statement: &'a AbstractSyntaxTree,
    global_block: Rc<RefCell<Block>>,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn new(statement: &'a AbstractSyntaxTree, global_block: Rc<RefCell<Block>>) -> Self {
        Self {
            statement,
            global_block,
        }
    }

    pub(crate) fn evaluate(&self) -> Result<Variable, CompilerError> {
        evaluate(self.statement, Rc::clone(&self.global_block))
    }
}

fn evaluate(
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
        AbstractSyntaxTree::Identifier(name) => {
            if let Some(value) = block.borrow().get_symbol(name) {
                Ok(value)
            } else {
                if let Some(variable) = GlobalProperties::get_built_in_properties(name) {
                    Ok(variable)
                } else {
                    Err(CompilerError::UndefinedVariable(name.clone()))
                }
            }
        }
        AbstractSyntaxTree::AssignmentExpression(name, operator, expression) => {
            evaluate_assignment_expression(name, operator, expression, block)
        }
        AbstractSyntaxTree::ParenthesizedExpression(expression) => evaluate(expression, block),
        AbstractSyntaxTree::BlockStatement(block) => evaluate_block(Rc::clone(block)),
        AbstractSyntaxTree::IfStatement(condition, if_block_or_statement, else_statement) => {
            evaluate_if_statement(condition, if_block_or_statement, else_statement, block)
        }
        AbstractSyntaxTree::ElseStatement(if_or_block_statement) => {
            evaluate(if_or_block_statement, block)
        }
        AbstractSyntaxTree::LoopStatement(condition, block_to_execute) => {
            evaluate_loop_statement(condition, block_to_execute, block)
        }
        AbstractSyntaxTree::CallStatement(name, arguements) => {
            evalute_call_statement(name.to_string(), arguements, block)
        }
    }
}

fn evalute_call_statement(
    name: String,
    arguements: &SeperatedStatements,
    block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    if Symbol::OpenParanthesis != arguements.enclosed_with {
        return Err(CompilerError::InvalidEncloser(arguements.enclosed_with));
    }
    if Symbol::Comma != arguements.seperated_with {
        return Err(CompilerError::InvalidSeperator(arguements.seperated_with));
    }
    let mut evaluated_arguements: Vec<Variable> = vec![];
    for arguement in arguements.iter() {
        let evaluated_arguement = evaluate(arguement, Rc::clone(&block))?;
        evaluated_arguements.push(evaluated_arguement);
    }

    match GlobalProperties::get_built_in_function(&name) {
        Some(built_in_function) => built_in_function(evaluated_arguements),
        None => Err(CompilerError::UndefinedFunction(name)),
    }
}

fn evaluate_loop_statement(
    condition_statement: &AbstractSyntaxTree,
    block_or_statement_to_execute: &AbstractSyntaxTree,
    block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    let mut condition = evaluate(condition_statement, Rc::clone(&block))?;
    let mut result = Variable::from(false);
    while condition.is_truthy() {
        result = evaluate(block_or_statement_to_execute, Rc::clone(&block))?;
        condition = evaluate(condition_statement, Rc::clone(&block))?;
    }
    Ok(result)
}

fn evaluate_if_statement(
    condition: &AbstractSyntaxTree,
    if_block_or_statement: &AbstractSyntaxTree,
    else_statement: &Option<Box<AbstractSyntaxTree>>,
    scope_block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    let condition = evaluate(condition, Rc::clone(&scope_block))?;
    if condition.is_truthy() {
        evaluate(if_block_or_statement, Rc::clone(&scope_block))
    } else if let Some(else_block) = else_statement {
        evaluate(else_block, scope_block)
    } else {
        Ok(Variable::from(false))
    }
}

fn evaluate_block(block: Rc<RefCell<Block>>) -> Result<Variable, CompilerError> {
    let mut result = Variable::from(false);
    for statement in block.borrow().statements.iter() {
        result = evaluate(statement, Rc::clone(&block))?;
    }
    block.borrow().clear_symbols();
    Ok(result)
}

fn evaluate_assignment_expression(
    name: &String,
    operator: &Operator,
    expression: &AbstractSyntaxTree,
    block: Rc<RefCell<Block>>,
) -> Result<Variable, CompilerError> {
    let right_hand = evaluate(expression, Rc::clone(&block))?;
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
            Addition => Ok(Addition.evaluate_unary(evaluate(expression, block)?)?),
            Subtraction => Ok(Subtraction.evaluate_unary(evaluate(expression, block)?)?),
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::ArithmeticOperator(*operator),
            )),
        },
        LogicalOperator(operator) => match operator {
            Not => Ok(Not.evaluate_unary(evaluate(expression, block)?)?),
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
    let left = evaluate(left, Rc::clone(&block))?;
    let right = evaluate(right, block)?;
    let result = match operator {
        ArithmeticOperator(_) | RelationalOperator(_) | LogicalOperator(_) => {
            operator.evaluate(left, right)?
        }
        operator => return Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
    };
    Ok(result)
}
