use crate::symbols::Symbol::*;
use crate::{
    errors::LexerError,
    lexer::{Lexer, Token, TokenKind},
};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let lexer = Lexer::new(source);
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Expression, LexerError> {
        self.lexer.prepare()?;
        // self.parse_expression()
        self.parse_expression(0)
    }

    fn parse_expression(&self, parent_precedence: u8) -> Result<Expression, LexerError> {
        let mut left = self.parse_factor()?;

        while let Ok(operator_token) = self.lexer.get_current_token()
            && let TokenKind::OperatorToken(operator) = operator_token.kind {
            let precedence = operator.get_precedence();
            if precedence <= parent_precedence {
                break;
            }
            self.lexer.advance();
            let right = self.parse_expression(precedence)?;
            left = Expression::BinaryExpression(Box::new(left), operator_token, Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&self) -> Result<Expression, LexerError> {
        match self.lexer.get_current_token_and_advance() {
            Ok(token) => match token.kind {
                TokenKind::LiteralToken(_) => Ok(Expression::LiteralExpression(token)),
                TokenKind::SymbolToken(symbol) => match symbol {
                    OpenParanthesis => {
                        // let expression = self.parse_expression()?;
                        let expression = self.parse_expression(0)?;
                        match self.lexer.get_current_token_and_advance()?.kind {
                            TokenKind::SymbolToken(CloseParanthesis) => {
                                Ok(Expression::ParanthesizedExpression(Box::new(expression)))
                            }
                            token_kind => Err(LexerError::UnExpectedToken(
                                TokenKind::SymbolToken(CloseParanthesis),
                                token_kind,
                            )),
                        }
                    }
                    CloseParanthesis => Err(LexerError::UnExpectedToken(
                        TokenKind::SymbolToken(OpenParanthesis),
                        TokenKind::SymbolToken(CloseParanthesis),
                    )),
                },
                tokenkind => Err(LexerError::UnExpectedToken(
                    TokenKind::SymbolToken(OpenParanthesis),
                    tokenkind,
                )),
            },
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    LiteralExpression(&'a Token),
    BinaryExpression(Box<Expression<'a>>, &'a Token, Box<Expression<'a>>),
    ParanthesizedExpression(Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn print(node: &Expression, intent: String) {
        match node {
            Expression::LiteralExpression(number) => {
                println!("{intent}{:?}", number.kind);
            }
            Expression::BinaryExpression(left, operator, right) => {
                println!("{intent}{:?}", operator.kind);
                Expression::print(left, format!("{intent}    "));
                Expression::print(right, format!("{intent}    "));
            }
            Expression::ParanthesizedExpression(expression) => {
                Expression::print(&expression, format!("{intent}    "));
            }
        }
    }
}
