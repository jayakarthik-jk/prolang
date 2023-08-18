#[cfg(test)]

/// only for testing purposes
fn get_lexer(source: String) -> crate::lexing::lexer::Lexer {
    use crate::lexing::lexer::Lexer;
    let mut lexer = Lexer::new(source);
    lexer.lex().unwrap();
    lexer
}
#[test]

fn arithmetic_expressions() {
    use crate::common::variables::Variable;
    use crate::lexing::symbols::Symbol::*;
    use crate::lexing::token::TokenKind::*;

    let source = "1 + 2 * 3 - 4 / 5 ** 6".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Plus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Minus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Slash));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(6))
    );
}

#[test]
fn paranthesized_arithmetic_expression() {
    use crate::common::variables::Variable;
    use crate::lexing::symbols::Symbol::*;
    use crate::lexing::symbols::Symbol::{CloseParanthesis, OpenParanthesis};
    use crate::lexing::token::TokenKind::*;
    let source = "(1 + 2) * 3 - 4 / 5".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(OpenParanthesis)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Plus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(CloseParanthesis)
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Minus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Slash));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
}

// test for relational operators
#[test]
fn relational_expression() {
    use crate::common::variables::Variable;
    use crate::lexing::symbols::Symbol::*;
    use crate::lexing::token::TokenKind::*;
    let source = "1 < 2 <= 3 > 4 >= 5 == 6 != 7".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(LessThan));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(LessThan));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(GreaterThan)
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(Exclamation)
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(7))
    );
}

// test for logical operators
#[test]
fn logical_expression() {
    use crate::common::variables::Variable;
    use crate::lexing::keywords::Keyword::*;
    use crate::lexing::token::TokenKind::*;
    let source = "1 and 2 or 3 xor 4 not 5".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(And));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Or));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Xor));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Not));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
}

// test for assignment operators
#[test]
fn assignment_expression() {
    use crate::common::variables::Variable;
    use crate::lexing::symbols::Symbol::*;
    use crate::lexing::token::TokenKind::*;

    let source = "1 = 2 += 3 -= 4 *= 5 /= 6 %= 7 **= 8".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Plus));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Minus));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Slash));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(6))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Percent));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(7))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(8))
    );
}

// test for unary operators
#[test]
fn all_expressions() {
    use crate::common::variables::Variable;
    use crate::lexing::keywords::Keyword::*;
    use crate::lexing::symbols::Symbol::*;
    use crate::lexing::token::TokenKind::*;
    let source = "1 + 2 - 3 * 4 / 5 % 6 ** 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15 xor 16 not 17 = 18 += 19 -= 20 *= 21 /= 22 %= 23 **= 24".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Plus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(2))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Minus));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(3))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(4))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Slash));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(5))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Percent));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(6))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(7))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(LessThan));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(8))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(LessThan));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(9))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(10))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(GreaterThan)
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(11))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(12))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Symbol(Exclamation)
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(13))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(And));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(14))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Or));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(15))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Xor));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(16))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Not));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(17))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(18))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Plus));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(19))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Minus));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(20))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(21))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Slash));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(22))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Percent));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(23))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Asterisk));
    assert_eq!(lexer.get_current_token_and_advance().kind, Symbol(Equals));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(24))
    );
}

// write test for identifiers
#[test]
fn test_valid_identifiers() {
    use crate::lexing::token::TokenKind::*;
    let source = "a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z _ abc _abc abc_ abc123 abc_123 abc_123_def"
        .to_string();
    let reference = source.to_string();
    let lexer = get_lexer(source);
    for ch in reference.split_whitespace() {
        assert_eq!(
            lexer.get_current_token_and_advance().kind,
            Identifier(ch.to_string())
        );
    }
}

#[test]
fn test_invalid_identifiers() {
    use crate::common::variables::Variable;
    use crate::lexing::token::TokenKind::*;

    let source = "1abc".to_string();
    let lexer = get_lexer(source);
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Identifier("abc".to_string())
    );
}

#[test]
fn test_keywords() {
    use crate::common::variables::Variable;
    use crate::lexing::keywords::Keyword::*;
    use crate::lexing::token::TokenKind::*;

    let source = "and or xor not true false mutable".to_string();
    let lexer = get_lexer(source);
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(And));
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Or));
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Xor));
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Not));
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(true))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        Literal(Variable::from(false))
    );
    assert_eq!(lexer.get_current_token_and_advance().kind, Keyword(Mutable));
}
