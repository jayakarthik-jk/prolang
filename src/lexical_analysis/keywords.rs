use std::fmt::Display;

use crate::common::datatypes::DataType::Boolean;
use crate::common::errors::CompilerError;
use crate::common::operators::logical::Logical;
use crate::common::operators::Operator::LogicalOperator;
use crate::lexical_analysis::lexer::Token;
use crate::lexical_analysis::lexer::TokenKind::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
}

impl Keyword {
    pub fn parse(word: String, line: usize, column: usize) -> Result<Token, CompilerError> {
        let token = match word.as_str() {
            "true" => Token::new(LiteralToken(Boolean(true)), line, column),
            "false" => Token::new(LiteralToken(Boolean(false)), line, column),
            "and" => Token::new(OperatorToken(LogicalOperator(Logical::And)), line, column),
            "or" => Token::new(OperatorToken(LogicalOperator(Logical::Or)), line, column),
            "not" => Token::new(OperatorToken(LogicalOperator(Logical::Not)), line, column),
            "xor" => Token::new(OperatorToken(LogicalOperator(Logical::Xor)), line, column),
            identifier => Token::new(IdentifierToken(identifier.to_string()), line, column),
        };
        Ok(token)
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Keyword::True => "true",
            Keyword::False => "false",
        };
        write!(f, "{}", text)
    }
}
