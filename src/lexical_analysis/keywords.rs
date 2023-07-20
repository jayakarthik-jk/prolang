use std::fmt::Display;

use crate::common::datatypes::{DataType, Variable};

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
    Null,
    Mutable,
    Nullable,
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
            "mutable" => KeywordToken(Mutable),
            "nullable" => KeywordToken(Nullable),
            "null" => LiteralToken(Variable::new(DataType::Null)),
            identifier => IdentifierToken(identifier.to_string()),
        };
        token
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            True => "true",
            False => "false",
            And => "and",
            Or => "or",
            Not => "not",
            Xor => "xor",
            Is => "is",
            Mutable => "mutable",
            Nullable => "nullable",
            Null => "null",
        };
        write!(f, "{}", text)
    }
}
