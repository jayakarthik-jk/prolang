use crate::expressions::SyntaxExpression;
use crate::symbols::Symbol::*;
use crate::{
    errors::CompilerError,
    lexer::{Lexer, TokenKind},
};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let lexer = Lexer::new(source);
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<SyntaxExpression, CompilerError> {
        self.lexer.prepare()?;
        self.parse_expression(0)
    }

    fn parse_expression(&self, parent_precedence: u8) -> Result<SyntaxExpression, CompilerError> {

        let mut left = if let Ok(operator_token) = self.lexer.get_current_token() 
        && let TokenKind::OperatorToken(operator) = operator_token.kind
        && operator.get_unery_precedence() >= parent_precedence {
            self.lexer.advance();
            let expression = self.parse_expression(operator.get_unery_precedence())?;
            SyntaxExpression::UnaryExpression(operator_token, Box::new(expression))
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
            let right = self.parse_expression(precedence)?;
            left = SyntaxExpression::BinaryExpression(Box::new(left), operator_token, Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&self) -> Result<SyntaxExpression, CompilerError> {
        match self.lexer.get_current_token_and_advance() {
            Ok(token) => match &token.kind {
                TokenKind::LiteralToken(_) => Ok(SyntaxExpression::LiteralExpression(token)),
                TokenKind::SymbolToken(symbol) => match symbol {
                    OpenParanthesis => {
                        // let expression = self.parse_expression()?;
                        let expression = self.parse_expression(0)?;
                        match &self.lexer.get_current_token_and_advance()?.kind {
                            TokenKind::SymbolToken(CloseParanthesis) => {
                                Ok(expression)
                            }
                            token_kind => Err(CompilerError::UnExpectedTokenError(
                                TokenKind::SymbolToken(CloseParanthesis),
                                token_kind.clone(),
                            )),
                        }
                    }
                    CloseParanthesis => Err(CompilerError::UnExpectedTokenError(
                        TokenKind::SymbolToken(OpenParanthesis),
                        TokenKind::SymbolToken(CloseParanthesis),
                    )),
                },
                kind => Err(CompilerError::UnExpectedTokenError(
                    TokenKind::SymbolToken(OpenParanthesis),
                    kind.clone(),
                )),
            },
            Err(error) => Err(error),
        }
    }
}
