use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical::Not;
use crate::common::operators::Operator::{
    self, ArithmeticOperator, LogicalOperator, RelationalOperator,
};
use crate::common::symbol_table::SymbolTable;
use crate::semantic_analysis::semantic_tree::SemanticTree;

pub struct Evaluator {
    root: Box<SemanticTree>,
}

impl Evaluator {
    pub fn new(root: SemanticTree) -> Self {
        Self {
            root: Box::new(root),
        }
    }

    pub fn evaluate(&self) -> Result<Variable, CompilerError> {
        self._evaluate(&self.root)
    }

    fn _evaluate(&self, root: &Box<SemanticTree>) -> Result<Variable, CompilerError> {
        match root.as_ref() {
            SemanticTree::LiteralExpression(literal) => Ok(literal.clone()),
            SemanticTree::BinaryExpression(left, operator, right) => {
                let left = self._evaluate(left)?;
                let right = self._evaluate(right)?;
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
            SemanticTree::UnaryExpression(operator, expression) => match operator {
                ArithmeticOperator(operator) => match operator {
                    Addition => Ok(Addition.evaluate_unary(self._evaluate(expression)?)?),
                    Subtraction => Ok(Subtraction.evaluate_unary(self._evaluate(expression)?)?),
                    operation => Err(CompilerError::InvalidOperatorForUnaryOperation(
                        Operator::ArithmeticOperator(operation.clone()),
                    )),
                },
                LogicalOperator(operator) => match operator {
                    Not => Ok(Not.evaluate_unary(self._evaluate(expression)?)?),
                    operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                        Operator::LogicalOperator(operator.clone()),
                    )),
                },
                operator => Err(CompilerError::InvalidOperatorForUnaryOperation(
                    operator.clone(),
                )),
            },
            SemanticTree::IdentifierExpression(name) => {
                match SymbolTable::get(name) {
                    Some(value) => Ok(value.clone()),
                    None => Err(CompilerError::UndefinedVariable(name.clone())),
                }
            }
            SemanticTree::AssignmentExpression(name, operator, expression) => {
                let right_hand = self._evaluate(expression)?;

                match operator {
                    Operator::AssignmentOperator(assigmnent) => match assigmnent {
                        Assingment::SimpleAssignment => {
                            if SymbolTable::contains(name) && SymbolTable::get(name).unwrap().is_mutable() {
                                SymbolTable::add(name.to_string(), Variable::new_mutable(right_hand.value.clone()));
                            } else if SymbolTable::contains(name) {
                                return Err(CompilerError::ImmutableVariable(name.clone()));
                            } else {
                                SymbolTable::add(name.to_string(), right_hand.clone());
                            }
                            Ok(right_hand)
                        }
                        assignment_operator => {
                            if let Some(variable) = SymbolTable::get(name) {
                                let result = assignment_operator
                                    .evaluate(variable.clone(), right_hand.clone())?;
                                SymbolTable::add(name.clone(), result.clone());
                                Ok(result)
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
        }
    }
}
