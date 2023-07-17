use std::fmt::Display;

use crate::common::{datatypes::DataType, operators::Operator};

use super::{keywords::Keyword, symbols::Symbol};


#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
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
    LiteralToken(DataType),
    /// number of whitespace
    WhitespaceToken(usize),
    NewLineToken,
    OperatorToken(Operator),
    KeywordToken(Keyword),
    SymbolToken(Symbol),
    FactoryToken,
    IdentifierToken(String),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LiteralToken(a) => write!(f, "{}", a),
            TokenKind::WhitespaceToken(a) => write!(f, "{}", a),
            TokenKind::NewLineToken => write!(f, "NewLineToken"),
            TokenKind::OperatorToken(a) => write!(f, "{}", a),
            TokenKind::KeywordToken(a) => write!(f, "{}", a),
            TokenKind::SymbolToken(a) => write!(f, "{}", a),
            TokenKind::FactoryToken => write!(f, "FactoryToken"),
            TokenKind::IdentifierToken(a) => write!(f, "{}", a),
        }
    }
}
