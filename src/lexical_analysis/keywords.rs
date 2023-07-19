use std::fmt::Display;

use crate::common::datatypes::Variable;

use crate::lexical_analysis::token::TokenKind::*;

use super::token::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
    And,
    Or,
    Not,
    Xor,
    Is,
}
use self::Keyword::*;

impl Keyword {
    pub fn get_keyword_kind(keyword: &str) -> TokenKind {
        let token = match keyword {
            "true" => LiteralToken(Variable::from(true)),
            "false" => LiteralToken(Variable::from(false)),
            "is" => KeywordToken(Is),
            "and" => KeywordToken(And),
            "or" => KeywordToken(Or),
            "not" => KeywordToken(Not),
            "xor" => KeywordToken(Xor),
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
            Keyword::And => "and",
            Keyword::Or => "or",
            Keyword::Not => "not",
            Keyword::Xor => "xor",
            Keyword::Is => "is",
        };
        write!(f, "{}", text)
    }
}
