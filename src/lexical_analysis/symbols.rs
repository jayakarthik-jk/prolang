use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
    OpenParanthesis,
    CloseParanthesis,
    // OpenSquareBracket,
    // CloseSquareBracket,
    // OpenCurlyBracket,
    // CloseCurlyBracket,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Symbol::OpenParanthesis => "(",
            Symbol::CloseParanthesis => ")",
            // Symbol::OpenSquareBracket => "[",
            // Symbol::CloseSquareBracket => "]",
            // Symbol::OpenCurlyBracket => "{",
            // Symbol::CloseCurlyBracket => "}",
        };
        write!(f, "{}", text)
    }
}
