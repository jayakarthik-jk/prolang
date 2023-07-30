use std::rc::Rc;

use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::common::operators::Operator;
use crate::syntax_analysis::ast::AbstractSyntaxTree;
use crate::syntax_analysis::block::Block;

pub struct Binder<'a> {
    statement: &'a Box<AbstractSyntaxTree>,
    global_block: Rc<Block>,
    pub display_process: bool,
}

impl<'a> Binder<'a> {
    pub fn new(statement: &'a Box<AbstractSyntaxTree>, global_block: Rc<Block>) -> Self {
        Self {
            display_process: false,
            statement,
            global_block,
        }
    }

    pub fn bind(&self) -> Result<Box<AbstractSyntaxTree>, CompilerError> {
        let bound_statement = self.bind_statement(self.statement, Rc::clone(&self.global_block))?;
        Ok(Box::new(bound_statement))
    }

    fn bind_block(&self, block: Rc<Block>) -> Result<Rc<Block>, CompilerError> {
        let mut bound_block = Block::new();
        let mut statements = Vec::new();
        for statement in block.statements.iter() {
            let statement = self.bind_statement(statement, Rc::clone(&block))?;
            statements.push(Box::new(statement));
        }
        bound_block.statements = statements;
        Ok(Rc::new(bound_block))
    }

    fn bind_statement(
        &self,
        expression: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding expression: {}", expression);
        }
        match expression.as_ref() {
            AbstractSyntaxTree::Literal(token) => self.bind_literal_statement(token, block),
            AbstractSyntaxTree::Identifier(name) => {
                self.bind_identifier_statement(name.to_string(), block)
            }
            AbstractSyntaxTree::UnaryExpression(operator_token, expression) => {
                self.bind_unary_expression(operator_token, expression, block)
            }
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                self.bind_binary_expression(left, operator, right, block)
            }
            AbstractSyntaxTree::AssignmentExpression(
                identifier_expression,
                operator,
                expression,
            ) => {
                self.bind_assignment_expression(identifier_expression, operator, expression, block)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                self.bind_parenthesized_expression(expression, block)
            }
            AbstractSyntaxTree::BlockStatement(block) => Ok(AbstractSyntaxTree::BlockStatement(
                self.bind_block(Rc::clone(block))?,
            )),
        }
    }

    fn bind_literal_statement(
        &self,
        variable: &Variable,
        _: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding literal: {}", variable);
        }
        Ok(AbstractSyntaxTree::Literal(variable.clone()))
    }

    fn bind_identifier_statement(
        &self,
        name: String,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding identifier: {}", name);
        }
        if block.contains_symbol(&name) {
            Ok(AbstractSyntaxTree::Identifier(name))
        } else {
            Err(CompilerError::UndefinedVariable(name))
        }
    }
    fn bind_unary_expression(
        &self,
        operator: &Operator,
        expression: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding unary: {} {}", operator, expression);
        }
        let expression = self.bind_statement(expression, block)?;

        Ok(AbstractSyntaxTree::UnaryExpression(
            *operator,
            Box::new(expression),
        ))
    }

    fn bind_binary_expression(
        &self,
        left: &Box<AbstractSyntaxTree>,
        operator: &Operator,
        right: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding binary: {} {} {}", left, operator, right);
        }
        let left = self.bind_statement(left, Rc::clone(&block))?;
        let right = self.bind_statement(right, Rc::clone(&block))?;

        Ok(AbstractSyntaxTree::BinaryExpression(
            Box::new(left),
            *operator,
            Box::new(right),
        ))
    }

    fn bind_assignment_expression(
        &self,
        name: &String,
        operator: &Operator,
        expression: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.display_process {
            println!("binding assignment: {} {} {}", name, operator, expression);
        }
        let expression = self.bind_statement(expression, block)?;

        Ok(AbstractSyntaxTree::AssignmentExpression(
            name.clone(),
            *operator,
            Box::new(expression),
        ))
    }

    fn bind_parenthesized_expression(
        &self,
        expression: &Box<AbstractSyntaxTree>,
        block: Rc<Block>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.bind_statement(expression, block)
    }
}
