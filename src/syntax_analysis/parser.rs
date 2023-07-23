use crate::common::datatypes::{DataType, Variable};
use crate::common::errors::CompilerError;
use crate::common::operators::arithmetic::Arithmetic::*;
use crate::common::operators::assignment::Assingment::*;
use crate::common::operators::logical::Logical;
use crate::common::operators::relational::Relational::*;
use crate::common::operators::Operator;
use crate::common::operators::Operator::*;
use crate::common::symbol_table::SymbolTable;
use crate::lexical_analysis::keywords::Keyword;
use crate::lexical_analysis::lexer::Lexer;
use crate::lexical_analysis::symbols::Symbol::*;
use crate::lexical_analysis::token::TokenKind;
use crate::syntax_analysis::ast::AbstractSyntaxTree;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<AbstractSyntaxTree, CompilerError> {
        if self.lexer.get_token_count() == 0 {
            self.lexer.lex()?;
        }
        self.parse_expression()
    }

    pub fn parse_expression(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        let identifier_token = self.lexer.get_current_token();
        match &identifier_token.kind {
            TokenKind::KeywordToken(keyword) => match keyword {
                Keyword::Mutable => self.handle_mutable_keyword(),
                Keyword::Nullable => self.handle_nullable_keyword(),
                _ => self.parse_arithmetic_expression(0),
            },
            TokenKind::IdentifierToken(name) => {
                if let Some((operator, length)) = self.match_operator(1) {
                    if let AssignmentOperator(_) = operator {
                        for _ in 0..length {
                            self.lexer.advance();
                        }
                        let expression = self.parse_assignment_expression()?;

                        Ok(AbstractSyntaxTree::AssignmentExpression(
                            Box::new(AbstractSyntaxTree::IdentifierExpression(name.to_string())),
                            operator,
                            Box::new(expression),
                        ))
                    } else {
                        self.parse_arithmetic_expression(0)
                    }
                } else {
                    self.parse_arithmetic_expression(0)
                }
            }
            _ => self.parse_arithmetic_expression(0),
            // TokenKind::LiteralToken(_) => todo!(),
            // TokenKind::WhitespaceToken(_) => todo!(),
            // TokenKind::NewLineToken => todo!(),
            // TokenKind::SymbolToken(_) => todo!(),
            // TokenKind::FactoryToken => todo!(),
            // TokenKind::EndOfFileToken => todo!(),
        }
    }

    fn parse_arithmetic_expression(
        &self,
        parent_precedence: u8,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut left = if let Some((operator, _)) = self.match_operator(0) {
            if operator.get_unery_precedence() >= parent_precedence {
                self.lexer.advance();
                let expression =
                    self.parse_arithmetic_expression(operator.get_unery_precedence())?;
                AbstractSyntaxTree::UnaryExpression(operator, Box::new(expression))
            } else {
                self.parse_factor()?
            }
        } else {
            self.parse_factor()?
        };

        while let Some((operator, length)) = self.match_operator(0) {
            let precedence = operator.get_binary_precedence();
            if precedence <= parent_precedence {
                break;
            }
            for _ in 0..length {
                self.lexer.advance();
            }
            let right = self.parse_arithmetic_expression(precedence)?;
            left = AbstractSyntaxTree::BinaryExpression(Box::new(left), operator, Box::new(right));
        }

        Ok(left)
    }

    fn parse_factor(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        let token = self.lexer.get_current_token_and_advance();
        match &token.kind {
            TokenKind::LiteralToken(variable) => {
                Ok(AbstractSyntaxTree::LiteralExpression(variable.clone()))
            }
            TokenKind::SymbolToken(symbol) => match symbol {
                OpenParanthesis => {
                    let expression = self.parse_expression()?;
                    let next_token = self.lexer.get_current_token_and_advance();
                    if next_token.kind == TokenKind::SymbolToken(CloseParanthesis) {
                        Ok(AbstractSyntaxTree::ParenthesizedExpression(Box::new(
                            expression,
                        )))
                    } else {
                        Err(CompilerError::UnexpectedToken(
                            TokenKind::SymbolToken(CloseParanthesis),
                            token.line,
                            token.column,
                        ))
                    }
                }
                CloseParanthesis => Err(CompilerError::UnexpectedToken(
                    TokenKind::SymbolToken(CloseParanthesis),
                    token.line,
                    token.column,
                )),
                _ => todo!("parse_factor"),
            },
            TokenKind::IdentifierToken(name) => {
                Ok(AbstractSyntaxTree::IdentifierExpression(name.clone()))
            }
            kind => Err(CompilerError::UnexpectedToken(
                kind.clone(),
                token.line,
                token.column,
            )),
        }
    }

    fn handle_nullable_keyword(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        if let TokenKind::IdentifierToken(variable_name) = &self.lexer.peek(1).kind {
            if let Some((operator, length)) = self.match_operator(2) {
                // nullable variable_name operator expression
                for _ in 0..length {
                    self.lexer.advance();
                }
                let expression = self.parse_assignment_expression()?;

                handle_nullable_assignment(variable_name, operator, expression)
            } else {
                // nullable variable_name
                self.lexer.advance();
                self.lexer.advance();
                handle_nullable_declaration(variable_name)
            }
        } else {
            // nullable
            Err(CompilerError::InvalidUseOfNullableKeyword)
        }
    }

    fn handle_mutable_keyword(&self) -> Result<AbstractSyntaxTree, CompilerError> {
        if let TokenKind::IdentifierToken(variable_name) = &self.lexer.peek(1).kind {
            if let Some((operator, length)) = self.match_operator(2) {
                // `mutable` variable_name operator expression
                for _ in 0..length {
                    self.lexer.advance();
                }
                let expression = self.parse_assignment_expression()?;
                handle_mutable_assignment(variable_name, operator, expression)
            } else {
                // `mutable` variable_name
                if SymbolTable::contains(variable_name) {
                    return Err(CompilerError::CannotConvertFromImmutableToMutable);
                } else {
                    Err(CompilerError::NullAssignmentOfNonNullableVariable)
                }
            }
        } else {
            Err(CompilerError::InvalidUseOfMutableKeyword)
        }
    }

    fn match_operator(&self, offset: usize) -> Option<(Operator, usize)> {
        // TODO: use match instead of if let to include keyword operators
        // like 'and', 'or', 'not', 'xor', 'is', 'in', 'not in', 'is not'
        let (operator, length) = if let TokenKind::SymbolToken(operator_symbol) =
            self.lexer.peek(offset).kind
        {
            match operator_symbol {
                Equals => {
                    // =
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // ==
                            (RelationalOperator(Equality), 2)
                        } else {
                            // =
                            (AssignmentOperator(SimpleAssignment), 1)
                        }
                    } else {
                        // =
                        (AssignmentOperator(SimpleAssignment), 1)
                    }
                }
                Plus => {
                    // +
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // +=
                            (AssignmentOperator(AdditionAssignment), 2)
                        } else {
                            // +
                            (ArithmeticOperator(Addition), 1)
                        }
                    } else {
                        // +
                        (ArithmeticOperator(Addition), 1)
                    }
                }
                Minus => {
                    // -
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // -=
                            (AssignmentOperator(SubtractionAssignment), 2)
                        } else {
                            // -
                            (ArithmeticOperator(Subtraction), 1)
                        }
                    } else {
                        // -
                        (ArithmeticOperator(Subtraction), 1)
                    }
                }
                Asterisk => {
                    // *
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // *=
                            (AssignmentOperator(MultiplicationAssignment), 2)
                        } else if second_symbol == Asterisk {
                            // **
                            if let TokenKind::SymbolToken(third_token) =
                                self.lexer.peek(offset + 2).kind
                            {
                                if third_token == Equals {
                                    // **=
                                    (AssignmentOperator(ExponentiationAssignment), 3)
                                } else {
                                    // **
                                    (ArithmeticOperator(Exponentiation), 2)
                                }
                            } else {
                                // **
                                (ArithmeticOperator(Exponentiation), 2)
                            }
                        } else {
                            // *
                            (ArithmeticOperator(Multiplication), 1)
                        }
                    } else {
                        // *
                        (ArithmeticOperator(Multiplication), 1)
                    }
                }
                Slash => {
                    // /
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // /=
                            (AssignmentOperator(DivisionAssignment), 2)
                        } else {
                            // /
                            (ArithmeticOperator(Division), 1)
                        }
                    } else {
                        // /
                        (ArithmeticOperator(Division), 1)
                    }
                }
                Percent => {
                    // %
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // %=
                            (AssignmentOperator(ModuloAssignment), 2)
                        } else {
                            // %
                            (ArithmeticOperator(Modulo), 1)
                        }
                    } else {
                        // %
                        (ArithmeticOperator(Modulo), 1)
                    }
                }
                Exclamation => {
                    // !
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // !=
                            (RelationalOperator(InEquality), 2)
                        } else {
                            // !
                            (LogicalOperator(Logical::Not), 1)
                        }
                    } else {
                        // !
                        (LogicalOperator(Logical::Not), 1)
                    }
                }
                crate::lexical_analysis::symbols::Symbol::GreaterThan => {
                    // >
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // >=
                            (RelationalOperator(GreaterThanOrEquals), 2)
                        } else {
                            // >
                            (
                                RelationalOperator(
                                    crate::common::operators::relational::Relational::GreaterThan,
                                ),
                                1,
                            )
                        }
                    } else {
                        // >
                        (
                            RelationalOperator(
                                crate::common::operators::relational::Relational::GreaterThan,
                            ),
                            1,
                        )
                    }
                }
                crate::lexical_analysis::symbols::Symbol::LessThan => {
                    // <
                    if let TokenKind::SymbolToken(second_symbol) = self.lexer.peek(offset + 1).kind
                    {
                        if second_symbol == Equals {
                            // <=
                            (RelationalOperator(LessThanOrEquals), 2)
                        } else {
                            // <
                            (
                                RelationalOperator(
                                    crate::common::operators::relational::Relational::LessThan,
                                ),
                                1,
                            )
                        }
                    } else {
                        // <
                        (
                            RelationalOperator(
                                crate::common::operators::relational::Relational::LessThan,
                            ),
                            1,
                        )
                    }
                }
                _ => {
                    return None;
                }
            }
        } else if let TokenKind::KeywordToken(keyword) = &self.lexer.peek(offset).kind {
            match keyword {
                Keyword::Is => {
                    if let TokenKind::KeywordToken(keyword) = &self.lexer.peek(offset + 1).kind {
                        if Keyword::Not == *keyword {
                            (RelationalOperator(InEquality), 2)
                        } else {
                            (RelationalOperator(Equality), 1)
                        }
                    } else {
                        (RelationalOperator(Equality), 1)
                    }
                }
                Keyword::And => (LogicalOperator(Logical::And), 1),
                Keyword::Or => (LogicalOperator(Logical::Or), 1),
                Keyword::Not => (LogicalOperator(Logical::Not), 1),
                Keyword::Xor => (LogicalOperator(Logical::Xor), 1),
                _ => {
                    return None;
                }
            }
        } else {
            return None;
        };
        Some((operator, offset + length))
    }
}

