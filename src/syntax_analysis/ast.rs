use std::fmt::Display;

use crate::common::{datatypes::Variable, operators::Operator};

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTree {
    LiteralExpression(Variable),
    IdentifierExpression(String),
    UnaryExpression(Operator, Box<AbstractSyntaxTree>),
    BinaryExpression(Box<AbstractSyntaxTree>, Operator, Box<AbstractSyntaxTree>),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    /// indentifer, assignment operator, expression
    AssignmentExpression(Box<AbstractSyntaxTree>, Operator, Box<AbstractSyntaxTree>),
}

impl AbstractSyntaxTree {
    pub fn print(node: &AbstractSyntaxTree) {
        AbstractSyntaxTree::print_tree(node, 0);
    }
    fn print_tree(node: &AbstractSyntaxTree, indent: usize) {
        match node {
            AbstractSyntaxTree::UnaryExpression(operator, operand) => {
                println!("{}├─{}", " ".repeat(indent), operator);
                AbstractSyntaxTree::print_tree(operand, indent + 3);
            }
            AbstractSyntaxTree::BinaryExpression(left, operator, right) => {
                println!("{}├─{}", " ".repeat(indent), operator);
                AbstractSyntaxTree::print_tree(left, indent + 3);
                AbstractSyntaxTree::print_tree(right, indent + 3);
            }
            AbstractSyntaxTree::LiteralExpression(value) => {
                println!("{}└─{}", " ".repeat(indent), value);
            }
            AbstractSyntaxTree::AssignmentExpression(identifier, equals, expression) => {
                println!("{}├─{}", " ".repeat(indent), equals);
                println!("{}   └─{}", " ".repeat(indent), identifier);
                AbstractSyntaxTree::print_tree(expression, indent + 3);
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                println!("{}└─<( )>", " ".repeat(indent));
                AbstractSyntaxTree::print_tree(expression, indent + 4);
            }
            AbstractSyntaxTree::IdentifierExpression(name) => {
                println!("{}└─{}", " ".repeat(indent), name);
            }
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
            AbstractSyntaxTree::LiteralExpression(value) => write!(f, "{}", value),
            AbstractSyntaxTree::AssignmentExpression(identifier, equals, expression) => {
                write!(f, "{} {} {}", identifier, equals, expression)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                write!(f, "( {} )", expression)
            }
            AbstractSyntaxTree::IdentifierExpression(name) => {
                write!(f, "{}", name)
            }
        }
    }
}
