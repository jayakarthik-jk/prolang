use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::common::datatypes::DataType;
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment::*;
use crate::common::operators::relational::Relational::*;
use crate::common::operators::{Operator, Operator::*};
use crate::common::symbol_table::SymbolTable;
use crate::lexical_analysis::keywords::Keyword;
use crate::lexical_analysis::symbols::Symbol::{self, *};
pub struct Lexer {
    source: Vec<u8>,
    tokens: Vec<Rc<Token>>,
    index: RefCell<usize>,
    position: usize,
    current: char,
    pub line: usize,
    pub column: usize,
    symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Lexer {
    pub fn new(source: String, symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
        let source: Vec<u8> = source.bytes().collect();
        let current = if source.is_empty() {
            '\0'
        } else {
            source[0] as char
        };

        Self {
            source,
            tokens: Vec::new(),
            index: RefCell::new(0),
            position: 0,
            current,
            line: 1,
            column: 1,
            symbol_table,
        }
    }

    fn _next(&mut self) -> char {
        self.current = if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.column += 1;
            self.position += 1;
            self.source[self.position] as char
        };
        self.current
    }

    fn _peek(&self, offset: usize) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + offset] as char
        }
    }

    pub fn parse_token(&mut self) -> Result<Token, CompilerError> {
        if self.current.is_ascii_whitespace() {
            let start = self.position;
            while self.current.is_ascii_whitespace() {
                self._next();
            }
            let count = self.position - start;
            let token = Token::new(TokenKind::WhitespaceToken(count), self.line, self.column);
            return Ok(token);
        }

        let current_token = match self.current {
            '\r' if self._peek(1) == '\n' => {
                let token = Token::new(TokenKind::NewLineToken, self.line, self.column);
                self._next();
                self._next();
                self.line += 1;
                self.column = 1;
                token
            }
            '\0' => return Err(CompilerError::EndOfSourceCodeError),
            current if current.is_ascii_digit() => {
                let start = self.position;
                while self.current.is_ascii_digit() {
                    self._next();
                }
                if self.current == '.' {
                    self._next();
                    while self.current.is_ascii_digit() {
                        self._next();
                    }
                    let number = &self.source[start..self.position];
                    let number_as_string = String::from_utf8_lossy(number);
                    let number: f64 = match number_as_string.parse() {
                        Ok(number) => number,
                        Err(_) => {
                            return Err(CompilerError::InvalidNumber(
                                number_as_string.to_string(),
                                self.line,
                                self.column - number_as_string.len(),
                            ));
                        }
                    };
                    Token::new(
                        TokenKind::LiteralToken(DataType::Float(number)),
                        self.line,
                        self.column - number_as_string.len(),
                    )
                } else {
                    let number = &self.source[start..self.position];
                    let number_as_string = String::from_utf8_lossy(number);
                    let number: i128 = match number_as_string.parse() {
                        Ok(number) => number,
                        Err(_) => {
                            return Err(CompilerError::InvalidNumber(
                                number_as_string.to_string(),
                                self.line,
                                self.column - number_as_string.len(),
                            ));
                        }
                    };
                    Token::new(
                        TokenKind::LiteralToken(DataType::Integer(number)),
                        self.line,
                        self.column - number_as_string.len(),
                    )
                }
            }
            current if current.is_alphabetic() || current == '_' => {
                let start = self.position;
                while self.current.is_ascii_alphanumeric() || self.current == '_' {
                    self._next();
                }
                let word = &self.source[start..self.position];
                let word = String::from_utf8_lossy(word);
                self.parse_keyword(word.to_string())
            }
            current if current == '\'' => {
                self._next();
                let start = self.position;
                while self.current != '\'' && self._next() != '\0' {
                    // TODO: Handle escape characters
                }
                let string = if self.current == '\'' {
                    let string = &self.source[start..self.position];
                    let string = String::from_utf8_lossy(string);
                    string
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::LiteralToken(DataType::String(string.to_string())),
                    self.line,
                    self.column - string.len(),
                );
                self._next();
                token
            }
            current if current == '\"' => {
                self._next();
                let start = self.position;
                while self.current != '\"' && self._next() != '\0' {
                    // TODO: Handle escape characters
                }
                let string = if self.current == '\"' {
                    let string = &self.source[start..self.position];
                    let string = String::from_utf8_lossy(string);
                    string
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::LiteralToken(DataType::String(string.to_string())),
                    self.line,
                    self.column - string.len(),
                );
                self._next();
                token
            }
            '+' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(AdditionAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Addition)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '-' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(SubtractionAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Subtraction)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '*' => {
                if self._peek(1) == '*' && self._peek(2) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(ExponentiationAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else if self._peek(1) == '*' {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Exponentiation)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(MultiplicationAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Multiplication)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '/' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(DivisionAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Division)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '%' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(ModuloAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Modulo)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '(' => {
                let token = Token::new(
                    TokenKind::SymbolToken(OpenParanthesis),
                    self.line,
                    self.column,
                );
                self._next();
                token
            }
            ')' => {
                let token = Token::new(
                    TokenKind::SymbolToken(CloseParanthesis),
                    self.line,
                    self.column,
                );
                self._next();
                token
            }
            '=' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(RelationalOperator(Equals)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(AssingmentOperator(SimpleAssignment)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '<' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(RelationalOperator(LessThanOrEquals)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(RelationalOperator(LessThan)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }
            '>' => {
                if self._peek(1) == '=' {
                    let token = Token::new(
                        TokenKind::OperatorToken(RelationalOperator(GreaterThanOrEquals)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    self._next();
                    token
                } else {
                    let token = Token::new(
                        TokenKind::OperatorToken(RelationalOperator(GreaterThan)),
                        self.line,
                        self.column,
                    );
                    self._next();
                    token
                }
            }

            _ => {
                return Err(CompilerError::InvalidCharacter(
                    self.current,
                    self.line,
                    self.column,
                ))
            }
        };

        Ok(current_token)
        // return Ok(self.tokens.last().unwrap());
    }

    pub fn lex_with_whitespace(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.parse_token() {
                Ok(token) => {
                    self.tokens.push(Rc::new(token));
                }
                Err(error) if CompilerError::EndOfSourceCodeError == error => {
                    return Ok(());
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
    }

    pub fn lex(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.parse_token() {
                Ok(token) => match token.kind {
                    TokenKind::NewLineToken | TokenKind::WhitespaceToken(_) => {}
                    _ => {
                        self.tokens.push(Rc::new(token));
                    }
                },
                Err(error) if CompilerError::EndOfSourceCodeError == error => {
                    return Ok(());
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
    }

    /// returns tokens\[pointer\]
    pub fn advance(&self) {
        let mut index = self.index.borrow_mut();
        *index += 1;
    }

    /// returns --pointer
    pub fn rewind(&self) {
        let mut index = self.index.borrow_mut();
        if *index > 0 {
            *index -= 1;
        }
    }

    pub fn peek(&self, offset: usize) -> Result<Rc<Token>, CompilerError> {
        let index = *self.index.borrow();
        match self.tokens.get(index + offset) {
            Some(token) => Ok(Rc::clone(token)),
            None => {
                if self.tokens.is_empty() {
                    Err(CompilerError::NoTokensAvailable)
                } else {
                    Err(CompilerError::EndOfSourceCodeError)
                }
            }
        }
    }

    /// returns tokens\[pointer\]
    pub fn get_current_token(&self) -> Result<Rc<Token>, CompilerError> {
        let index = *self.index.borrow();
        match self.tokens.get(index) {
            Some(token) => Ok(Rc::clone(token)),
            None => {
                if self.tokens.is_empty() {
                    Err(CompilerError::NoTokensAvailable)
                } else {
                    Err(CompilerError::EndOfSourceCodeError)
                }
            }
        }
    }

    /// returns tokens\[++pointer\]
    pub fn advance_and_get_current_token(&self) -> Result<Rc<Token>, CompilerError> {
        self.advance();
        self.get_current_token()
    }

    /// returns tokens\[--pointer\]
    pub fn rewind_and_get_current_token(&self) -> Result<Rc<Token>, CompilerError> {
        self.rewind();
        self.get_current_token()
    }

    /// returns tokens\[pointer++\]
    pub fn get_current_token_and_advance(&self) -> Result<Rc<Token>, CompilerError> {
        let current = self.get_current_token();
        self.advance();
        current
    }

    /// returns tokens\[pointer--\]
    pub fn get_current_token_and_rewind(&self) -> Result<Rc<Token>, CompilerError> {
        let current = self.get_current_token();
        self.rewind();
        current
    }

    /// returns tokens.len()
    pub fn get_token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn generate_factory_token(&self, line: usize, column: usize) -> Rc<Token> {
        Rc::new(Token::new(TokenKind::FactoryToken, line, column))
    }

    fn parse_keyword(&self, word: String) -> Token {
        let keyword = Keyword::get_keyword_kind(&word);
        let mut symbol_table = self.symbol_table.borrow_mut();
        if let TokenKind::IdentifierToken(name) = &keyword
        && !symbol_table.variables.contains_key(name) {
            symbol_table
                .variables
                .insert(name.clone(), DataType::InternalUndefined);
        }
        Token::new(keyword, self.line, self.column - word.len())
    }
}

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
