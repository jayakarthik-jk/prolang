use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use crate::common::{datatypes::Variable, operators::Operator};

use super::block::Block;

#[derive(Debug)]
pub enum AbstractSyntaxTree {
    // Factors
    Literal(Variable),
    Identifier(String),
    // Expressions
    UnaryExpression(Operator, Box<AbstractSyntaxTree>),
    BinaryExpression(Box<AbstractSyntaxTree>, Operator, Box<AbstractSyntaxTree>),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    AssignmentExpression(String, Operator, Box<AbstractSyntaxTree>),

    // statements
    BlockStatement(Rc<RefCell<Block>>),
    IfStatement(
        Box<AbstractSyntaxTree>,         // condition
        Box<AbstractSyntaxTree>,         // block
        Option<Box<AbstractSyntaxTree>>, // else statement
    ),
    ElseStatement(Box<AbstractSyntaxTree>), // block or if statement

    LoopStatement(Box<AbstractSyntaxTree>, Box<AbstractSyntaxTree>),
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
        }
    }
}
