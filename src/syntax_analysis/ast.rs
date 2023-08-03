use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use crate::common::{datatypes::Variable, operators::Operator};

use super::block::Block;

#[derive(Debug)]
pub enum AbstractSyntaxTree {
    // Factors
    Literal(Variable),
    Identifier(String),
    // Statements
    UnaryExpression(Operator, Box<AbstractSyntaxTree>),
    BinaryExpression(Box<AbstractSyntaxTree>, Operator, Box<AbstractSyntaxTree>),
    ParenthesizedExpression(Box<AbstractSyntaxTree>),
    AssignmentExpression(String, Operator, Box<AbstractSyntaxTree>),

    // statement
    BlockStatement(Rc<RefCell<Block>>),
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
            AbstractSyntaxTree::Literal(value) => {
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
            AbstractSyntaxTree::Identifier(name) => {
                println!("{}└─{}", " ".repeat(indent), name);
            }
            AbstractSyntaxTree::BlockStatement(block) => {
                println!("{}└─<block>", " ".repeat(indent));
                for statement in block.borrow().statements.iter() {
                    AbstractSyntaxTree::print_tree(statement, indent + 4);
                }
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
            AbstractSyntaxTree::Literal(value) => write!(f, "{}", value),
            AbstractSyntaxTree::AssignmentExpression(identifier, equals, expression) => {
                write!(f, "{} {} {}", identifier, equals, expression)
            }
            AbstractSyntaxTree::ParenthesizedExpression(expression) => {
                write!(f, "( {} )", expression)
            }
            AbstractSyntaxTree::Identifier(name) => {
                write!(f, "{}", name)
            }
            AbstractSyntaxTree::BlockStatement(block) => {
                write!(f, "{:?}", block.borrow().statements)
            }
        }
    }
}
