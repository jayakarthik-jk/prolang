use crate::datatypes::DataType::Boolean;
use crate::lexer::TokenKind::LiteralToken;
use crate::lexer::TokenKind::OperatorToken;
use crate::operators::logical::Logical;
use crate::operators::Operator::LogicalOperator;
use crate::{errors::CompilerError, lexer::Token};
#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
}

impl Keyword {
    pub fn parse(word: String, position: usize) -> Result<Token, CompilerError> {
        let token = match word.as_str() {
            "true" => Token::new(LiteralToken(Boolean(true)), position),
            "false" => Token::new(LiteralToken(Boolean(false)), position),
            "and" => Token::new(OperatorToken(LogicalOperator(Logical::And)), position),
            "or" => Token::new(OperatorToken(LogicalOperator(Logical::Or)), position),
            "not" => Token::new(OperatorToken(LogicalOperator(Logical::Not)), position),
            "xor" => Token::new(OperatorToken(LogicalOperator(Logical::Xor)), position),
            _ => return Err(CompilerError::UnknownKeywordError(word)),
        };
        Ok(token)
    }
}
