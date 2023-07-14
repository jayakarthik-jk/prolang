use crate::datatypes::DataType::Boolean;
use crate::lexer::TokenKind::LiteralToken;
use crate::{errors::CompilerError, lexer::Token};
pub enum Keyword {
    True,
    False,
}

impl Keyword {
    pub fn parse(word: String, position: usize) -> Result<Token, CompilerError> {
        let token = match word.as_str() {
            "true" => Token::new(LiteralToken(Boolean(true)), position),
            "false" => Token::new(LiteralToken(Boolean(false)), position),
            _ => return Err(CompilerError::UnknownKeywordError(word)),
        };
        Ok(token)
    }
}
