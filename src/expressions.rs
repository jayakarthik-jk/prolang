use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum SyntaxExpression<'a> {
    LiteralExpression(&'a Token),
    UnaryExpression(&'a Token, Box<SyntaxExpression<'a>>),
    BinaryExpression(
        Box<SyntaxExpression<'a>>,
        &'a Token,
        Box<SyntaxExpression<'a>>,
    ),
}

impl<'a> SyntaxExpression<'a> {
    pub fn print(node: &SyntaxExpression, intent: String) {
        match node {
            SyntaxExpression::LiteralExpression(number) => {
                println!("{intent}{:?}", number.kind);
            }
            SyntaxExpression::BinaryExpression(left, operator, right) => {
                println!("{intent}{:?}", operator.kind);
                SyntaxExpression::print(left, format!("{intent}    "));
                SyntaxExpression::print(right, format!("{intent}    "));
            }
            SyntaxExpression::UnaryExpression(operator, expression) => {
                println!("{intent}{:?}", operator.kind);
                SyntaxExpression::print(&expression, format!("{intent}    "));
            }
        }
    }
}
