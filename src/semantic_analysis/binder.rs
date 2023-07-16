use std::cell::RefCell;
use std::rc::Rc;

use crate::common::datatypes::DataType;
use crate::common::errors::CompilerError;
use crate::common::symbol_table::SymbolTable;
use crate::lexical_analysis::lexer::Token;
use crate::lexical_analysis::lexer::TokenKind::IdentifierToken;
use crate::semantic_analysis::semantic_tree::SemanticTree;
use crate::syntax_analysis::ast::AbstractSyntaxTree;

pub struct Binder {
    root: Box<AbstractSyntaxTree>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
    pub display_process: bool,
}

impl Binder {
    pub fn new(root: AbstractSyntaxTree, symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            root: Box::new(root),
            symbol_table,
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
            AbstractSyntaxTree::IdentifierExpression(expression) => {
                self.bind_identifier_expression(expression)
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

    fn bind_literal_expression(&self, token: &Rc<Token>) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding literal: {}", token);
        }
        Ok(SemanticTree::LiteralExpression(Rc::clone(token)))
    }

    fn bind_identifier_expression(
        &self,
        identifier_token: &Rc<Token>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding identifier: {}", identifier_token);
        }
        if let IdentifierToken(name) = &identifier_token.kind {
            let table = self.symbol_table.borrow();
            let variable = table.variables.get(name);
            if let Some(value) = variable
            && value != &DataType::InternalUndefined {
                Ok(SemanticTree::IdentifierExpression(identifier_token.clone()))
            } else {
                Err(CompilerError::UndefinedVariable(name.clone()))
            }
        } else {
            Err(CompilerError::InvalidTokenAsIdentifier(
                identifier_token.kind.clone(),
            ))
        }
    }
    fn bind_unary_expression(
        &self,
        operator_token: &Rc<Token>,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        if self.display_process {
            println!("binding unary: {} {}", operator_token, expression);
        }
        let expression = self.bind_expression(expression)?;

        Ok(SemanticTree::UnaryExpression(
            Rc::clone(operator_token),
            Box::new(expression),
        ))
    }

    fn bind_binary_expression(
        &self,
        left: &Box<AbstractSyntaxTree>,
        operator: &Rc<Token>,
        right: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        println!("binding binary: {} {} {}", left, operator, right);
        let left = self.bind_expression(left)?;
        let right = self.bind_expression(right)?;

        Ok(SemanticTree::BinaryExpression(
            Box::new(left),
            Rc::clone(operator),
            Box::new(right),
        ))
    }

    fn bind_assignment_expression(
        &self,
        identifier_expression: &Box<AbstractSyntaxTree>,
        operator: &Rc<Token>,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        println!(
            "binding assignment: {} {} {}",
            identifier_expression, operator, expression
        );
        let expression = self.bind_expression(expression)?;

        match identifier_expression.as_ref() {
            AbstractSyntaxTree::IdentifierExpression(identifier_token) => {
                match &identifier_token.as_ref().kind {
                    IdentifierToken(name) => Ok(SemanticTree::AssignmentExpression(
                        name.clone(),
                        Rc::clone(operator),
                        Box::new(expression),
                    )),
                    token => Err(CompilerError::InvalidTokenAsIdentifier(token.clone())),
                }
            }
            _ => Err(CompilerError::InvalidExpressionAssignment(
                operator.line,
                operator.column,
            )),
        }
    }

    fn bind_parenthesized_expression(
        &self,
        expression: &Box<AbstractSyntaxTree>,
    ) -> Result<SemanticTree, CompilerError> {
        self.bind_expression(expression)
    }
}
