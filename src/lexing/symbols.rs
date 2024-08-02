use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
    OpenParanthesis,
    CloseParanthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equals,
    Exclamation,
    GreaterThan,
    LessThan,
    Comma,
    Colon,
    Semicolon,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Symbol::*;
        let text = match self {
            OpenParanthesis => "(",
            CloseParanthesis => ")",
            Plus => "+",
            Minus => "-",
            Asterisk => "*",
            Slash => "/",
            Percent => "%",
            Equals => "=",
            Exclamation => "!",
            GreaterThan => ">",
            LessThan => "<",
            OpenSquareBracket => "[",
            CloseSquareBracket => "]",
            OpenCurlyBracket => "{",
            CloseCurlyBracket => "}",
            Comma => ",",
            Colon => ":",
            Semicolon => ";",
            // TODO: add bitwise operators &, |, ^
        };
        write!(f, "{}", text)
    }
}
