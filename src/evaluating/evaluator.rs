use std::sync::mpsc::Receiver;
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

use super::global::Global;

pub(crate) struct Evaluator {
    statement_receiver: Receiver<AbstractSyntaxTree>,
    global: Global,
}

impl Evaluator {
    pub(crate) fn new(statement_receiver: Receiver<AbstractSyntaxTree>, global: Global) -> Self {
        Self {
            global,
            statement_receiver,
        }
    }

    pub(crate) fn evaluate(&self) {
        for statement in &self.statement_receiver {
            if let Err(err) = self.evaluate_statement(&statement, Arc::clone(&self.global.block)) {
                eprintln!("{err}");
            }
        }
    }

    fn evaluate_statement(
        &self,
        statement: &AbstractSyntaxTree,
        block: Arc<RwLock<Block>>,
    ) -> Result<Literal, CompilerError> {
        match statement {
            AbstractSyntaxTree::Literal(literal) => Ok(literal.clone()),
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                self.evaluate_binary_expression(left, block, right, operator)
            }
            AbstractSyntaxTree::UnaryExpression(operator, expression) => {
                self.evaluate_unary_expression(operator, expression, block)
            }
            AbstractSyntaxTree::Identifier(name) => {
                if let Some(value) = block.read().unwrap().get_symbol(name) {
                    Ok(value)
                } else if let Some(variable) = self.global.get_built_in_properties(name) {
                    Ok(variable)
                } else {
                    Err(CompilerError::UndefinedVariable(name.clone()))
                }
            }
            AbstractSyntaxTree::AssignmentExpression(name, operator, expression) => {
                self.evaluate_assignment_expression(name, operator, expression, block)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                self.evaluate_statement(expression, block)
            }
            AbstractSyntaxTree::BlockStatement(block_statement) => {
                self.evaluate_block(Arc::clone(block_statement))
            }
            AbstractSyntaxTree::IfStatement(condition, if_block_or_statement, else_statement) => {
                self.evaluate_if_statement(condition, if_block_or_statement, else_statement, block)
            }
            AbstractSyntaxTree::ElseStatement(if_or_block_statement) => {
                self.evaluate_statement(if_or_block_statement, block)
            }
            AbstractSyntaxTree::LoopStatement(condition, block_to_execute) => {
                self.evaluate_loop_statement(condition, block_to_execute, block)
            }
            AbstractSyntaxTree::CallStatement(name, arguements) => {
                self.evalute_call_statement(name.to_string(), arguements, block)
            }
            AbstractSyntaxTree::ReturnStatement(statement) => {
                if !block.read().unwrap().is_function {
                    return Err(CompilerError::ReturnOutsideFunction);
                }
                Ok(Literal::new(
                    DataType::Return(Box::new(self.evaluate_statement(statement, block)?)),
                    false,
                ))
            }
            AbstractSyntaxTree::BreakStatement(statement) => {
                if !block.read().unwrap().is_loop {
                    return Err(CompilerError::BreakOutsideLoop);
                }
                Ok(Literal::new(
                    DataType::Break(Box::new(self.evaluate_statement(statement, block)?)),
                    false,
                ))
            }
            AbstractSyntaxTree::SkipStatement(statement) => {
                if !block.read().unwrap().is_loop {
                    return Err(CompilerError::SkipOutsideLoop);
                }
                Ok(Literal::new(
                    DataType::Skip(Box::new(self.evaluate_statement(statement, block)?)),
                    false,
                ))
            }
        }
    }

    fn evalute_call_statement(
        &self,
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
            let evaluated_arguement = self.evaluate_statement(arguement, Arc::clone(&block))?;
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

                return self.evaluate_block(Arc::clone(&function.block));
            } else {
                return Err(CompilerError::NotAFunction(name));
            }
        }
        match self.global.get_built_in_function(&name) {
            Some(built_in_function) => built_in_function(evaluated_arguements),
            None => Err(CompilerError::UndefinedFunction(name)),
        }
    }

    fn evaluate_loop_statement(
        &self,
        condition_statement: &AbstractSyntaxTree,
        block_or_statement_to_execute: &AbstractSyntaxTree,
        block: Arc<RwLock<Block>>,
    ) -> Result<Literal, CompilerError> {
        let mut result = Literal::from(false);
        let mut skip_count = 0;
        while self
            .evaluate_statement(condition_statement, Arc::clone(&block))?
            .is_truthy()?
        {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }
            result = self.evaluate_statement(block_or_statement_to_execute, Arc::clone(&block))?;
            if let DataType::Break(value_to_return) = result.value {
                result = *value_to_return;
                break;
            }
            if let DataType::Skip(count) = result.value.clone() {
                if let DataType::Integer(count) = count.value {
                    skip_count = count;
                } else {
                    return Err(CompilerError::SkipCountTypeMisMatch(
                        count.value.to_string(),
                    ));
                }
            }
        }
        Ok(result)
    }

    fn evaluate_if_statement(
        &self,
        condition: &AbstractSyntaxTree,
        if_block_or_statement: &AbstractSyntaxTree,
        else_statement: &Option<Box<AbstractSyntaxTree>>,
        scope_block: Arc<RwLock<Block>>,
    ) -> Result<Literal, CompilerError> {
        let condition = self.evaluate_statement(condition, Arc::clone(&scope_block))?;
        if condition.is_truthy()? {
            self.evaluate_statement(if_block_or_statement, Arc::clone(&scope_block))
        } else if let Some(else_block) = else_statement {
            self.evaluate_statement(else_block, scope_block)
        } else {
            Ok(Literal::from(false))
        }
    }

    fn evaluate_block(&self, block: Arc<RwLock<Block>>) -> Result<Literal, CompilerError> {
        let mut result = Literal::from(false);
        for statement in block.read().unwrap().statements.iter() {
            result = self.evaluate_statement(statement, Arc::clone(&block))?;
            if let DataType::Return(statement) = result.value {
                result = *statement;
                break;
            }
            if let DataType::Break(_) = result.value {
                break;
            }
            if let DataType::Skip(_) = result.value {
                break;
            }
        }
        block.read().unwrap().clear_symbols();
        Ok(result)
    }

    fn evaluate_assignment_expression(
        &self,
        name: &str,
        operator: &Operator,
        expression: &AbstractSyntaxTree,
        block: Arc<RwLock<Block>>,
    ) -> Result<Literal, CompilerError> {
        let right_hand = self.evaluate_statement(expression, Arc::clone(&block))?;
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
        &self,
        operator: &Operator,
        expression: &AbstractSyntaxTree,
        block: Arc<RwLock<Block>>,
    ) -> Result<Literal, CompilerError> {
        match operator {
            Arithmetic(operator) => match operator {
                Addition => {
                    Ok(Addition.evaluate_unary(self.evaluate_statement(expression, block)?)?)
                }
                Subtraction => {
                    Ok(Subtraction.evaluate_unary(self.evaluate_statement(expression, block)?)?)
                }
                operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                    Operator::Arithmetic(*operator),
                )),
            },
            Logical(operator) => match operator {
                Not => Ok(Not.evaluate_unary(self.evaluate_statement(expression, block)?)?),
                operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                    Operator::Logical(*operator),
                )),
            },
            operator => Err(CompilerError::InvalidOperatorForUnaryOperation(*operator)),
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: &AbstractSyntaxTree,
        block: Arc<RwLock<Block>>,
        right: &AbstractSyntaxTree,
        operator: &Operator,
    ) -> Result<Literal, CompilerError> {
        let left = self.evaluate_statement(left, Arc::clone(&block))?;
        let right = self.evaluate_statement(right, block)?;
        let result = match operator {
            Arithmetic(_) | Relational(_) | Logical(_) => operator.evaluate(left, right)?,
            operator => return Err(CompilerError::InvalidOperatorForBinaryOperation(*operator)),
        };
        Ok(result)
    }
}
