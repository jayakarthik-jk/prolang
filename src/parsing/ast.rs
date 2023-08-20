use std::fmt::Display;
use std::sync::{Arc, RwLock};

use crate::common::literal::Literal;
use crate::common::operators::Operator;

use super::block::Block;
use super::seperated_statements::SeperatedStatements;

#[derive(Debug)]
pub(crate) enum AbstractSyntaxTree {
    // Factors
    Literal(Literal),
    Identifier(String),
    // Object(SeperatedStatements<KeyValuePair>),
    // Expressions
    UnaryExpression(Operator, Box<AbstractSyntaxTree>),
    BinaryExpression(Box<AbstractSyntaxTree>, Operator, Box<AbstractSyntaxTree>),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    AssignmentExpression(String, Operator, Box<AbstractSyntaxTree>),

    // statements
    BlockStatement(Arc<RwLock<Block>>),
    IfStatement(
        Box<AbstractSyntaxTree>,         // condition
        Box<AbstractSyntaxTree>,         // block
        Option<Box<AbstractSyntaxTree>>, // else statement
    ),
    ElseStatement(Box<AbstractSyntaxTree>), // block or if statement

    LoopStatement(Box<AbstractSyntaxTree>, Box<AbstractSyntaxTree>),
    CallStatement(String, SeperatedStatements<Box<AbstractSyntaxTree>>),
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
        match self {
            AbstractSyntaxTree::UnaryExpression(operator, operand) => {
                write!(f, "{}{}", operator, operand)
            }
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                write!(f, "{} {} {}", left, operator, right)
            }
            AbstractSyntaxTree::Literal(value) => write!(f, "{}", value),
            AbstractSyntaxTree::AssignmentExpression(identifier, equals, expression) => {
                write!(f, "{} {} {}", identifier, equals, expression)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                write!(f, "( {} )", expression)
            }
            AbstractSyntaxTree::Identifier(name) => write!(f, "{}", name),

            AbstractSyntaxTree::BlockStatement(_) => write!(f, "{{ block }}"),
            AbstractSyntaxTree::IfStatement(_, _, _) => write!(f, "if condition {{ block }}"),
            AbstractSyntaxTree::ElseStatement(_) => write!(f, "else {{ block }}"),
            AbstractSyntaxTree::LoopStatement(_, _) => write!(f, "loop until condition {{ }}"),
            AbstractSyntaxTree::CallStatement(name, _) => write!(f, "Function call: {name}"),
        }
    }
}
