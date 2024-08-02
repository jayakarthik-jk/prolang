use std::sync::mpsc::{Receiver, Sender};
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
use crate::lexing::symbols::Symbol::*;
use crate::lexing::token::{Token, TokenKind};
use crate::parsing::ast::AbstractSyntaxTree;

pub(crate) struct Parser {
    token_receiver: Receiver<Token>,
    statement_transmitter: Sender<AbstractSyntaxTree>,
    global_block: Arc<RwLock<Block>>,
    temp_token_buf: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(
        token_receiver: Receiver<Token>,
        statement_transmitter: Sender<AbstractSyntaxTree>,
        global_block: Arc<RwLock<Block>>,
    ) -> Self {
        Self {
            token_receiver,
            statement_transmitter,
            global_block,
            temp_token_buf: Vec::new(),
        }
    }

    fn get_current_token(&mut self) -> Token {
        if let Some(token) = self.temp_token_buf.pop() {
            token
        } else {
            self.token_receiver
                .recv()
                .unwrap_or_else(|_| Token::new(TokenKind::EndOfFile, 0, 0))
        }
    }

    pub(crate) fn parse(mut self) {
        let mut current = self.get_current_token();
        while TokenKind::EndOfFile != current.kind {
            self.temp_token_buf.push(current);
            let statement = match self.parse_statement(Arc::clone(&self.global_block)) {
                Ok(statement) => statement,
                Err(err) => {
                    eprintln!("{err}");
                    panic!("{err}")
                }
            };
            self.statement_transmitter.send(statement).unwrap();
            current = self.get_current_token();
        }
    }

    fn parse_statement(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let token = self.get_current_token();
        match &token.kind {
            TokenKind::Symbol(OpenParanthesis) => {
                self.temp_token_buf.push(token);
                self.parse_function_statement(block)
            }
            TokenKind::Symbol(OpenCurlyBracket) => {
                Ok(AbstractSyntaxTree::BlockStatement(self.parse_block(block)?))
            }
            TokenKind::Keyword(Keyword::Loop) => self.parse_loop_statement(block),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(block),
            TokenKind::Keyword(Keyword::Break) => self.parse_break_statement(block),
            TokenKind::Keyword(Keyword::Skip) => self.parse_skip_statement(block),
            _ => {
                self.temp_token_buf.push(token);
                self.parse_expression(block)
            }
        }
    }

