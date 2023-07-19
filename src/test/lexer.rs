#[cfg(test)]

/// only for testing purposes
fn get_lexer(source: String) -> crate::lexical_analysis::lexer::Lexer {
    use crate::common::symbol_table::SymbolTable;
    use crate::lexical_analysis::lexer::Lexer;
    let symbol_table = SymbolTable::sharable();
    let mut lexer = Lexer::new(source, symbol_table);
    lexer.lex().unwrap();
    lexer
}
#[test]
fn arithmetic_expressions() {
    use crate::lexical_analysis::symbols::Symbol::*;
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;

    let source = "1 + 2 * 3 - 4 / 5 ** 6".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Plus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Minus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Slash)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(6))
    );
}

#[test]
fn paranthesized_arithmetic_expression() {
    use crate::lexical_analysis::symbols::Symbol::*;
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::symbols::Symbol::{CloseParanthesis, OpenParanthesis};
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "(1 + 2) * 3 - 4 / 5".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(OpenParanthesis)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Plus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(CloseParanthesis)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Minus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Slash)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
}

// test for relational operators
#[test]
fn relational_expression() {
    use crate::lexical_analysis::symbols::Symbol::*;
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 < 2 <= 3 > 4 >= 5 == 6 != 7".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(LessThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(LessThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Exclamation)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(7))
    );
}

// test for logical operators
#[test]
fn logical_expression() {
    use crate::lexical_analysis::keywords::Keyword::*;
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 and 2 or 3 xor 4 not 5".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(And)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Or)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Xor)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Not)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
}

// test for assignment operators
#[test]
fn assignment_expression() {
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;
    use crate::lexical_analysis::symbols::Symbol::*;

    let source = "1 = 2 += 3 -= 4 *= 5 /= 6 %= 7 **= 8".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Plus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Minus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Slash)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Percent)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(7))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(8))
    );
}

// test for unary operators
#[test]
fn all_expressions() {
    use crate::lexical_analysis::symbols::Symbol::*;
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;
    use crate::lexical_analysis::keywords::Keyword::*;
    let source = "1 + 2 - 3 * 4 / 5 % 6 ** 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15 xor 16 not 17 = 18 += 19 -= 20 *= 21 /= 22 %= 23 **= 24".to_string();
    let lexer = get_lexer(source);

    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Plus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Minus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Slash)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Percent)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(7))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(LessThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(8))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(LessThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(9))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(10))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(GreaterThan)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(11))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(12))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Exclamation)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(13))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(And)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(14))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Or)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(15))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Xor)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(16))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Not)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(17))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(18))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Plus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(19))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Minus)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(20))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(21))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Slash)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(22))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Percent)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(23))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Asterisk)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        SymbolToken(Equals)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(24))
    );
}

// write test for identifiers
#[test]
fn test_valid_identifiers() {
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "a b c d e f g h i j k l m n o p q r s t u v w x y z 
                        A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
                        _ abc _abc abc_ abc123 abc_123 abc_123_def"
        .to_string();
    let reference = source.to_string();
    let lexer = get_lexer(source);
    for ch in reference.split_whitespace() {
        assert_eq!(
            lexer.get_current_token_and_advance().kind,
            IdentifierToken(ch.to_string())
        );
    }
}

#[test]
fn test_invalid_identifiers() {
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;

    let source = "1abc".to_string();
    let lexer = get_lexer(source);
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        IdentifierToken("abc".to_string())
    );
}

#[test]
fn test_keywords() {
    use crate::common::datatypes::Variable;
    use crate::lexical_analysis::token::TokenKind::*;
    use crate::lexical_analysis::keywords::Keyword::*;

    let source = "and or xor not true false".to_string();
    let lexer = get_lexer(source);
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(And)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Or)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Xor)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        KeywordToken(Not)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(true))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().kind,
        LiteralToken(Variable::from(false))
    );
}