fn handle_mutable_assignment(
    variable_name: &String,
    operator: Operator,
    expression: AbstractSyntaxTree,
) -> Result<AbstractSyntaxTree, CompilerError> {
    match operator {
        AssignmentOperator(SimpleAssignment) => {
            if SymbolTable::contains(variable_name) {
                let old_variable = SymbolTable::get(variable_name).unwrap();
                if old_variable.is_mutable() {
                    // `mutable` variable_name = old_expression
                    // `mutable` variable_name = new_expression
                    // TODO: add diagnostics. saying
                    // you don't need to use mutable keyword twice, once it is declared as mutable it will be mutable forever
                    // TODO: use Internal undefined instead of null
                    SymbolTable::add(
                        variable_name.to_string(),
                        Variable::new_mutable(DataType::Null),
                    );
                } else {
                    // variable_name = old_expression
                    // `mutable` variable_name = new_expression
                    return Err(CompilerError::CannotConvertFromImmutableToMutable);
                }
            } else {
                // `mutable` variable_name = expression
                SymbolTable::add(
                    variable_name.to_string(),
                    Variable::new_mutable(DataType::Null),
                );
            }

            Ok(AbstractSyntaxTree::AssignmentExpression(
                Box::new(AbstractSyntaxTree::IdentifierExpression(
                    variable_name.to_string(),
                )),
                operator,
                Box::new(expression),
            ))
        }
        // mutable a += 10
        AssignmentOperator(_) => {
            if SymbolTable::contains(variable_name) {
                let old_variable = SymbolTable::get(variable_name).unwrap();
                if old_variable.is_mutable() {
                    // `mutable` variable_name operator old_expression
                    // `mutable` variable_name assignment_operator new expression
                    // TODO: add diagnostics. saying
                    // you don't need to use mutable keyword twice, once it is declared as mutable it will be mutable forever
                    // TODO: use Internal undefined instead of null
                    SymbolTable::add(
                        variable_name.to_string(),
                        Variable::new_mutable(DataType::Null),
                    );
                } else {
                    // variable_name operator expression
                    // `mutable` variable_name assignment_operator new expression
                    return Err(CompilerError::CannotConvertFromImmutableToMutable);
                }
                SymbolTable::add(
                    variable_name.to_string(),
                    Variable::new_mutable(SymbolTable::get(variable_name).unwrap().value),
                );
            } else {
                return Err(CompilerError::UndefinedVariable(variable_name.to_string()));
            }
            Ok(AbstractSyntaxTree::AssignmentExpression(
                Box::new(AbstractSyntaxTree::IdentifierExpression(
                    variable_name.to_string(),
                )),
                operator,
                Box::new(expression),
            ))
        }
        _ => Err(CompilerError::InvalidOperationAsAssignmentOperation),
    }
}

