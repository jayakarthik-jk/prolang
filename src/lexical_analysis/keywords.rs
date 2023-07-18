use std::fmt::Display;

use crate::common::datatypes::Variable;
use crate::common::operators::logical::Logical;
use crate::common::operators::Operator::LogicalOperator;
use crate::lexical_analysis::token::TokenKind::*;

use super::token::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
}

impl Keyword {
    pub fn get_keyword_kind(keyword: &str) -> TokenKind {
        let token = match keyword {
            "true" => LiteralToken(Variable::from(true)),
            "false" => LiteralToken(Variable::from(false)),
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
