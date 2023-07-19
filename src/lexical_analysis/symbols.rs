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
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Symbol::OpenParanthesis => "(",
            Symbol::CloseParanthesis => ")",
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Asterisk => "*",
            Symbol::Slash => "/",
            Symbol::Percent => "%",
            Symbol::Equals => "=",
            Symbol::Exclamation => "!",
            Symbol::GreaterThan => ">",
            Symbol::LessThan => "<",
            Symbol::OpenSquareBracket => "[",
            Symbol::CloseSquareBracket => "]",
            Symbol::OpenCurlyBracket => "{",
            Symbol::CloseCurlyBracket => "}",
        };
        write!(f, "{}", text)
    }
}
