use std::fmt::Display;

use crate::common::literal::Literal;
use crate::common::operators::Operator;
use crate::lexing::symbols::Symbol;
use crate::lexing::token::TokenKind;

pub enum CompilerError {
    // NoTokensAvailable,

    // // Lexical Errors
    // InvalidCharacter(
    //     char,  // character
    //     usize, // line
    //     usize, // column
    // ),
    // InvalidNumber(
    //     String, // number
    //     usize,  // line
    //     usize,  // column
    // ),
    // UnterminatedString(usize, usize),
    // InvalidKeyword,
    // InvalidUtf8Character,

    // Syntax Errors
    UnexpectedToken(
        TokenKind, // Token
        usize,     // line
        usize,     // column
    ),
    UnexpectedTokenWithExpected(
        TokenKind, // Unexpected Token
        TokenKind, // Expected Token
        usize,     // line
        usize,     // column
    ),
    InvalidOperationAsAssignmentOperation,
    CannotConvertFromImmutableToMutable,
    UnInitializedVariable(String),
    MissingArrow(
        TokenKind, // Token
        usize,     // line
        usize,     // column
    ),

    // Evaluation Errors
    InvalidOperatorForBinaryOperation(Operator),
    InvalidOperatorForUnaryOperation(Operator),

    // Semantic Errors
    UndefinedVariable(String),
    InvalidSeperator(Symbol),
    InvalidEncloser(Symbol),
    UndefinedFunction(String),
    // InvalidExpressionAssignment,
    InvalidAssignment,
    InvalidStringParsing(Literal),
    InvalidUneryOperation,
    UnsupportedOperationBetween(
        Literal,  // Left
        Operator, // Operator
        Literal,  // Right
    ),
    MathUndefined,
    InvalidUseOfMutableKeyword,
    ImmutableVariable(String),
    OperationOnFunction,
    NotAFunction(String),
    ArgumentLengthMismatch(
        String, // function name
        usize,  // expected
        usize,  // got
    ),
    OperationOnReturn,
    ReturnOutsideFunction,
    BreakOutsideLoop,
    OperationOnBreak,
    OperationOnSkip,
    SkipOutsideLoop,
    SkipCountTypeMisMatch(String),
    InvalidType(String),

    // Internal Errors
    OperationOnUndefined,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            // CompilerError::NoTokensAvailable => "No tokens available".to_string(),
            // CompilerError::InvalidCharacter(character, line, column) => format!(
            //     "Invalid character '{}' at line {}, column {}",
            //     character, line, column
            // ),
            // CompilerError::InvalidNumber(number, line, column) => format!(
            //     "Invalid number '{}' at line {}, column {}",
            //     number, line, column
            // ),
            // CompilerError::UnterminatedString(line, column) => {
            //     format!("Unterminated string at line {}, column {}", line, column)
            // }
            CompilerError::UnexpectedToken(token, line, column) => format!(
                "Unexpected token '{}' at line {}, column {}",
                token, line, column
            ),
            CompilerError::InvalidOperatorForBinaryOperation(operator) => {
                format!("Invalid operator '{}' for binary operation", operator)
            }
            CompilerError::InvalidOperatorForUnaryOperation(operator) => {
                format!("Invalid operator '{}' for unary operation", operator)
            }
            CompilerError::UndefinedVariable(name) => format!("Undefined variable '{}'", name),
            // CompilerError::InvalidExpressionAssignment => {
            //     "Invalid expression assignment".to_string()
            // }
            CompilerError::InvalidAssignment => "Invalid assignment".to_string(),
            // CompilerError::InvalidKeyword => "Invalid keyword".to_string(),
            CompilerError::InvalidStringParsing(a) => {
                format!("Invalid string parsing: '{}' is not a valid Number", a)
            }
            CompilerError::InvalidUneryOperation => "Invalid unary operation".to_string(),
            CompilerError::UnsupportedOperationBetween(left, operator, right) => {
                format!(
                    "Unsupported operation {} between '{}' and '{}'",
                    operator, left, right
                )
            }
            CompilerError::MathUndefined => "Math Error: undefined".to_string(),
            CompilerError::OperationOnUndefined => {
                panic!("operation on undefined. this should not be happening, place an issue")
            }
            // CompilerError::InvalidUtf8Character => "Invalid UTF-8 character".to_string(),
            CompilerError::InvalidUseOfMutableKeyword => {
                "Invalid use of `mutable` keyword".to_string()
            }
            CompilerError::ImmutableVariable(name) => {
                format!("cannot mutate Immutable variable '{}'", name)
            }
            CompilerError::InvalidOperationAsAssignmentOperation => {
                "Invalid operation as assignment operation".to_string()
            }
            CompilerError::CannotConvertFromImmutableToMutable => {
                "Cannot convert from Immutable to Mutable".to_string()
            }
            CompilerError::UnInitializedVariable(name) => format!("Uninitialized variable {name}"),
            CompilerError::UnexpectedTokenWithExpected(unexpected, expected, line, column) => {
                format!(
                    "Unexpected token '{}' at line {}, column {}. Expected {}",
                    unexpected, line, column, expected
                )
            }
            CompilerError::InvalidSeperator(seperator) => format!("invalid seperator {seperator}"),
            CompilerError::InvalidEncloser(encloser) => format!("invalid encloser {encloser}"),
            CompilerError::UndefinedFunction(name) => format!("Undefined function {name}"),
            CompilerError::MissingArrow(token, line, column) => format!(
                "Expected arrow `=>` but got '{}' at line {}, column {}",
                token, line, column
            ),
            CompilerError::OperationOnFunction => {
                "Cannot perform operation on a `Function`".to_string()
            }
            CompilerError::NotAFunction(name) => format!("{name} is not a function"),
            CompilerError::ArgumentLengthMismatch(name, parameter_count, argument_count) => {
                format!(
                    "Function {name} expects {parameter_count} arguements but got {argument_count}"
                )
            }
            CompilerError::OperationOnReturn => {
                "Cannot perform operation on `return` Keyword".to_string()
            }
            CompilerError::ReturnOutsideFunction => {
                "return statement can only occur inside a function".to_string()
            }
            CompilerError::BreakOutsideLoop => {
                "break statement can only occur inside a loop".to_string()
            }
            CompilerError::OperationOnBreak => {
                "cannot perform operation on `break` keyword".to_string()
            }
            CompilerError::OperationOnSkip => {
                "cannot perform operation on `skip` keyword".to_string()
            }
            CompilerError::SkipOutsideLoop => {
                "skip statement can only occur inside a loop".to_string()
            }
            CompilerError::SkipCountTypeMisMatch(datatype) => {
                format!("skip count must be an integer, but got {datatype}")
            }
            CompilerError::InvalidType(received_type) => {
                format!("Invalid type {received_type}")
            }
        };
        write!(f, "{}", text)
    }
}
