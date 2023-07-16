use std::fmt::Display;

use crate::common::datatypes::DataType::Boolean;
use crate::common::operators::logical::Logical;
use crate::common::operators::Operator::LogicalOperator;
use crate::lexical_analysis::lexer::TokenKind::*;

use super::lexer::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
}

impl Keyword {
    pub fn get_keyword_kind(keyword: &str) -> TokenKind {
        let token = match keyword {
            "true" => LiteralToken(Boolean(true)),
            "false" => LiteralToken(Boolean(false)),
            "and" => OperatorToken(LogicalOperator(Logical::And)),
            "or" => OperatorToken(LogicalOperator(Logical::Or)),
            "not" => OperatorToken(LogicalOperator(Logical::Not)),
            "xor" => OperatorToken(LogicalOperator(Logical::Xor)),
            identifier => IdentifierToken(identifier.to_string()),
        };
        token
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
