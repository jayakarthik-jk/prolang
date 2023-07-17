use std::{fmt::Display, rc::Rc};

use crate::lexical_analysis::token::Token;

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTree {
    LiteralExpression(Rc<Token>),
    IdentifierExpression(Rc<Token>),
    UnaryExpression(Rc<Token>, Box<AbstractSyntaxTree>),
    BinaryExpression(Box<AbstractSyntaxTree>, Rc<Token>, Box<AbstractSyntaxTree>),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    AssignmentExpression(Box<AbstractSyntaxTree>, Rc<Token>, Box<AbstractSyntaxTree>),
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
                if let AbstractSyntaxTree::IdentifierExpression(identifier) = &**identifier {
                    println!("{}   └─{}", " ".repeat(indent), identifier);
                }
                AbstractSyntaxTree::print_tree(expression, indent + 3);
            }
            AbstractSyntaxTree::IdentifierExpression(value) => {
                println!("{}└─{}", " ".repeat(indent), value);
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                println!("{}└─<( )>", " ".repeat(indent));
                AbstractSyntaxTree::print_tree(expression, indent + 4);
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
            AbstractSyntaxTree::IdentifierExpression(value) => write!(f, "{}", value),
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                write!(f, "( {} )", expression)
            }
        }
    }
}
