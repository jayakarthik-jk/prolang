use super::block::Block;
use super::seperated_statements::SeperatedStatements;
use crate::common::literal::Literal;
use crate::common::operators::Operator;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub(crate) enum AbstractSyntaxTree {
    // Factors
    Literal(Literal),
    Identifier(String),
    // Object(SeperatedStatements<KeyValuePair>),
    // Expressions
    UnaryExpression(Operator, Box<AbstractSyntaxTree>),
    BinaryExpression(
        Box<AbstractSyntaxTree>, // left
        Operator,                // operator
        Box<AbstractSyntaxTree>, // right
    ),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    AssignmentExpression(
        String,                  // identifier
        Operator,                // assignment operator
        Box<AbstractSyntaxTree>, // expression
    ),

    // statements
    BlockStatement(Arc<RwLock<Block>>),
    IfStatement(
        Box<AbstractSyntaxTree>,         // condition
        Box<AbstractSyntaxTree>,         // block
        Option<Box<AbstractSyntaxTree>>, // else statement
    ),
    ElseStatement(Box<AbstractSyntaxTree>), // block or if statement

    LoopStatement(
        Box<AbstractSyntaxTree>, // condition
        Box<AbstractSyntaxTree>, // block or statement
    ),
    CallStatement(
        String,                                       // name
        SeperatedStatements<Box<AbstractSyntaxTree>>, // arguments
    ),
    ReturnStatement(Box<AbstractSyntaxTree>),
    BreakStatement(Box<AbstractSyntaxTree>),
    SkipStatement(Box<AbstractSyntaxTree>),
}

impl AbstractSyntaxTree {
    pub(crate) fn to_block(&self) -> Result<Arc<RwLock<Block>>, String> {
        if let AbstractSyntaxTree::BlockStatement(block) = self {
            Ok(Arc::clone(block))
        } else {
            Err("Not a block statement".to_string())
        }
    }
}

impl Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            AbstractSyntaxTree::UnaryExpression(operator, operand) => {
                format!("{}{}", operator, operand)
            }
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                format!("{} {} {}", left, operator, right)
            }
            AbstractSyntaxTree::Literal(value) => format!("{}", value),
            AbstractSyntaxTree::AssignmentExpression(identifier, equals, expression) => {
                format!("{} {} {}", identifier, equals, expression)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                format!("( {} )", expression)
            }
            AbstractSyntaxTree::Identifier(name) => name.to_string(),

            AbstractSyntaxTree::BlockStatement(_) => "{{ block }}".to_string(),
            AbstractSyntaxTree::IfStatement(_, _, _) => "if condition {{ block }}".to_string(),
            AbstractSyntaxTree::ElseStatement(_) => "else {{ block }}".to_string(),
            AbstractSyntaxTree::LoopStatement(_, _) => "loop until condition {{ }}".to_string(),
            AbstractSyntaxTree::CallStatement(name, _) => format!("Function call: {name}"),
            AbstractSyntaxTree::ReturnStatement(_) => "return".to_string(),
            AbstractSyntaxTree::BreakStatement(_) => "break".to_string(),
            AbstractSyntaxTree::SkipStatement(_) => "skip".to_string(),
        };
        write!(f, "{output}")
    }
}