fn handle_nullable_declaration(
    variable_name: &String,
) -> Result<AbstractSyntaxTree, CompilerError> {
    if SymbolTable::contains(variable_name) {
        let variable = SymbolTable::get(variable_name).unwrap().as_nullable();
        SymbolTable::add(variable_name.to_string(), variable);
    } else {
        SymbolTable::add(
            variable_name.to_string(),
            Variable::new_nullable(DataType::Null),
        );
    }
    Ok(AbstractSyntaxTree::IdentifierExpression(
        variable_name.to_string(),
    ))
}

fn handle_nullable_assignment(
    variable_name: &String,
    operator: Operator,
    expression: AbstractSyntaxTree,
) -> Result<AbstractSyntaxTree, CompilerError> {
    match operator {
        AssignmentOperator(SimpleAssignment) => {
            if SymbolTable::contains(variable_name) {
                let old_variable = SymbolTable::get(variable_name).unwrap();
                if old_variable.is_nullable() {
                    // nullable variable_name oeprator expression
                    // nullable variable_name = expression
                    // TODO: add diagnostics. saying
                    // you don't need to use nullable keyword twice, once it is declared as nullable it will be nullable forever
                }
                // variable_name oeprator expression
                // nullable variable_name = expression

                let variable = SymbolTable::get(variable_name).unwrap().as_nullable();
                SymbolTable::add(variable_name.to_string(), variable);
            } else {
                // nullable variable_name = expression
                // TODO: use Internal undefined instead of null
                SymbolTable::add(
                    variable_name.to_string(),
                    Variable::new_nullable(DataType::Null),
                );
            }

            Ok(AbstractSyntaxTree::AssignmentExpression(
                Box::new(AbstractSyntaxTree::IdentifierExpression(
                    variable_name.to_string(),
                )),
                operator,
                Box::new(expression),
            ))
        }
        // may be need some work here
        _ => Err(CompilerError::OperationOnNull),
    }
}
