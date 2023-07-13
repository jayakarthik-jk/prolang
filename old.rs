fn parse_expression(&self) -> Result<Expression, LexerError> {
    let mut left = self.parse_term()?;

    if let Ok(operator) = self.lexer.get_current_token()
    && (operator.kind == TokenKind::OperatorToken(ArithmeticOperator(Addition))
    || operator.kind == TokenKind::OperatorToken(ArithmeticOperator(Subtraction))) {
        self.lexer.advance();
        let right = self.parse_expression()?;
            left = Expression::BinaryExpression(Box::new(left), operator, Box::new(right));
    }
    Ok(left)
}

fn parse_term(&self) -> Result<Expression, LexerError> {
    let mut left = self.parse_factor()?;

    if let Ok(operator) = self.lexer.get_current_token()
        && (operator.kind == TokenKind::OperatorToken(ArithmeticOperator(Multiplication))
        || operator.kind == TokenKind::OperatorToken(ArithmeticOperator(Division))) {
            self.lexer.advance();
            let right = self.parse_term()?;
            left = Expression::BinaryExpression(Box::new(left), operator, Box::new(right));
    }
    Ok(left)
}
