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
    Mutable,
    If,
    Else,
    Loop,
    Until,
}
use self::Keyword::*;

impl Keyword {
    pub fn get_keyword_kind(keyword: &str) -> TokenKind {
        match keyword {
            "true" => LiteralToken(Variable::from(true)),
            "false" => LiteralToken(Variable::from(false)),
            "is" => KeywordToken(Is),
            "and" => KeywordToken(And),
            "or" => KeywordToken(Or),
            "not" => KeywordToken(Not),
            "xor" => KeywordToken(Xor),
            "mutable" => KeywordToken(Mutable),
            "if" => KeywordToken(If),
            "else" => KeywordToken(Else),
            "loop" => KeywordToken(Loop),
            "until" => KeywordToken(Until),
            identifier => IdentifierToken(identifier.to_string()),
        }
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
            If => "if",
            Else => "else",
            Loop => "loop",
            Until => "until",
        };
        write!(f, "{}", text)
    }
}
