use crate::common::datatypes::{Variable, DataType};
use crate::common::errors::CompilerError;
use crate::common::operators::Operator;
use crate::common::symbol_table::SymbolTable;
use crate::semantic_analysis::semantic_tree::SemanticTree;
use crate::syntax_analysis::ast::AbstractSyntaxTree;

pub struct Binder {
    root: Box<AbstractSyntaxTree>,
    pub display_process: bool,
}

impl Binder {
    pub fn new(root: AbstractSyntaxTree) -> Self {
        Self {
            root: Box::new(root),
            display_process: false,
        }
    }

    pub fn bind(&self) -> Result<SemanticTree, CompilerError> {
        self.bind_expression(&self.root)
    }

    fn bind_expression(
        &self,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding expression: {}", expression);
        }
        match expression.as_ref() {
            AbstractSyntaxTree::LiteralExpression(token) => self.bind_literal_expression(token),
            AbstractSyntaxTree::IdentifierExpression(name) => {
                self.bind_identifier_expression(name.to_string())
            }
            AbstractSyntaxTree::UnaryExpression(operator_token, expression) => {
                self.bind_unary_expression(operator_token, expression)
            }
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                self.bind_binary_expression(left, operator, right)
            }
            AbstractSyntaxTree::AssignmentExpression(
                identifier_expression,
                operator,
                expression,
            ) => self.bind_assignment_expression(identifier_expression, operator, expression),
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                self.bind_parenthesized_expression(expression)
            }
        }
    }

    fn bind_literal_expression(&self, variable: &Variable) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding literal: {}", variable);
        }
        Ok(SemanticTree::LiteralExpression(variable.clone()))
    }

    fn bind_identifier_expression(&self, name: String) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding identifier: {}", name);
        }

        if let Some(variable) = SymbolTable::get(&name) {
            if variable.value == DataType::InternalUndefined {
                Ok(SemanticTree::IdentifierExpression(name))
            } else {
                Err(CompilerError::UndefinedVariable(name))
            }
        } else {
            Err(CompilerError::UndefinedVariable(name))
        }
    }
    fn bind_unary_expression(
        &self,
        operator: &Operator,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding unary: {} {}", operator, expression);
        }
        let expression = self.bind_expression(expression)?;

        Ok(SemanticTree::UnaryExpression(
            operator.clone(),
            Box::new(expression),
        ))
    }

    fn bind_binary_expression(
        &self,
        left: &Box<AbstractSyntaxTree>,
        operator: &Operator,
        right: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding binary: {} {} {}", left, operator, right);
        }
        let left = self.bind_expression(left)?;
        let right = self.bind_expression(right)?;

        Ok(SemanticTree::BinaryExpression(
            Box::new(left),
            operator.clone(),
            Box::new(right),
        ))
    }

    fn bind_assignment_expression(
        &self,
        identifier_expression: &Box<AbstractSyntaxTree>,
        operator: &Operator,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!(
                "binding assignment: {} {} {}",
                identifier_expression, operator, expression
            );
        }
        let expression = self.bind_expression(expression)?;

        match identifier_expression.as_ref() {
            AbstractSyntaxTree::IdentifierExpression(name) => {
                Ok(SemanticTree::AssignmentExpression(
                    name.clone(),
                    operator.clone(),
                    Box::new(expression),
                ))
            }
            _ => Err(CompilerError::InvalidExpressionAssignment),
        }
    }

    fn bind_parenthesized_expression(
        &self,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        self.bind_expression(expression)
    }
}
