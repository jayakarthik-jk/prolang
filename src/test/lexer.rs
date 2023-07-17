#[cfg(test)]

/// only for testing purposes
fn get_lexer(source: String) -> crate::lexical_analysis::lexer::Lexer {
    use crate::common::symbol_table::SymbolTable;
    use crate::lexical_analysis::lexer::Lexer;
    let symbol_table = SymbolTable::sharable();
    Lexer::new(source, symbol_table)
}
#[test]
fn arithmetic_expressions() {
    use crate::common::datatypes::DataType::Integer;
    use crate::common::operators::arithmetic::Arithmetic::*;
    use crate::common::operators::Operator::ArithmeticOperator;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 + 2 * 3 - 4 / 5 ** 6".to_string();
    let mut lexer = get_lexer(source);

    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Addition))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Multiplication))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Subtraction))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Division))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Exponentiation))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(6))
    );
}

#[test]
fn paranthesized_arithmetic_expression() {
    use crate::common::datatypes::DataType::Integer;
    use crate::common::operators::arithmetic::Arithmetic::*;
    use crate::common::operators::Operator::ArithmeticOperator;
    use crate::lexical_analysis::symbols::Symbol::{CloseParanthesis, OpenParanthesis};
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "(1 + 2) * 3 - 4 / 5".to_string();
    let mut lexer = get_lexer(source);

    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        SymbolToken(OpenParanthesis)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Addition))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        SymbolToken(CloseParanthesis)
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Multiplication))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Subtraction))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Division))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
}

// test for relational operators
#[test]
fn relational_expression() {
    use crate::common::datatypes::DataType::Integer;
    use crate::common::operators::relational::Relational::*;
    use crate::common::operators::Operator::RelationalOperator;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 < 2 <= 3 > 4 >= 5 == 6 != 7".to_string();
    let mut lexer = get_lexer(source);

    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(LessThan))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(LessThanOrEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(GreaterThan))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(GreaterThanOrEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(Equals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(NotEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(7))
    );
}

// test for logical operators
#[test]
fn logical_expression() {
    use crate::common::datatypes::DataType::Integer;
    use crate::common::operators::logical::Logical::*;
    use crate::common::operators::Operator::LogicalOperator;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 and 2 or 3 xor 4 not 5".to_string();
    let mut lexer = get_lexer(source);

    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(And))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Or))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Xor))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Not))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
}

// test for assignment operators
#[test]
fn assignment_expression() {
    use crate::common::datatypes::DataType::*;
    use crate::common::operators::assignment::Assingment::*;
    use crate::common::operators::Operator::AssingmentOperator;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 = 2 += 3 -= 4 *= 5 /= 6 %= 7 **= 8".to_string();
    let mut lexer = get_lexer(source);

    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(SimpleAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(AdditionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(SubtractionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(MultiplicationAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(DivisionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(ModuloAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(7))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(ExponentiationAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(8))
    );
}

// test for unary operators
#[test]
fn all_expressions() {
    use crate::common::datatypes::DataType::*;
    use crate::common::operators::arithmetic::Arithmetic::*;
    use crate::common::operators::assignment::Assingment::*;
    use crate::common::operators::logical::Logical::*;
    use crate::common::operators::relational::Relational::*;
    use crate::common::operators::Operator::*;
    use crate::lexical_analysis::token::TokenKind::*;
    let source = "1 + 2 - 3 * 4 / 5 % 6 ** 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15 xor 16 not 17 = 18 += 19 -= 20 *= 21 /= 22 %= 23 **= 24".to_string();
    let mut lexer = get_lexer(source);
    lexer.lex().unwrap();

    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Addition))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(2))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Subtraction))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(3))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Multiplication))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(4))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Division))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(5))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Modulo))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(6))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(ArithmeticOperator(Exponentiation))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(7))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(LessThan))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(8))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(LessThanOrEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(9))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(GreaterThan))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(10))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(GreaterThanOrEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(11))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(Equals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(12))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(RelationalOperator(NotEquals))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(13))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(And))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(14))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Or))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(15))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Xor))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(16))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(LogicalOperator(Not))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(17))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(SimpleAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(18))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(AdditionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(19))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(SubtractionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(20))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(MultiplicationAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(21))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(DivisionAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(22))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(ModuloAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(23))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        OperatorToken(AssingmentOperator(ExponentiationAssignment))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(24))
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
    let mut lexer = get_lexer(source);
    lexer.lex().unwrap();
    for ch in reference.split_whitespace() {
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            IdentifierToken(ch.to_string())
        );
    }
}

#[test]
fn test_invalid_identifiers() {
    use crate::common::datatypes::DataType::*;
    use crate::lexical_analysis::token::TokenKind::*;

    let source = "1abc".to_string();
    let mut lexer = get_lexer(source);
    lexer.lex().unwrap();
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        LiteralToken(Integer(1))
    );
    assert_eq!(
        lexer.get_current_token_and_advance().unwrap().kind,
        IdentifierToken("abc".to_string())
    );
}

#[test]
fn test_keywords() {
    use crate::lexical_analysis::token::TokenKind::*;
    use crate::common::operators::Operator::LogicalOperator;
    use crate::common::operators::logical::Logical::*;
    use crate::common::datatypes::DataType::Boolean;

    let source = "and or xor not true false".to_string();
    let mut lexer = get_lexer(source);
    lexer.lex().unwrap();
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            OperatorToken(LogicalOperator(And))
        );
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            OperatorToken(LogicalOperator(Or))
        );
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            OperatorToken(LogicalOperator(Xor))
        );
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            OperatorToken(LogicalOperator(Not))
        );
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            LiteralToken(Boolean(true))
        );
        assert_eq!(
            lexer.get_current_token_and_advance().unwrap().kind,
            LiteralToken(Boolean(false))
        );
}