use std::fmt::Display;

use crate::common::datatypes::Variable;

use super::{keywords::Keyword, symbols::Symbol};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LiteralToken(Variable),
    /// number of whitespace
    WhitespaceToken(usize),
    NewLineToken,
    KeywordToken(Keyword),
    SymbolToken(Symbol),
    IdentifierToken(String),
    EndOfFileToken,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LiteralToken(a) => write!(f, "{}", a),
            TokenKind::WhitespaceToken(a) => write!(f, "{}", a),
            TokenKind::NewLineToken => write!(f, "NewLineToken"),
            TokenKind::KeywordToken(a) => write!(f, "{}", a),
            TokenKind::SymbolToken(a) => write!(f, "{}", a),
            TokenKind::IdentifierToken(a) => write!(f, "{}", a),
            TokenKind::EndOfFileToken => write!(f, "EndOfFileToken"),
        }
    }
}
