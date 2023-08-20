use std::sync::{Arc, RwLock};

use crate::common::datatypes::DataType;
use crate::common::errors::CompilerError;
use crate::common::literal::Literal;
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
    global_block: Arc<RwLock<Block>>,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn new(statement: &'a AbstractSyntaxTree, global_block: Arc<RwLock<Block>>) -> Self {
        Self {
            statement,
            global_block,
        }
    }

    pub(crate) fn evaluate(&self) -> Result<Literal, CompilerError> {
        evaluate(self.statement, Arc::clone(&self.global_block))
    }
}

fn evaluate(
    statement: &AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    match statement {
        AbstractSyntaxTree::Literal(literal) => Ok(literal.clone()),
        AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
            evaluate_binary_expression(left, block, right, operator)
        }
        AbstractSyntaxTree::UnaryExpression(operator, expression) => {
            evaluate_unary_expression(operator, expression, block)
        }
        AbstractSyntaxTree::Identifier(name) => {
            if let Some(value) = block.read().unwrap().get_symbol(name) {
                Ok(value)
            } else if let Some(variable) = GlobalProperties::get_built_in_properties(name) {
                Ok(variable)
            } else {
                Err(CompilerError::UndefinedVariable(name.clone()))
            }
        }
        AbstractSyntaxTree::AssignmentExpression(name, operator, expression) => {
            evaluate_assignment_expression(name, operator, expression, block)
        }
        AbstractSyntaxTree::ParenthesizedExpression(expression) => evaluate(expression, block),
        AbstractSyntaxTree::BlockStatement(block_statement) => {
            evaluate_block(Arc::clone(block_statement))
        }
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
        AbstractSyntaxTree::ReturnStatement(statement) => Ok(Literal::new(
            DataType::Return(Box::new(evaluate(statement, block)?)),
            false,
        )),
    }
}

fn evalute_call_statement(
    name: String,
    arguements: &SeperatedStatements<Box<AbstractSyntaxTree>>,
    block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    if Symbol::OpenParanthesis != arguements.enclosed_with {
        return Err(CompilerError::InvalidEncloser(arguements.enclosed_with));
    }
    if Symbol::Comma != arguements.seperated_with {
        return Err(CompilerError::InvalidSeperator(arguements.seperated_with));
    }

    let mut evaluated_arguements: Vec<Literal> = vec![];
    for arguement in arguements.iter() {
        let evaluated_arguement = evaluate(arguement, Arc::clone(&block))?;
        evaluated_arguements.push(evaluated_arguement);
    }
    if let Some(function) = Arc::clone(&block).read().unwrap().get_symbol(&name) {
        if let DataType::Function(function) = function.value {
            if evaluated_arguements.len() != function.parameters.len() {
                return Err(CompilerError::ArgumentLengthMismatch(
                    name.clone(),
                    function.parameters.len(),
                    evaluated_arguements.len(),
                ));
            }
            for (index, parameter) in function.parameters.iter().enumerate() {
                if let AbstractSyntaxTree::Identifier(name) = parameter {
                    function
                        .block
                        .read()
                        .unwrap()
                        .add_symbol(name.clone(), evaluated_arguements[index].clone());
                }
            }

            return evaluate_block(Arc::clone(&function.block));
        } else {
            return Err(CompilerError::NotAFunction(name));
        }
    }
    match GlobalProperties::get_built_in_function(&name) {
        Some(built_in_function) => built_in_function(evaluated_arguements),
        None => Err(CompilerError::UndefinedFunction(name)),
    }
}

fn evaluate_loop_statement(
    condition_statement: &AbstractSyntaxTree,
    block_or_statement_to_execute: &AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    let mut condition = evaluate(condition_statement, Arc::clone(&block))?;
    let mut result = Literal::from(false);
    while condition.is_truthy() {
        result = evaluate(block_or_statement_to_execute, Arc::clone(&block))?;
        condition = evaluate(condition_statement, Arc::clone(&block))?;
    }
    Ok(result)
}

fn evaluate_if_statement(
    condition: &AbstractSyntaxTree,
    if_block_or_statement: &AbstractSyntaxTree,
    else_statement: &Option<Box<AbstractSyntaxTree>>,
    scope_block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    let condition = evaluate(condition, Arc::clone(&scope_block))?;
    if condition.is_truthy() {
        evaluate(if_block_or_statement, Arc::clone(&scope_block))
    } else if let Some(else_block) = else_statement {
        evaluate(else_block, scope_block)
    } else {
        Ok(Literal::from(false))
    }
}

fn evaluate_block(block: Arc<RwLock<Block>>) -> Result<Literal, CompilerError> {
    let mut result = Literal::from(false);
    for statement in block.read().unwrap().statements.iter() {
        result = evaluate(statement, Arc::clone(&block))?;
        if let DataType::Return(statement) = result.value {
            if block.read().unwrap().is_function {
                result = *statement;
                break;
            } else {
                return Err(CompilerError::ReturnOutsideFunction);
            }
        }
    }
    block.read().unwrap().clear_symbols();
    Ok(result)
}

fn evaluate_assignment_expression(
    name: &str,
    operator: &Operator,
    expression: &AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    let right_hand = evaluate(expression, Arc::clone(&block))?;
    let block = block.read().unwrap();
    match operator {
        Operator::Assignment(assigmnent) => match assigmnent {
            Assingment::Simple => {
                if let Some(old_variable) = block.get_symbol(name) {
                    if old_variable.is_mutable() {
                        block.add_symbol(name.to_string(), right_hand.clone().to_mutable());
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
                        block.add_symbol(name.to_string(), result.clone().to_mutable());
                        Ok(result)
                    } else {
                        Err(CompilerError::ImmutableVariable(name.to_string()))
                    }
                } else {
                    Err(CompilerError::UndefinedVariable(name.to_string()))
                }
            }
        },
        operator => Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
    }
}

fn evaluate_unary_expression(
    operator: &Operator,
    expression: &AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
) -> Result<Literal, CompilerError> {
    match operator {
        Arithmetic(operator) => match operator {
            Addition => Ok(Addition.evaluate_unary(evaluate(expression, block)?)?),
            Subtraction => Ok(Subtraction.evaluate_unary(evaluate(expression, block)?)?),
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::Arithmetic(*operator),
            )),
        },
        Logical(operator) => match operator {
            Not => Ok(Not.evaluate_unary(evaluate(expression, block)?)?),
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                Operator::Logical(*operator),
            )),
        },
        operator => Err(CompilerError::InvalidOperatorForUnaryOperation(*operator)),
    }
}

fn evaluate_binary_expression(
    left: &AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
    right: &AbstractSyntaxTree,
    operator: &Operator,
) -> Result<Literal, CompilerError> {
    let left = evaluate(left, Arc::clone(&block))?;
    let right = evaluate(right, block)?;
    let result = match operator {
        Arithmetic(_) | Relational(_) | Logical(_) => operator.evaluate(left, right)?,
        operator => return Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
    };
    Ok(result)
}