    fn parse_break_statement(
        &mut self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let current = self.get_current_token();
        if let TokenKind::Symbol(symbol) = current.kind {
            if symbol == Semicolon {
                return Ok(AbstractSyntaxTree::BreakStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(false)),
                )));
            }
        }
        self.temp_token_buf.push(current);
        let returnable = self.parse_statement(parent)?;
        Ok(AbstractSyntaxTree::ReturnStatement(Box::new(returnable)))
    }

    fn parse_return_statement(
        &mut self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let current = self.get_current_token();
        if let TokenKind::Symbol(symbol) = current.kind {
            if symbol == Semicolon {
                return Ok(AbstractSyntaxTree::ReturnStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(false)),
                )));
            }
        }
        self.temp_token_buf.push(current);
        let returnable = self.parse_statement(parent)?;
        Ok(AbstractSyntaxTree::ReturnStatement(Box::new(returnable)))
    }

    fn parse_skip_statement(
        &mut self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let current = self.get_current_token();
        if let TokenKind::Symbol(symbol) = current.kind {
            if symbol == Semicolon {
                return Ok(AbstractSyntaxTree::SkipStatement(Box::new(
                    AbstractSyntaxTree::Literal(Literal::from(1)),
                )));
            }
        }
        self.temp_token_buf.push(current);
        let skip_count = self.parse_expression(parent)?;
        Ok(AbstractSyntaxTree::SkipStatement(Box::new(skip_count)))
    }

    fn parse_block(
        &mut self,
        parent: Arc<RwLock<Block>>,
    ) -> Result<Arc<RwLock<Block>>, CompilerError> {
        let block = Arc::new(RwLock::new(Block::from(parent)));
        let mut current = self.get_current_token();
        while TokenKind::Symbol(CloseCurlyBracket) != current.kind
            && TokenKind::EndOfFile != current.kind
        {
            self.temp_token_buf.push(current);
            let statement = self.parse_statement(Arc::clone(&block))?;
            block.write().unwrap().statements.push(statement);
            current = self.get_current_token();
        }
        if TokenKind::Symbol(CloseCurlyBracket) != current.kind {
            return Err(CompilerError::UnexpectedTokenWithExpected(
                current.kind,
                TokenKind::Symbol(CloseCurlyBracket),
                current.line,
                current.column,
            ));
        }
        Ok(block)
    }

    fn parse_if_statement(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let condition = self.parse_expression(Arc::clone(&block))?;
        let if_block = self.parse_statement(Arc::clone(&block))?;
        let current = self.get_current_token();
        let else_block = if TokenKind::Keyword(Keyword::Else) == current.kind {
            Some(Box::new(self.parse_else_block(Arc::clone(&block))?))
        } else {
            self.temp_token_buf.push(current);
            None
        };
        Ok(AbstractSyntaxTree::IfStatement(
            Box::new(condition),
            Box::new(if_block),
            else_block,
        ))
    }

    fn parse_else_block(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        Ok(AbstractSyntaxTree::ElseStatement(Box::new(
            self.parse_statement(Arc::clone(&block))?,
        )))
    }

    fn parse_loop_statement(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut condition = AbstractSyntaxTree::Literal(Literal::from(true));
        let current = self.get_current_token();

        if TokenKind::Keyword(Keyword::While) == current.kind {
            condition = self.parse_expression(Arc::clone(&block))?;
        } else if TokenKind::Keyword(Keyword::Until) == current.kind {
            condition = self.parse_expression(Arc::clone(&block))?;
            condition = AbstractSyntaxTree::UnaryExpression(
                Operator::Logical(Logical::Not),
                Box::new(condition),
            );
        } else {
            self.temp_token_buf.push(current);
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
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let open_paranthesis = self.get_current_token();
        let mut parameters: Vec<AbstractSyntaxTree> = Vec::new();
        let mut current = self.get_current_token();
        while TokenKind::Symbol(CloseParanthesis) != current.kind
            && TokenKind::EndOfFile != current.kind
        {
            if let TokenKind::Identifier(name) = &current.kind {
                let parameter = AbstractSyntaxTree::Identifier(name.to_owned());
                parameters.push(parameter);
            } else {
                self.temp_token_buf.push(current);
                self.temp_token_buf.push(open_paranthesis);
                return self.parse_expression(block);
            }
            let prev = current;
            current = self.get_current_token();
            if TokenKind::Symbol(CloseParanthesis) != current.kind
                && TokenKind::Symbol(Comma) != current.kind
            {
                self.temp_token_buf.push(current);
                self.temp_token_buf.push(prev);
                self.temp_token_buf.push(open_paranthesis);
                return self.parse_expression(block);
            } else if TokenKind::Symbol(Comma) == current.kind {
                current = self.get_current_token();
            }
        }

        let equal = self.get_current_token();
        let arrow = self.get_current_token();

        if TokenKind::Symbol(Equals) != equal.kind && TokenKind::Symbol(GreaterThan) != arrow.kind {
            return Err(CompilerError::MissingArrow(
                current.kind,
                current.line,
                current.column,
            ));
        }

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
        &mut self,
        name: String,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut arguments: Vec<Box<AbstractSyntaxTree>> = Vec::new();
        let mut current = self.get_current_token();
        while TokenKind::Symbol(CloseParanthesis) != current.kind
            && TokenKind::EndOfFile != current.kind
        {
            self.temp_token_buf.push(current);
            let expression = self.parse_expression(Arc::clone(&block))?;
            arguments.push(Box::new(expression));
            current = self.get_current_token();
            if TokenKind::Symbol(CloseParanthesis) != current.kind
                && TokenKind::Symbol(Comma) != current.kind
            {
                return Err(CompilerError::UnexpectedTokenWithExpected(
                    current.kind,
                    TokenKind::Symbol(Comma),
                    current.line,
                    current.column,
                ));
            }
        }
        self.temp_token_buf.push(current);
        self.match_token(TokenKind::Symbol(CloseParanthesis))?;

        let arguments = SeperatedStatements::new(Comma, OpenParanthesis, arguments);
        Ok(AbstractSyntaxTree::CallStatement(name, arguments))
    }

    fn parse_expression(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        self.parse_assignment_expression(block)
    }

    fn parse_assignment_expression(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let identifier_token = self.get_current_token();
        match &identifier_token.kind {
            TokenKind::Keyword(Keyword::Let) => return self.handle_mutable_keyword(block),
            TokenKind::Identifier(name) => {
                if let Some(operator) = self.match_assignment_operator() {
                    let expression = self.parse_statement(block)?;

                    return Ok(AbstractSyntaxTree::AssignmentExpression(
                        name.to_owned(),
                        operator,
                        Box::new(expression),
                    ));
                }
            }
            _ => {}
        }
        self.temp_token_buf.push(identifier_token);
        self.parse_arithmetic_expression(0, block)
    }

    fn parse_arithmetic_expression(
        &mut self,
        parent_precedence: u8,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let mut left = if let Some(operator) = self.match_operator() {
            if operator.get_unery_precedence() >= parent_precedence {
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

        while let Some(operator) = self.match_operator() {
            let precedence = operator.get_binary_precedence();
            if precedence <= parent_precedence {
                break;
            }
            let right = self.parse_arithmetic_expression(precedence, Arc::clone(&block))?;
            left = AbstractSyntaxTree::BinaryExpression(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn match_token(&mut self, kind: TokenKind) -> Result<Token, CompilerError> {
        let token = self.get_current_token();
        if kind == token.kind {
            Ok(token)
        } else {
            Err(CompilerError::UnexpectedTokenWithExpected(
                token.kind,
                kind,
                token.line,
                token.column,
            ))
        }
    }

    fn parse_factor(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let token = self.get_current_token();
        match token.kind {
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
                    TokenKind::Symbol(symbol),
                    token.line,
                    token.column,
                )),
            },
            TokenKind::Identifier(name) => {
                let current = self.get_current_token();
                if TokenKind::Symbol(OpenParanthesis) == current.kind {
                    self.parse_function_call_statement(name.to_string(), block)
                } else {
                    self.temp_token_buf.push(current);
                    Ok(AbstractSyntaxTree::Identifier(name.clone()))
                }
            }
            TokenKind::Keyword(Keyword::If) => self.parse_if_statement(block),
            kind => Err(CompilerError::UnexpectedToken(
                kind,
                token.line,
                token.column,
            )),
        }
    }

    fn handle_mutable_keyword(
        &mut self,
        block: Arc<RwLock<Block>>,
    ) -> Result<AbstractSyntaxTree, CompilerError> {
        let current = self.get_current_token();
        if let TokenKind::Identifier(variable_name) = &current.kind {
            if let Some(operator) = self.match_operator() {
                // `mutable` variable_name operator expression
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

    fn match_assignment_operator(&mut self) -> Option<Operator> {
        let token = self.get_current_token();
        if let TokenKind::Symbol(operator_symbol) = &token.kind {
            let second_token = self.get_current_token();
            match operator_symbol {
                Equals => {
                    // =
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // ==
                            self.temp_token_buf.push(second_token);
                            self.temp_token_buf.push(token);
                            return None;
                        }
                    }
                    self.temp_token_buf.push(second_token);
                    // =
                    return Some(Assignment(Assingment::Simple));
                }
                Plus => {
                    // +
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // +=
                            return Some(Assignment(Assingment::Addition));
                        }
                    }
                }
                Minus => {
                    // -
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // -=
                            return Some(Assignment(Assingment::Subtraction));
                        }
                    }
                }
                Asterisk => {
                    // *
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // *=
                            return Some(Assignment(Assingment::Multiplication));
                        } else if second_symbol == Asterisk {
                            // **
                            let current = self.get_current_token();
                            if let TokenKind::Symbol(third_token) = current.kind {
                                if third_token == Equals {
                                    // **=
                                    return Some(Assignment(Assingment::Exponentiation));
                                }
                            }
                            self.temp_token_buf.push(current);
                        }
                    }
                }
                Slash => {
                    // /
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // /=
                            return Some(Assignment(Assingment::Division));
                        }
                    }
                }
                Percent => {
                    // %
                    if let TokenKind::Symbol(second_symbol) = second_token.kind {
                        if second_symbol == Equals {
                            // %=
                            return Some(Assignment(Assingment::Modulo));
                        }
                    }
                }
                _ => {}
            }
            self.temp_token_buf.push(second_token);
        }
        self.temp_token_buf.push(token);
        None
    }

    fn match_operator(&mut self) -> Option<Operator> {
        let current = self.get_current_token();
        let operator = match &current.kind {
            TokenKind::Symbol(operator_symbol) => {
                let second_token = self.get_current_token();
                match operator_symbol {
                    Equals => {
                        // =
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // ==
                                return Some(Relational(Relational::Equality));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // =
                        Assignment(Assingment::Simple)
                    }
                    Plus => {
                        // +
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // +=
                                return Some(Assignment(Assingment::Addition));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // +
                        Arithmetic(Arithmetic::Addition)
                    }
                    Minus => {
                        // -
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // -=
                                return Some(Assignment(Assingment::Subtraction));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // -
                        Arithmetic(Arithmetic::Subtraction)
                    }
                    Asterisk => {
                        // *
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // *=
                                return Some(Assignment(Assingment::Multiplication));
                            } else if second_symbol == Asterisk {
                                // **
                                let current = self.get_current_token();
                                if let TokenKind::Symbol(third_token) = current.kind {
                                    if third_token == Equals {
                                        // **=
                                        return Some(Assignment(Assingment::Exponentiation));
                                    }
                                }
                                self.temp_token_buf.push(current);
                                // **
                                return Some(Arithmetic(Arithmetic::Exponentiation));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // *
                        Arithmetic(Arithmetic::Multiplication)
                    }
                    Slash => {
                        // /
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // /=
                                return Some(Assignment(Assingment::Division));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // /
                        Arithmetic(Arithmetic::Division)
                    }
                    Percent => {
                        // %
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // %=
                                return Some(Assignment(Assingment::Modulo));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // %
                        Arithmetic(Arithmetic::Modulo)
                    }
                    Exclamation => {
                        // !
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // !=
                                return Some(Relational(Relational::InEquality));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // !
                        Logical(Logical::Not)
                    }
                    GreaterThan => {
                        // >
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // >=
                                return Some(Relational(Relational::GreaterThanOrEquals));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // >
                        Relational(Relational::GreaterThan)
                    }
                    LessThan => {
                        // <
                        if let TokenKind::Symbol(second_symbol) = second_token.kind {
                            if second_symbol == Equals {
                                // <=
                                return Some(Relational(Relational::LessThanOrEquals));
                            }
                        }
                        self.temp_token_buf.push(second_token);
                        // <
                        Relational(Relational::LessThan)
                    }
                    _ => {
                        self.temp_token_buf.push(second_token);
                        self.temp_token_buf.push(current);
                        return None;
                    }
                }
            }
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Is => {
                    let current = self.get_current_token();
                    if let TokenKind::Keyword(keyword) = &current.kind {
                        if &Keyword::Not == keyword {
                            return Some(Relational(Relational::Equality));
                        }
                    }
                    self.temp_token_buf.push(current);
                    Relational(Relational::Equality)
                }
                Keyword::And => Logical(Logical::And),
                Keyword::Or => Logical(Logical::Or),
                Keyword::Not => Logical(Logical::Not),
                Keyword::Xor => Logical(Logical::Xor),
                _ => {
                    self.temp_token_buf.push(current);
                    return None;
                }
            },
            _ => {
                self.temp_token_buf.push(current);
                return None;
            }
        };
        Some(operator)
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
