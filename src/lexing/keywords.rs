use std::fmt::Display;

use crate::common::literal::Literal;

use crate::lexing::token::TokenKind::*;

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
    While,
    Return,
}
use self::Keyword::*;

impl Keyword {
    pub(crate) fn get_keyword_kind(keyword: &str) -> TokenKind {
        match keyword {
            "true" => Literal(Literal::from(true)),
            "false" => Literal(Literal::from(false)),
            "is" => Keyword(Is),
            "and" => Keyword(And),
            "or" => Keyword(Or),
            "not" => Keyword(Not),
            "xor" => Keyword(Xor),
            "mutable" => Keyword(Mutable),
            "if" => Keyword(If),
            "else" => Keyword(Else),
            "loop" => Keyword(Loop),
            "while" => Keyword(While),
            "return" => Keyword(Return),
            identifier => Identifier(identifier.to_string()),
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
            While => "while",
            Return => "return",
        };
        write!(f, "{}", text)
    }
}
