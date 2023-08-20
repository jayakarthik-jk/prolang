use std::fmt::Display;

use crate::common::literal::Literal;

use super::{keywords::Keyword, symbols::Symbol};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Literal(Literal),
    /// number of whitespace
    Whitespace(usize),
    NewLine,
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    EndOfFile,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Literal(a) => write!(f, "{}", a),
            TokenKind::Whitespace(a) => write!(f, "{}", a),
            TokenKind::NewLine => write!(f, "NewLineToken"),
            TokenKind::Keyword(a) => write!(f, "{}", a),
            TokenKind::Symbol(a) => write!(f, "{}", a),
            TokenKind::Identifier(a) => write!(f, "{}", a),
            TokenKind::EndOfFile => write!(f, "end of the file"),
        }
    }
}
