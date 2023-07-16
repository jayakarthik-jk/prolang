use std::cell::RefCell;
use std::rc::Rc;

use crate::common::datatypes::DataType;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical::Not;
use crate::common::operators::Operator::{
    self, ArithmeticOperator, LogicalOperator, RelationalOperator,
};
use crate::common::symbol_table::SymbolTable;
use crate::lexical_analysis::lexer::TokenKind;
use crate::semantic_analysis::semantic_tree::SemanticTree;

pub struct Evaluator {
    root: Box<SemanticTree>,
    symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Evaluator {
    pub fn new(root: SemanticTree, symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            root: Box::new(root),
            symbol_table,
        }
    }

    pub fn evaluate(&self) -> Result<DataType, CompilerError> {
        self._evaluate(&self.root)
    }

    fn _evaluate(&self, root: &Box<SemanticTree>) -> Result<DataType, CompilerError> {
        match root.as_ref() {
            SemanticTree::LiteralExpression(literal) => {
                if let TokenKind::LiteralToken(literal) = &literal.kind {
                    Ok(literal.clone())
                } else {
                    Err(CompilerError::InvalidTokenAsLiteral(literal.kind.clone()))
                }
            }
            SemanticTree::BinaryExpression(left, operator, right) => {
                let left = self._evaluate(left)?;
                let right = self._evaluate(right)?;
                match &operator.kind {
                    TokenKind::OperatorToken(operator) => {
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
                    kind => Err(CompilerError::InvalidTokenAsOperator(kind.clone())),
                }
            }
            SemanticTree::UnaryExpression(operator_token, expression) => {
                match &operator_token.kind {
                    TokenKind::OperatorToken(operator) => match operator {
                        ArithmeticOperator(operator) => match operator {
                            Addition => Ok(Addition.evaluate_unary(self._evaluate(expression)?)),
                            Subtraction => {
                                Ok(Subtraction.evaluate_unary(self._evaluate(expression)?))
                            }
                            operation => Err(CompilerError::INvalidOperatorForUnaryOperation(
                                Operator::ArithmeticOperator(operation.clone()),
                            )),
                        },
                        LogicalOperator(operator) => match operator {
                            Not => Ok(Not.evaluate_unary(self._evaluate(expression)?)),
                            operator => Err(CompilerError::INvalidOperatorForUnaryOperation(
                                Operator::LogicalOperator(operator.clone()),
                            )),
                        },
                        operator => Err(CompilerError::INvalidOperatorForUnaryOperation(
                            operator.clone(),
                        )),
                    },
                    kind => Err(CompilerError::InvalidTokenAsUnaryOperator(kind.clone())),
                }
            }
            SemanticTree::IdentifierExpression(token) => match &token.kind {
                TokenKind::IdentifierToken(name) => {
                    let symbol_table = self.symbol_table.borrow();
                    match symbol_table.variables.get(name) {
                        Some(value) => Ok(value.clone()),
                        None => Err(CompilerError::UndefinedVariable(name.clone())),
                    }
                }
                _ => Err(CompilerError::InvalidTokenAsIdentifier(token.kind.clone())),
            },
            SemanticTree::AssignmentExpression(name, operator, expression) => {
                let value = self._evaluate(expression)?;
                let mut symbol_table = self.symbol_table.borrow_mut();
                match &operator.kind {
                    TokenKind::OperatorToken(operator) => match operator {
                        Operator::AssingmentOperator(assigmnent) => match assigmnent {
                            Assingment::SimpleAssignment => {
                                symbol_table
                                    .variables
                                    .insert(name.to_string(), value.clone());
                                Ok(value)
                            }
                            assignment_operator => {
                                if let Some(variable) = symbol_table.variables.get(name) {
                                    let result = assignment_operator
                                        .evaluate(variable.clone(), value.clone())?;
                                    symbol_table.variables.insert(name.clone(), result.clone());
                                    Ok(result)
                                } else {
                                    Err(CompilerError::UndefinedVariable(name.clone()))
                                }
                            }
                        },
                        operator => Err(CompilerError::InvalidOperatorForBinaryOperation(
                            operator.clone(),
                        )),
                    },
                    kind => Err(CompilerError::InvalidTokenAsOperator(kind.clone())),
                }
            }
        }
    }
}
