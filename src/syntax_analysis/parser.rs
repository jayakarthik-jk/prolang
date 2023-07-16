use std::cell::RefCell;
use std::rc::Rc;

use crate::common::operators::Operator::*;
use crate::lexical_analysis::lexer::{Lexer, TokenKind, Token};
use crate::syntax_analysis::ast::AbstractSyntaxTree;
use crate::lexical_analysis::symbols::Symbol::*;
use crate::common::errors::CompilerError;
pub struct Parser {
    lexer: Lexer,
    pub diagnostics: RefCell<Vec<CompilerError>>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer, diagnostics: RefCell::new(Vec::new()) }
    }

    pub fn parse(&mut self) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.lexer.get_token_count() == 0 {
            self.lexer.lex()?;
        }
        self.parse_expression()
    }

    pub fn parse_expression(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        
        let identifier_token = self.lexer.get_current_token()?;
        if let TokenKind::IdentifierToken(_) = &identifier_token.kind
        && let Ok(operator_token) = self.lexer.peek(1) 
        && let TokenKind::OperatorToken(operator) = operator_token.kind 
        && let AssingmentOperator(_) = operator {
            self.lexer.advance();
            self.lexer.advance();
            let expression = self.parse_assignment_expression()?;
            Ok(AbstractSyntaxTree::AssignmentExpression(Box::new(AbstractSyntaxTree::IdentifierExpression(identifier_token)), operator_token, Box::new(expression)))
        } else {
            let expression = self.parse_arithmetic_expression(0)?;
            Ok(expression)
        }
    }
    fn parse_arithmetic_expression(&self, parent_precedence: u8) -> Result<AbstractSyntaxTree, CompilerError> {

        let mut left = if let Ok(operator_token) = self.lexer.get_current_token() 
        && let TokenKind::OperatorToken(operator) = operator_token.kind
        && operator.get_unery_precedence() >= parent_precedence {
            self.lexer.advance();
            let expression = self.parse_arithmetic_expression(operator.get_unery_precedence())?;
            AbstractSyntaxTree::UnaryExpression(operator_token, Box::new(expression))
        } else {
            self.parse_factor()?
        };

        while let Ok(operator_token) = self.lexer.get_current_token()
            && let TokenKind::OperatorToken(operator) = operator_token.kind {
            let precedence = operator.get_binary_precedence();
            if precedence <= parent_precedence {
                break;
            }
            self.lexer.advance();
            let right = self.parse_arithmetic_expression(precedence)?;
            left = AbstractSyntaxTree::BinaryExpression(Box::new(left), operator_token, Box::new(right));
        }
        Ok(left)
    }

    fn expect(&self, expected: TokenKind, line: usize, column: usize) -> Rc<Token> {
        match self.lexer.get_current_token_and_advance() {
            Ok(token) => {
                if token.kind == expected {
                    token
                } else {
                    let mut diagnostics = self.diagnostics.borrow_mut();
                    diagnostics.push(CompilerError::UnexpectedTokenWithExpected(expected, token.kind.clone(), line, column));
                    self.lexer.generate_factory_token(line, column)
                }
            }
            Err(error) => {
                let mut diagnostics: std::cell::RefMut<'_, Vec<CompilerError>> = self.diagnostics.borrow_mut();
                diagnostics.push(error);
                self.lexer.generate_factory_token(line, column)
            },
        }
    }

    fn parse_factor(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        match self.lexer.get_current_token_and_advance() {
            Ok(token) => match &token.kind {
                TokenKind::LiteralToken(_) => Ok(AbstractSyntaxTree::LiteralExpression(token)),
                TokenKind::SymbolToken(symbol) => match symbol {
                    OpenParanthesis => {
                        let expression = self.parse_expression()?;
                        self.expect(TokenKind::SymbolToken(CloseParanthesis), token.line, token.column);
                        Ok(AbstractSyntaxTree::ParenthesizedExpression(Box::new(expression)))
                    }
                    CloseParanthesis => {
                        println!("line: {} column: {}", token.line, token.column);
                        Err(CompilerError::UnexpectedToken(
                            TokenKind::SymbolToken(CloseParanthesis),
                            token.line,
                            token.column
                        ))
                    },
                },
                TokenKind::IdentifierToken(_) => Ok(AbstractSyntaxTree::IdentifierExpression(token)),
                kind => Err(CompilerError::UnexpectedToken(kind.clone(), token.line, token.column)),
            },
            Err(error) => Err(error),
        }
    }
}
