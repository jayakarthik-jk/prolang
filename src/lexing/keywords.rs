use std::fmt::Display;

use crate::common::literal::Literal;

use crate::lexing::token::TokenKind::*;

use super::token::TokenKind;

#[derive(PartialEq)]
pub enum Keyword {
    And,
    Or,
    Not,
    Xor,
    Is,
    Let,
    If,
    Else,
    Loop,
    While,
    Until,
    Return,
    Break,
    Skip,
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
            "let" => Keyword(Let),
            "if" => Keyword(If),
            "else" => Keyword(Else),
            "loop" => Keyword(Loop),
            "while" => Keyword(While),
            "return" => Keyword(Return),
            "break" => Keyword(Break),
            "skip" => Keyword(Skip),
            "until" => Keyword(Until),
            identifier => Identifier(identifier.to_string()),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            And => "and",
            Or => "or",
            Not => "not",
            Xor => "xor",
            Is => "is",
            Let => "let",
            If => "if",
            Else => "else",
            Loop => "loop",
            While => "while",
            Return => "return",
            Break => "break",
            Skip => "skip",
            Until => "until",
        };
        write!(f, "{}", text)
    }
}
