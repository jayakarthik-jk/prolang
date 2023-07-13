use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    LiteralExpression(&'a Token),
    BinaryExpression(Box<Expression<'a>>, &'a Token, Box<Expression<'a>>),
    ParanthesizedExpression(Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn print(node: &Expression, intent: String) {
        match node {
            Expression::LiteralExpression(number) => {
                println!("{intent}{:?}", number.kind);
            }
            Expression::BinaryExpression(left, operator, right) => {
                println!("{intent}{:?}", operator.kind);
                Expression::print(left, format!("{intent}    "));
                Expression::print(right, format!("{intent}    "));
            }
            Expression::ParanthesizedExpression(expression) => {
                Expression::print(&expression, format!("{intent}    "));
            }
        }
    }
}
