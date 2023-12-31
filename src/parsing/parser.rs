use std::rc::Rc;
use std::sync::{Arc, RwLock};

use super::block::Block;
use super::seperated_statements::SeperatedStatements;
use crate::common::datatypes::DataType;
use crate::common::diagnostics::Diagnostics;
use crate::common::errors::CompilerError;
use crate::common::functions::Function;
use crate::common::literal::Literal;
use crate::common::operators::arithmetic::Arithmetic;
use crate::common::operators::assignment::Assingment;
use crate::common::operators::logical::Logical;
use crate::common::operators::relational::Relational;
use crate::common::operators::Operator;
use crate::common::operators::Operator::*;
use crate::lexing::keywords::Keyword;
use crate::lexing::lexer::Lexer;
use crate::lexing::symbols::Symbol::*;
use crate::lexing::token::{Token, TokenKind};
use crate::parsing::ast::AbstractSyntaxTree;

pub(crate) struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub(crate) fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub(crate) fn parse(&mut self) -> Result<Arc<RwLock<Block>>, CompilerError> {
        if self.lexer.get_token_count() == 0 {
            self.lexer.lex()?;
        }
        let global_block = Arc::new(RwLock::new(Block::new()));
        while TokenKind::EndOfFile != self.lexer.get_current_token().kind {
            let statement = self.parse_statement(Arc::clone(&global_block))?;
            global_block.write().unwrap().statements.push(statement);
        }
        Ok(global_block)
    }

    fn parse_statement(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let token = self.lexer.get_current_token();
        match &token.kind {
            TokenKind::Symbol(OpenParanthesis) => self.parse_function_statement(block),
            TokenKind::Symbol(OpenCurlyBracket) => {
                Ok(AbstractSyntaxTree::BlockStatement(self.parse_block(block)?))
            }
            TokenKind::Keyword(Keyword::Loop) => self.parse_loop_statement(block),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(block),
            TokenKind::Keyword(Keyword::Break) => self.parse_break_statement(block),
            TokenKind::Keyword(Keyword::Skip) => self.parse_skip_statement(block),
            _ => self.parse_expression(block),
        }
    }

    fn parse_break_statement(
        &self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.lexer.advance();
        if let TokenKind::Symbol(symbol) = self.lexer.get_current_token().kind {
            if symbol == Semicolon {
                self.lexer.advance();
                return Ok(AbstractSyntaxTree::BreakStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(false)),
                )));
            }
        }
        let returnable = self.parse_statement(parent)?;
        Ok(AbstractSyntaxTree::ReturnStatement(Box::new(returnable)))
    }

    fn parse_return_statement(
        &self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.lexer.advance();
        if let TokenKind::Symbol(symbol) = self.lexer.get_current_token().kind {
            if symbol == Semicolon {
                self.lexer.advance();
                return Ok(AbstractSyntaxTree::ReturnStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(false)),
                )));
            }
        }
        let returnable = self.parse_statement(parent)?;
        Ok(AbstractSyntaxTree::ReturnStatement(Box::new(returnable)))
    }

    fn parse_skip_statement(
        &self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.lexer.advance();
        if let TokenKind::Symbol(symbol) = self.lexer.get_current_token().kind {
            if symbol == Semicolon {
                self.lexer.advance();
                return Ok(AbstractSyntaxTree::SkipStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(1)),
                )));
            }
        }
        let skip_count = self.parse_expression(parent)?;
        Ok(AbstractSyntaxTree::SkipStatement(Box::new(skip_count)))
    }

    fn parse_block(&self, parent: Arc<RwLock<Block>>) -> Result<Arc<RwLock<Block>>, CompilerError> {
        let block = Arc::new(RwLock::new(Block::from(parent)));
        self.lexer.advance();
        while TokenKind::Symbol(CloseCurlyBracket) != self.lexer.get_current_token().kind
            && TokenKind::EndOfFile != self.lexer.get_current_token().kind
        {
            let statement = self.parse_statement(Arc::clone(&block))?;
            block.write().unwrap().statements.push(statement);
        }
        if TokenKind::Symbol(CloseCurlyBracket) != self.lexer.get_current_token_and_advance().kind {
            return Err(CompilerError::UnexpectedTokenWithExpected(
                self.lexer.get_current_token().kind.clone(),
                TokenKind::Symbol(CloseCurlyBracket),
                self.lexer.get_current_token().line,
                self.lexer.get_current_token().column,
            ));
        }
        Ok(block)
    }

    fn parse_if_statement(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let condition = self.parse_expression(Arc::clone(&block))?;
        let if_block = self.parse_statement(Arc::clone(&block))?;
        let else_block = if TokenKind::Keyword(Keyword::Else) == self.lexer.get_current_token().kind
        {
            self.lexer.advance();
            Some(Box::new(self.parse_else_block(Arc::clone(&block))?))
        } else {
            None
        };
        Ok(AbstractSyntaxTree::IfStatement(
            Box::new(condition),
            Box::new(if_block),
            else_block,
        ))
    }

    fn parse_else_block(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        Ok(AbstractSyntaxTree::ElseStatement(Box::new(
            self.parse_statement(Arc::clone(&block))?,
        )))
    }

    fn parse_loop_statement(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.lexer.advance();
        let mut condition = AbstractSyntaxTree::Literal(Literal::from(true));

        if TokenKind::Keyword(Keyword::While) == self.lexer.get_current_token().kind {
            self.lexer.advance();
            condition = self.parse_expression(Arc::clone(&block))?;
        }

        if TokenKind::Keyword(Keyword::Until) == self.lexer.get_current_token().kind {
            self.lexer.advance();
            condition = self.parse_expression(Arc::clone(&block))?;
            condition = AbstractSyntaxTree::UnaryExpression(Operator::Logical(Logical::Not), Box::new(condition));
        }

        let previous_state = block.read().unwrap().is_loop;
        block.write().unwrap().is_loop = true;
        let block_to_execute = self.parse_statement(Arc::clone(&block))?;
        block.write().unwrap().is_loop = previous_state;

        Ok(AbstractSyntaxTree::LoopStatement(
            Box::new(condition),
            Box::new(block_to_execute),
        ))
    }

    fn parse_function_statement(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut parameters: Vec<AbstractSyntaxTree> = Vec::new();
        let mut count = 1;
        while TokenKind::Symbol(CloseParanthesis) != self.lexer.peek(count).kind
            && TokenKind::EndOfFile != self.lexer.peek(count).kind
        {
            let current = self.lexer.peek(count);
            if let TokenKind::Identifier(name) = &current.kind {
                let parameter = AbstractSyntaxTree::Identifier(name.to_string());
                parameters.push(parameter);
            } else {
                return self.parse_expression(block);
            }
            count += 1;

            if TokenKind::Symbol(CloseParanthesis) != self.lexer.peek(count).kind
                && TokenKind::Symbol(Comma) != self.lexer.peek(count).kind
            {
                return self.parse_expression(block);
            } else if TokenKind::Symbol(Comma) == self.lexer.peek(count).kind {
                count += 1;
            }
        }
        for _ in 0..=count {
            self.lexer.advance();
        }
        let current = self.lexer.get_current_token();
        if TokenKind::Symbol(Equals) != current.kind
            && TokenKind::Symbol(GreaterThan) != self.lexer.peek(1).kind
        {
            return Err(CompilerError::MissingArrow(
                current.kind.clone(),
                current.line,
                current.column,
            ));
        }
        self.lexer.advance();
        self.lexer.advance();
        let previous_state = block.read().unwrap().is_function;
        block.write().unwrap().is_function = true;
        let function_block_ast = self.parse_statement(Arc::clone(&block))?;
        block.write().unwrap().is_function = previous_state;
        let function_block = if let Ok(block) = function_block_ast.to_block() {
            block
        } else {
            let mut current_block = Block::from(vec![function_block_ast]);
            current_block.parent = Some(block);
            current_block.is_function = true;
            Arc::new(RwLock::new(current_block))
        };
        let parameters = SeperatedStatements::new(Comma, OpenParanthesis, parameters);
        let function = Function::new(function_block, parameters);
        let function = DataType::Function(Arc::new(function));
        let function = Literal::from(function);
        Ok(AbstractSyntaxTree::Literal(function))
    }

    fn parse_function_call_statement(
        &self,
        name: String,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.lexer.advance();
        let mut arguments: Vec<Box<AbstractSyntaxTree>> = Vec::new();
        while TokenKind::Symbol(CloseParanthesis) != self.lexer.get_current_token().kind
            && TokenKind::EndOfFile != self.lexer.get_current_token().kind
        {
            let expression = self.parse_expression(Arc::clone(&block))?;
            arguments.push(Box::new(expression));
            let current = self.lexer.get_current_token();
            if TokenKind::Symbol(CloseParanthesis) != current.kind
                && TokenKind::Symbol(Comma) != current.kind
            {
                return Err(CompilerError::UnexpectedTokenWithExpected(
                    current.kind.clone(),
                    TokenKind::Symbol(Comma),
                    current.line,
                    current.column,
                ));
            }
            if TokenKind::Symbol(Comma) == current.kind {
                self.lexer.advance();
            }
        }

        self.match_token(TokenKind::Symbol(CloseParanthesis))?;

        let arguments = SeperatedStatements::new(Comma, OpenParanthesis, arguments);
        Ok(AbstractSyntaxTree::CallStatement(name, arguments))
    }

    fn parse_expression(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.parse_assignment_expression(block)
    }

    fn parse_assignment_expression(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let identifier_token = self.lexer.get_current_token();
        match &identifier_token.kind {
            TokenKind::Keyword(Keyword::Let) => self.handle_mutable_keyword(block),
            TokenKind::Identifier(name) => {
                if let Some((operator, length)) = self.match_operator(1) {
                    if let Assignment(_) = operator {
                        for _ in 0..length {
                            self.lexer.advance();
                        }
                        let expression = self.parse_statement(block)?;

                        Ok(AbstractSyntaxTree::AssignmentExpression(
                            name.to_string(),
                            operator,
                            Box::new(expression),
                        ))
                    } else {
                        self.parse_arithmetic_expression(0, block)
                    }
                } else {
                    self.parse_arithmetic_expression(0, block)
                }
            }
            _ => self.parse_arithmetic_expression(0, block),
        }
    }

    fn parse_arithmetic_expression(
        &self,
        parent_precedence: u8,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut left = if let Some((operator, _)) = self.match_operator(0) {
            if operator.get_unery_precedence() >= parent_precedence {
                self.lexer.advance();
                let expression = self.parse_arithmetic_expression(
                    operator.get_unery_precedence(),
                    Arc::clone(&block),
                )?;
                AbstractSyntaxTree::UnaryExpression(operator, Box::new(expression))
            } else {
                self.parse_factor(Arc::clone(&block))?
            }
        } else {
            self.parse_factor(Arc::clone(&block))?
        };

        while let Some((operator, length)) = self.match_operator(0) {
            let precedence = operator.get_binary_precedence();
            if precedence <= parent_precedence {
                break;
            }
            for _ in 0..length {
                self.lexer.advance();
            }
            let right = self.parse_arithmetic_expression(precedence, Arc::clone(&block))?;
            left = AbstractSyntaxTree::BinaryExpression(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn match_token(&self, kind: TokenKind) -> Result<Rc<Token>, CompilerError> {
        let token = self.lexer.get_current_token();
        self.lexer.advance();
        if kind == token.kind {
            Ok(token)
        } else {
            Err(CompilerError::UnexpectedTokenWithExpected(
                token.kind.clone(),
                kind,
                token.line,
                token.column,
            ))
        }
    }

    fn parse_factor(&self, block: Arc<RwLock<Block>>) -> Result<AbstractSyntaxTree, CompilerError> {
        let token = self.lexer.get_current_token_and_advance();
        match &token.kind {
            TokenKind::Literal(variable) => Ok(AbstractSyntaxTree::Literal(variable.clone())),
            TokenKind::Symbol(symbol) => match symbol {
                OpenParanthesis => {
                    let expression = self.parse_arithmetic_expression(0, block)?;
                    self.match_token(TokenKind::Symbol(CloseParanthesis))?;
                    Ok(AbstractSyntaxTree::ParenthesizedExpression(Box::new(
                        expression,
                    )))
                }
                symbol => Err(CompilerError::UnexpectedToken(
                    TokenKind::Symbol(*symbol),
                    token.line,
                    token.column,
                )),
            },
            TokenKind::Identifier(name) => {
                if TokenKind::Symbol(OpenParanthesis) == self.lexer.get_current_token().kind {
                    self.parse_function_call_statement(name.to_string(), block)
                } else {
                    Ok(AbstractSyntaxTree::Identifier(name.clone()))
                }
            }
            TokenKind::Keyword(Keyword::If) => self.parse_if_statement(block),
            kind => Err(CompilerError::UnexpectedToken(
                kind.clone(),
                token.line,
                token.column,
            )),
        }
    }

    fn handle_mutable_keyword(
        &self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        if let TokenKind::Identifier(variable_name) = &self.lexer.peek(1).kind {
            if let Some((operator, length)) = self.match_operator(2) {
                // `mutable` variable_name operator expression
                for _ in 0..length {
                    self.lexer.advance();
                }
                let expression = self.parse_assignment_expression(Arc::clone(&block))?;
                handle_mutable_assignment(variable_name, operator, expression, block)
            } else {
                // `mutable` variable_name
                if block.read().unwrap().contains_symbol(variable_name) {
                    Err(CompilerError::CannotConvertFromImmutableToMutable)
                } else {
                    Err(CompilerError::UnInitializedVariable(
                        variable_name.to_string(),
                    ))
                }
            }
        } else {
            Err(CompilerError::InvalidUseOfMutableKeyword)
        }
    }

    fn match_operator(&self, offset: usize) -> Option<(Operator, usize)> {
        // TODO: use match instead of if let to include keyword operators
        // like 'and', 'or', 'not', 'xor', 'is', 'in', 'not in', 'is not'
        let (operator, length) = if let TokenKind::Symbol(operator_symbol) =
            self.lexer.peek(offset).kind
        {
            match operator_symbol {
                Equals => {
                    // =
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // ==
                            (Relational(Relational::Equality), 2)
                        } else {
                            // =
                            (Assignment(Assingment::Simple), 1)
                        }
                    } else {
                        // =
                        (Assignment(Assingment::Simple), 1)
                    }
                }
                Plus => {
                    // +
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // +=
                            (Assignment(Assingment::Addition), 2)
                        } else {
                            // +
                            (Arithmetic(Arithmetic::Addition), 1)
                        }
                    } else {
                        // +
                        (Arithmetic(Arithmetic::Addition), 1)
                    }
                }
                Minus => {
                    // -
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // -=
                            (Assignment(Assingment::Subtraction), 2)
                        } else {
                            // -
                            (Arithmetic(Arithmetic::Subtraction), 1)
                        }
                    } else {
                        // -
                        (Arithmetic(Arithmetic::Subtraction), 1)
                    }
                }
                Asterisk => {
                    // *
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // *=
                            (Assignment(Assingment::Multiplication), 2)
                        } else if second_symbol == Asterisk {
                            // **
                            if let TokenKind::Symbol(third_token) = self.lexer.peek(offset + 2).kind
                            {
                                if third_token == Equals {
                                    // **=
                                    (Assignment(Assingment::Exponentiation), 3)
                                } else {
                                    // **
                                    (Arithmetic(Arithmetic::Exponentiation), 2)
                                }
                            } else {
                                // **
                                (Arithmetic(Arithmetic::Exponentiation), 2)
                            }
                        } else {
                            // *
                            (Arithmetic(Arithmetic::Multiplication), 1)
                        }
                    } else {
                        // *
                        (Arithmetic(Arithmetic::Multiplication), 1)
                    }
                }
                Slash => {
                    // /
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // /=
                            (Assignment(Assingment::Division), 2)
                        } else {
                            // /
                            (Arithmetic(Arithmetic::Division), 1)
                        }
                    } else {
                        // /
                        (Arithmetic(Arithmetic::Division), 1)
                    }
                }
                Percent => {
                    // %
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // %=
                            (Assignment(Assingment::Modulo), 2)
                        } else {
                            // %
                            (Arithmetic(Arithmetic::Modulo), 1)
                        }
                    } else {
                        // %
                        (Arithmetic(Arithmetic::Modulo), 1)
                    }
                }
                Exclamation => {
                    // !
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // !=
                            (Relational(Relational::InEquality), 2)
                        } else {
                            // !
                            (Logical(Logical::Not), 1)
                        }
                    } else {
                        // !
                        (Logical(Logical::Not), 1)
                    }
                }
                GreaterThan => {
                    // >
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // >=
                            (Relational(Relational::GreaterThanOrEquals), 2)
                        } else {
                            // >
                            (Relational(Relational::GreaterThan), 1)
                        }
                    } else {
                        // >
                        (Relational(Relational::GreaterThan), 1)
                    }
                }
                LessThan => {
                    // <
                    if let TokenKind::Symbol(second_symbol) = self.lexer.peek(offset + 1).kind {
                        if second_symbol == Equals {
                            // <=
                            (Relational(Relational::LessThanOrEquals), 2)
                        } else {
                            // <
                            (
                                Relational(
                                    crate::common::operators::relational::Relational::LessThan,
                                ),
                                1,
                            )
                        }
                    } else {
                        // <
                        (
                            Relational(crate::common::operators::relational::Relational::LessThan),
                            1,
                        )
                    }
                }
                _ => {
                    return None;
                }
            }
        } else if let TokenKind::Keyword(keyword) = &self.lexer.peek(offset).kind {
            match keyword {
                Keyword::Is => {
                    if let TokenKind::Keyword(keyword) = &self.lexer.peek(offset + 1).kind {
                        if Keyword::Not == *keyword {
                            (Relational(Relational::Equality), 2)
                        } else {
                            (Relational(Relational::Equality), 1)
                        }
                    } else {
                        (Relational(Relational::Equality), 1)
                    }
                }
                Keyword::And => (Logical(Logical::And), 1),
                Keyword::Or => (Logical(Logical::Or), 1),
                Keyword::Not => (Logical(Logical::Not), 1),
                Keyword::Xor => (Logical(Logical::Xor), 1),
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
    variable_name: &str,
    operator: Operator,
    expression: AbstractSyntaxTree,
    block: Arc<RwLock<Block>>,
) -> Result<AbstractSyntaxTree, CompilerError> {
    let block = block.read().unwrap();
    match operator {
        Assignment(Assingment::Simple) => {
            if block.contains_symbol(variable_name) {
                let old_variable = block.get_symbol(variable_name).unwrap();
                if old_variable.is_mutable() {
                    // `mutable` variable_name = old_expression
                    // `mutable` variable_name = new_expression
                    Diagnostics::add_error(CompilerError::Warnings("You don't need to use mutable keyword twice, once it is declared as mutable it will be mutable forever"));
                    block.add_symbol(
                        variable_name.to_string(),
                        Literal::new_mutable(DataType::InternalUndefined),
                    );
                } else {
                    // variable_name = old_expression
                    // `mutable` variable_name = new_expression
                    return Err(CompilerError::CannotConvertFromImmutableToMutable);
                }
            } else {
                // `mutable` variable_name = expression
                block.add_symbol(
                    variable_name.to_string(),
                    Literal::new_mutable(DataType::InternalUndefined),
                );
            }

            Ok(AbstractSyntaxTree::AssignmentExpression(
                variable_name.to_string(),
                operator,
                Box::new(expression),
            ))
        }
        // mutable a += 10
        Assignment(_) => {
            if block.contains_symbol(variable_name) {
                let old_variable = block.get_symbol(variable_name).unwrap();
                if old_variable.is_mutable() {
                    // `mutable` variable_name operator old_expression
                    // `mutable` variable_name assignment_operator new expression
                    Diagnostics::add_error(CompilerError::Warnings("You don't need to use mutable keyword twice, once it is declared as mutable it will be mutable forever"));
                    block.add_symbol(
                        variable_name.to_string(),
                        Literal::new_mutable(DataType::InternalUndefined),
                    );
                } else {
                    // variable_name operator expression
                    // `mutable` variable_name assignment_operator new expression
                    return Err(CompilerError::CannotConvertFromImmutableToMutable);
                }
                block.add_symbol(
                    variable_name.to_string(),
                    Literal::new_mutable(block.get_symbol(variable_name).unwrap().value),
                );
            } else {
                return Err(CompilerError::UndefinedVariable(variable_name.to_string()));
            }
            Ok(AbstractSyntaxTree::AssignmentExpression(
                variable_name.to_string(),
                operator,
                Box::new(expression),
            ))
        }
        _ => Err(CompilerError::InvalidOperationAsAssignmentOperation),
    }
}
