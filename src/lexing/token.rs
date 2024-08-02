use std::fmt::Display;

use crate::common::literal::Literal;

use super::{keywords::Keyword, symbols::Symbol};

pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.kind)
    }
}

#[derive(PartialEq)]
pub enum TokenKind {
    Literal(Literal),
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    EndOfFile,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        match self {
            Literal(a) => write!(f, "{}", a),
            Keyword(a) => write!(f, "{}", a),
            Symbol(a) => write!(f, "{}", a),
            Identifier(a) => write!(f, "{}", a),
            EndOfFile => write!(f, "end of the file"),
        }
    }
}
