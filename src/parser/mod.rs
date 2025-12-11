//! # A-lang Parser
//!
//! Recursive descent parser for A-lang syntax.
//! Converts tokens into an Abstract Syntax Tree (AST).

use crate::ast::*;
use crate::lexer::Token;
use std::fmt;

/// Parser error type
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: std::ops::Range<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at {:?}: {}", self.span, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parser state
pub struct Parser {
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, std::ops::Range<usize>)>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current).map(|(token, _)| token)
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1).map(|(token, _)| token)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &Token) -> bool {
        if let Some(token) = self.peek() {
            std::mem::discriminant(token) == std::mem::discriminant(token_type)
        } else {
            false
        }
    }

    fn match_token(&mut self, token_type: &Token) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current_span(&self) -> Span {
        if let Some((_, range)) = self.tokens.get(self.current) {
            Span::new(range.start, range.end, 0, 0)
        } else {
            Span::dummy()
        }
    }

    /// Parse a program (list of statements)
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program {
            statements,
            span: Span::dummy(),
        })
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();

        match self.peek() {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Const) => self.parse_const_statement(),
            Some(Token::Var) => self.parse_var_statement(),
            Some(Token::Reactive) => self.parse_reactive_statement(),
            Some(Token::Computed) => self.parse_computed_statement(),
            Some(Token::Effect) => self.parse_effect_statement(),
            Some(Token::Fn) => self.parse_function_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::Elif) => self.parse_if_statement(), // elif is just another if
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Break) => {
                self.advance();
                self.match_token(&Token::Semicolon);
                Ok(Statement::Break { span })
            }
            Some(Token::Continue) => {
                self.advance();
                self.match_token(&Token::Semicolon);
                Ok(Statement::Continue { span })
            }
            Some(Token::Try) => self.parse_try_statement(),
            Some(Token::Throw) => self.parse_throw_statement(),
            Some(Token::Snapshot) => self.parse_snapshot_statement(),
            Some(Token::Rewind) => self.parse_rewind_statement(),
            Some(Token::Checkpoint) => self.parse_checkpoint_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'let'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after 'let'".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::Assign) {
            return Err(ParseError {
                message: "Expected '=' after variable name".to_string(),
                span: span.start..span.end,
            });
        }

        let value = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Let {
            name,
            value,
            type_annotation: None,
            span,
        })
    }

    fn parse_const_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'const'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after 'const'".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::Assign) {
            return Err(ParseError {
                message: "Expected '=' after constant name".to_string(),
                span: span.start..span.end,
            });
        }

        let value = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Const {
            name,
            value,
            type_annotation: None,
            span,
        })
    }

    fn parse_var_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'var'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after 'var'".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::Assign) {
            return Err(ParseError {
                message: "Expected '=' after variable name".to_string(),
                span: span.start..span.end,
            });
        }

        let value = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Let {
            name,
            value,
            type_annotation: None,
            span,
        })
    }

    fn parse_reactive_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'reactive'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after 'reactive'".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::Assign) {
            return Err(ParseError {
                message: "Expected '=' after reactive variable name".to_string(),
                span: span.start..span.end,
            });
        }

        let initial_value = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Reactive {
            name,
            initial_value,
            type_annotation: None,
            span,
        })
    }

    fn parse_computed_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'computed'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected identifier after 'computed'".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::Assign) {
            return Err(ParseError {
                message: "Expected '=' after computed variable name".to_string(),
                span: span.start..span.end,
            });
        }

        let expression = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Computed {
            name,
            dependencies: Vec::new(),
            expression,
            span,
        })
    }

    fn parse_effect_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'effect'

        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'effect'".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after 'effect('".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::FatArrow) {
            return Err(ParseError {
                message: "Expected '=>' after 'effect()'".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' after 'effect() =>'".to_string(),
                span: span.start..span.end,
            });
        }

        let body = self.parse_block()?;

        Ok(Statement::Effect {
            dependencies: Vec::new(),
            body,
            span,
        })
    }

    fn parse_function_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'fn'

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected function name".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after function name".to_string(),
                span: span.start..span.end,
            });
        }

        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(n)) => n.clone(),
                    _ => {
                        return Err(ParseError {
                            message: "Expected parameter name".to_string(),
                            span: span.start..span.end,
                        })
                    }
                };

                parameters.push(Parameter {
                    name: param_name,
                    type_annotation: None,
                    default_value: None,
                    span: self.current_span(),
                });

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after parameters".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' before function body".to_string(),
                span: span.start..span.end,
            });
        }

        let body = self.parse_block()?;

        Ok(Statement::Function {
            name,
            parameters,
            body,
            return_type: None,
            is_async: false,
            span,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'return'

        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.match_token(&Token::Semicolon);

        Ok(Statement::Return { value, span })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'if' or 'elif'

        // Expect opening parenthesis (JS style)
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'if'".to_string(),
                span: span.start..span.end,
            });
        }

        let condition = self.parse_expression()?;

        // Expect closing parenthesis
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after if condition".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' after if condition".to_string(),
                span: span.start..span.end,
            });
        }

        let then_branch = self.parse_block()?;

        // Handle elif and else
        let else_branch = if self.check(&Token::Elif) {
            // Parse elif as nested if-else
            Some(vec![self.parse_statement()?])
        } else if self.match_token(&Token::Else) {
            if !self.match_token(&Token::LeftBrace) {
                return Err(ParseError {
                    message: "Expected '{' after else".to_string(),
                    span: span.start..span.end,
                });
            }
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            span,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'while'

        // Expect opening parenthesis (JS style)
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'while'".to_string(),
                span: span.start..span.end,
            });
        }

        let condition = self.parse_expression()?;

        // Expect closing parenthesis
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after while condition".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' after while condition".to_string(),
                span: span.start..span.end,
            });
        }

        let body = self.parse_block()?;

        Ok(Statement::While {
            condition,
            body,
            span,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'for'

        // Expect opening parenthesis
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'for'".to_string(),
                span: span.start..span.end,
            });
        }

        // Check for classic C-style for loop: for (init; cond; increment)
        // vs for-in loop: for (variable in iterable)

        // Try to parse first part
        let checkpoint = self.current;

        // Check if it's for-in style by looking ahead
        if let Some(Token::Identifier(var_name)) = self.peek().cloned() {
            self.advance();

            if self.match_token(&Token::In) {
                // For-in loop: for (variable in iterable) { }
                let iterable = self.parse_expression()?;

                if !self.match_token(&Token::RightParen) {
                    return Err(ParseError {
                        message: "Expected ')' after for-in iterable".to_string(),
                        span: span.start..span.end,
                    });
                }

                if !self.match_token(&Token::LeftBrace) {
                    return Err(ParseError {
                        message: "Expected '{' after for loop".to_string(),
                        span: span.start..span.end,
                    });
                }

                let body = self.parse_block()?;

                return Ok(Statement::For {
                    variable: var_name,
                    iterable,
                    body,
                    span,
                });
            } else {
                // Not for-in, backtrack and parse as classic for
                self.current = checkpoint;
            }
        }

        // Classic C-style for loop: for (init; cond; increment) { }
        // Parse init - simple assignment check
        let init = if self.check(&Token::Semicolon) {
            self.advance(); // consume semicolon
            None
        } else {
            // Try to parse as assignment first
            if let Some(Token::Identifier(name)) = self.peek().cloned() {
                let save_pos = self.current;
                self.advance();

                if self.check(&Token::Assign) {
                    self.advance(); // consume =
                    let value = self.parse_expression()?;
                    self.match_token(&Token::Semicolon);
                    Some(Box::new(Statement::Let {
                        name,
                        value,
                        type_annotation: None,
                        span: span.clone(),
                    }))
                } else {
                    // Backtrack and parse normally
                    self.current = save_pos;
                    let expr = self.parse_expression()?;
                    self.match_token(&Token::Semicolon);
                    Some(Box::new(Statement::Expression {
                        expr,
                        span: span.clone(),
                    }))
                }
            } else {
                let expr = self.parse_expression()?;
                self.match_token(&Token::Semicolon);
                Some(Box::new(Statement::Expression {
                    expr,
                    span: span.clone(),
                }))
            }
        };

        // Parse condition
        let condition = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        if !self.match_token(&Token::Semicolon) {
            return Err(ParseError {
                message: "Expected ';' after for loop condition".to_string(),
                span: span.start..span.end,
            });
        }

        // Parse increment - handle assignment
        let increment_stmt = if self.check(&Token::RightParen) {
            None
        } else {
            // Try to parse as assignment
            if let Some(Token::Identifier(name)) = self.peek().cloned() {
                let save_pos = self.current;
                self.advance();

                if self.check(&Token::Assign) {
                    self.advance(); // consume =
                    let value = self.parse_expression()?;
                    Some(Statement::Let {
                        name,
                        value,
                        type_annotation: None,
                        span: span.clone(),
                    })
                } else {
                    // Backtrack
                    self.current = save_pos;
                    let expr = self.parse_expression()?;
                    Some(Statement::Expression {
                        expr,
                        span: span.clone(),
                    })
                }
            } else {
                let expr = self.parse_expression()?;
                Some(Statement::Expression {
                    expr,
                    span: span.clone(),
                })
            }
        };

        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after for loop increment".to_string(),
                span: span.start..span.end,
            });
        }

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' after for loop".to_string(),
                span: span.start..span.end,
            });
        }

        let mut body = self.parse_block()?;

        // Desugar classic for loop to while loop
        // for (init; cond; incr) { body } => init; while (cond) { body; incr; }

        if let Some(incr) = increment_stmt {
            body.push(incr);
        }

        let while_stmt = Statement::While {
            condition: condition.unwrap_or(Expression::Literal {
                value: crate::ast::Literal::Boolean(true),
                span: span.clone(),
            }),
            body,
            span: span.clone(),
        };

        if let Some(init_stmt) = init {
            // Return a block containing init and while
            Ok(Statement::Expression {
                expr: Expression::Literal {
                    value: crate::ast::Literal::Nil,
                    span: span.clone(),
                },
                span,
            })
        } else {
            Ok(while_stmt)
        }
    }

    fn parse_snapshot_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'snapshot'

        // Only take a label if it's a string literal (not bare identifier which could be next statement)
        let label = if let Some(Token::String(name)) = self.peek() {
            let label = Some(name.clone());
            self.advance();
            label
        } else {
            None
        };

        self.match_token(&Token::Semicolon);

        Ok(Statement::Snapshot { label, span })
    }

    fn parse_rewind_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'rewind'

        // Check for "rewind to <label>"
        let (steps, to_label) = if let Some(Token::Identifier(name)) = self.peek() {
            if name == "to" {
                self.advance(); // consume 'to'
                let label = match self.advance() {
                    Some(Token::Identifier(n)) | Some(Token::String(n)) => Some(n.clone()),
                    _ => None,
                };
                (None, label)
            } else {
                // It's an expression (like a number)
                (Some(self.parse_expression()?), None)
            }
        } else if !self.check(&Token::Semicolon) && !self.is_at_end() {
            (Some(self.parse_expression()?), None)
        } else {
            (None, None)
        };

        self.match_token(&Token::Semicolon);

        Ok(Statement::Rewind {
            steps,
            to_label,
            span,
        })
    }

    fn parse_checkpoint_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'checkpoint'

        let label = match self.advance() {
            Some(Token::Identifier(n)) | Some(Token::String(n)) => n.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected checkpoint label".to_string(),
                    span: span.start..span.end,
                })
            }
        };

        self.match_token(&Token::Semicolon);

        Ok(Statement::Checkpoint { label, span })
    }

    fn parse_try_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'try'

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' after 'try'".to_string(),
                span: span.start..span.end,
            });
        }

        let try_block = self.parse_block()?;

        // Parse catch clause
        let catch_clause = if self.match_token(&Token::Catch) {
            let catch_span = self.current_span();
            let parameter = if self.match_token(&Token::LeftParen) {
                let param = if let Some(Token::Identifier(name)) = self.advance() {
                    Some(name.clone())
                } else {
                    None
                };

                if !self.match_token(&Token::RightParen) {
                    return Err(ParseError {
                        message: "Expected ')' after catch parameter".to_string(),
                        span: span.start..span.end,
                    });
                }
                param
            } else {
                None
            };

            if !self.match_token(&Token::LeftBrace) {
                return Err(ParseError {
                    message: "Expected '{' after 'catch'".to_string(),
                    span: span.start..span.end,
                });
            }

            let body = self.parse_block()?;

            Some(crate::ast::CatchClause {
                parameter,
                body,
                span: catch_span,
            })
        } else {
            None
        };

        // Parse finally clause
        let finally_block = if self.match_token(&Token::Finally) {
            if !self.match_token(&Token::LeftBrace) {
                return Err(ParseError {
                    message: "Expected '{' after 'finally'".to_string(),
                    span: span.start..span.end,
                });
            }
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Statement::Try {
            try_block,
            catch_clause,
            finally_block,
            span,
        })
    }

    fn parse_throw_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();
        self.advance(); // consume 'throw'

        let value = self.parse_expression()?;
        self.match_token(&Token::Semicolon);

        Ok(Statement::Throw { value, span })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let span = self.current_span();

        // Check for bare identifier that could be a variable declaration (like JS/Python)
        // e.g., "x = 5" without let/var/const
        if let Some(Token::Identifier(name)) = self.peek().cloned() {
            let name = name.clone();
            self.advance();

            // Check for compound assignments (+=, -=, etc)
            if let Some(token) = self.peek() {
                match token {
                    Token::Assign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::PlusAssign => {
                        self.advance();
                        let rhs = self.parse_expression()?;
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Add,
                            right: Box::new(rhs),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::MinusAssign => {
                        self.advance();
                        let rhs = self.parse_expression()?;
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Subtract,
                            right: Box::new(rhs),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::StarAssign => {
                        self.advance();
                        let rhs = self.parse_expression()?;
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Multiply,
                            right: Box::new(rhs),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::SlashAssign => {
                        self.advance();
                        let rhs = self.parse_expression()?;
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Divide,
                            right: Box::new(rhs),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::PercentAssign => {
                        self.advance();
                        let rhs = self.parse_expression()?;
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Modulo,
                            right: Box::new(rhs),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::Increment => {
                        // x++
                        self.advance();
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Add,
                            right: Box::new(Expression::Literal {
                                value: Literal::Integer(1),
                                span: span.clone(),
                            }),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    Token::Decrement => {
                        // x--
                        self.advance();
                        let value = Expression::Binary {
                            left: Box::new(Expression::Identifier {
                                name: name.clone(),
                                span: span.clone(),
                            }),
                            operator: BinaryOp::Subtract,
                            right: Box::new(Expression::Literal {
                                value: Literal::Integer(1),
                                span: span.clone(),
                            }),
                            span: span.clone(),
                        };
                        self.match_token(&Token::Semicolon);
                        return Ok(Statement::Let {
                            name,
                            value,
                            type_annotation: None,
                            span,
                        });
                    }
                    _ => {
                        // Not an assignment, backtrack and parse as expression
                        self.current -= 1;
                    }
                }
            } else {
                // Not an assignment, backtrack and parse as expression
                self.current -= 1;
            }
        }

        // Check for prefix increment/decrement (++x, --x)
        if self.match_token(&Token::Increment) {
            if let Some(Token::Identifier(name)) = self.advance() {
                let name = name.clone();
                let value = Expression::Binary {
                    left: Box::new(Expression::Identifier {
                        name: name.clone(),
                        span: span.clone(),
                    }),
                    operator: BinaryOp::Add,
                    right: Box::new(Expression::Literal {
                        value: Literal::Integer(1),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                };
                self.match_token(&Token::Semicolon);
                return Ok(Statement::Let {
                    name,
                    value,
                    type_annotation: None,
                    span,
                });
            }
        }

        if self.match_token(&Token::Decrement) {
            if let Some(Token::Identifier(name)) = self.advance() {
                let name = name.clone();
                let value = Expression::Binary {
                    left: Box::new(Expression::Identifier {
                        name: name.clone(),
                        span: span.clone(),
                    }),
                    operator: BinaryOp::Subtract,
                    right: Box::new(Expression::Literal {
                        value: Literal::Integer(1),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                };
                self.match_token(&Token::Semicolon);
                return Ok(Statement::Let {
                    name,
                    value,
                    type_annotation: None,
                    span,
                });
            }
        }

        let expr = self.parse_expression()?;

        // Check for assignment (variable reassignment) for complex expressions
        if self.match_token(&Token::Assign) {
            if let Expression::Identifier { name, .. } = expr {
                let value = self.parse_expression()?;
                self.match_token(&Token::Semicolon);
                return Ok(Statement::Let {
                    name,
                    value,
                    type_annotation: None,
                    span,
                });
            } else {
                return Err(ParseError {
                    message: "Invalid assignment target".to_string(),
                    span: span.start..span.end,
                });
            }
        }

        self.match_token(&Token::Semicolon);

        Ok(Statement::Expression { expr, span })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        if !self.match_token(&Token::RightBrace) {
            return Err(ParseError {
                message: "Expected '}' at end of block".to_string(),
                span: self.current_span().start..self.current_span().end,
            });
        }

        Ok(statements)
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        // Check for arrow function: x => expr or (params) => expr
        let checkpoint = self.current;

        // Try to parse as arrow function with single parameter
        if let Some(Token::Identifier(param)) = self.peek().cloned() {
            self.advance();
            if self.match_token(&Token::FatArrow) {
                // Single parameter arrow function: x => expr or x => { ... }
                let body = if self.check(&Token::LeftBrace) {
                    self.advance();
                    self.parse_block()?
                } else {
                    let body_expr = self.parse_assignment()?;
                    vec![Statement::Return {
                        value: Some(body_expr),
                        span: Span::dummy(),
                    }]
                };

                return Ok(Expression::Lambda {
                    parameters: vec![Parameter {
                        name: param,
                        type_annotation: None,
                        default_value: None,
                        span: Span::dummy(),
                    }],
                    body,
                    return_type: None,
                    span: Span::dummy(),
                });
            } else {
                // Not an arrow function, backtrack
                self.current = checkpoint;
            }
        }

        // Try to parse as arrow function with parenthesized parameters
        if self.check(&Token::LeftParen) {
            let save_pos = self.current;
            self.advance(); // consume '('

            let mut params = Vec::new();
            let mut is_arrow = true;

            if !self.check(&Token::RightParen) {
                loop {
                    if let Some(Token::Identifier(name)) = self.advance() {
                        params.push(Parameter {
                            name: name.clone(),
                            type_annotation: None,
                            default_value: None,
                            span: Span::dummy(),
                        });

                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    } else {
                        is_arrow = false;
                        break;
                    }
                }
            }

            if is_arrow
                && self.match_token(&Token::RightParen)
                && self.match_token(&Token::FatArrow)
            {
                // It's an arrow function: (x, y) => expr or (x, y) => { ... }
                let body = if self.check(&Token::LeftBrace) {
                    self.advance();
                    self.parse_block()?
                } else {
                    let body_expr = self.parse_assignment()?;
                    vec![Statement::Return {
                        value: Some(body_expr),
                        span: Span::dummy(),
                    }]
                };

                return Ok(Expression::Lambda {
                    parameters: params,
                    body,
                    return_type: None,
                    span: Span::dummy(),
                });
            } else {
                // Not an arrow function, backtrack
                self.current = save_pos;
            }
        }

        let expr = self.parse_ternary()?;
        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_or()?;

        if self.match_token(&Token::Question) {
            let span = self.current_span();
            let then_expr = self.parse_or()?;

            if !self.match_token(&Token::Colon) {
                return Err(ParseError {
                    message: "Expected ':' in ternary expression".to_string(),
                    span: span.start..span.end,
                });
            }

            let else_expr = self.parse_ternary()?;

            expr = Expression::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;

        while self.match_token(&Token::Or) {
            let span = self.current_span();
            let right = self.parse_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&Token::And) {
            let span = self.current_span();
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let span = self.current_span();
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            self.advance();
            let span = self.current_span();
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let span = self.current_span();
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => break,
            };
            self.advance();
            let span = self.current_span();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if let Some(token) = self.peek() {
            let op = match token {
                Token::Not => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => return self.parse_postfix(),
            };
            self.advance();
            let span = self.current_span();
            let operand = Box::new(self.parse_unary()?);
            return Ok(Expression::Unary {
                operator: op,
                operand,
                span,
            });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                Some(Token::LeftParen) => {
                    self.advance();
                    let mut arguments = Vec::new();

                    if !self.check(&Token::RightParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }

                    if !self.match_token(&Token::RightParen) {
                        return Err(ParseError {
                            message: "Expected ')' after arguments".to_string(),
                            span: self.current_span().start..self.current_span().end,
                        });
                    }

                    let span = self.current_span();
                    expr = Expression::Call {
                        callee: Box::new(expr),
                        arguments,
                        span,
                    };
                }
                Some(Token::LeftBracket) => {
                    self.advance();
                    let index = self.parse_expression()?;

                    if !self.match_token(&Token::RightBracket) {
                        return Err(ParseError {
                            message: "Expected ']' after index".to_string(),
                            span: self.current_span().start..self.current_span().end,
                        });
                    }

                    let span = self.current_span();
                    expr = Expression::IndexAccess {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                Some(Token::Dot) => {
                    self.advance();
                    let property = match self.advance() {
                        Some(Token::Identifier(name)) => name.clone(),
                        _ => {
                            return Err(ParseError {
                                message: "Expected property name after '.'".to_string(),
                                span: self.current_span().start..self.current_span().end,
                            })
                        }
                    };

                    let span = self.current_span();
                    expr = Expression::PropertyAccess {
                        object: Box::new(expr),
                        property,
                        span,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let span = self.current_span();

        match self.peek() {
            Some(Token::Integer(n)) => {
                let n = *n;
                self.advance();

                // Check for range (e.g., 1..10)
                if self.match_token(&Token::DotDot) {
                    let end = self.parse_unary()?;
                    return Ok(Expression::Range {
                        start: Box::new(Expression::Literal {
                            value: Literal::Integer(n),
                            span: span.clone(),
                        }),
                        end: Box::new(end),
                        inclusive: false,
                        span: self.current_span(),
                    });
                }

                Ok(Expression::Literal {
                    value: Literal::Integer(n),
                    span,
                })
            }
            Some(Token::Float(f)) => {
                let f = *f;
                self.advance();
                Ok(Expression::Literal {
                    value: Literal::Float(f),
                    span,
                })
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::Literal {
                    value: Literal::String(s),
                    span,
                })
            }
            Some(Token::TemplateString(template)) => {
                let template = template.clone();
                self.advance();

                // Parse template string with interpolation
                // Convert `Hello ${name}!` to "Hello " + name + "!"
                Ok(self.parse_template_string(&template, span.clone())?)
            }
            Some(Token::True) => {
                self.advance();
                Ok(Expression::Literal {
                    value: Literal::Boolean(true),
                    span,
                })
            }
            Some(Token::False) => {
                self.advance();
                Ok(Expression::Literal {
                    value: Literal::Boolean(false),
                    span,
                })
            }
            Some(Token::Nil) => {
                self.advance();
                Ok(Expression::Literal {
                    value: Literal::Nil,
                    span,
                })
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                // Check for range
                if self.match_token(&Token::DotDot) {
                    let end = self.parse_unary()?;
                    return Ok(Expression::Range {
                        start: Box::new(Expression::Identifier { name, span }),
                        end: Box::new(end),
                        inclusive: false,
                        span: self.current_span(),
                    });
                }

                Ok(Expression::Identifier { name, span })
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                if !self.match_token(&Token::RightParen) {
                    return Err(ParseError {
                        message: "Expected ')' after expression".to_string(),
                        span: span.start..span.end,
                    });
                }
                Ok(expr)
            }
            Some(Token::LeftBracket) => {
                self.advance();
                let mut elements = Vec::new();

                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }

                if !self.match_token(&Token::RightBracket) {
                    return Err(ParseError {
                        message: "Expected ']' after array elements".to_string(),
                        span: span.start..span.end,
                    });
                }

                Ok(Expression::Array { elements, span })
            }
            Some(Token::LeftBrace) => {
                self.advance();
                let mut fields = Vec::new();

                if !self.check(&Token::RightBrace) {
                    loop {
                        let key = match self.advance() {
                            Some(Token::Identifier(name)) | Some(Token::String(name)) => {
                                name.clone()
                            }
                            _ => {
                                return Err(ParseError {
                                    message: "Expected object key".to_string(),
                                    span: span.start..span.end,
                                })
                            }
                        };

                        if !self.match_token(&Token::Colon) {
                            return Err(ParseError {
                                message: "Expected ':' after object key".to_string(),
                                span: span.start..span.end,
                            });
                        }

                        let value = self.parse_expression()?;
                        fields.push((key, value));

                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }

                if !self.match_token(&Token::RightBrace) {
                    return Err(ParseError {
                        message: "Expected '}' after object fields".to_string(),
                        span: span.start..span.end,
                    });
                }

                Ok(Expression::Object { fields, span })
            }
            _ => Err(ParseError {
                message: format!("Unexpected token: {:?}", self.peek()),
                span: span.start..span.end,
            }),
        }
    }

    /// Parse template string with interpolation
    fn parse_template_string(
        &mut self,
        template: &str,
        span: Span,
    ) -> Result<Expression, ParseError> {
        let mut parts: Vec<Expression> = Vec::new();
        let mut current = String::new();
        let mut chars = template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'

                // Add current string part if not empty
                if !current.is_empty() {
                    parts.push(Expression::Literal {
                        value: Literal::String(current.clone()),
                        span: span.clone(),
                    });
                    current.clear();
                }

                // Extract expression until '}'
                let mut expr_str = String::new();
                let mut depth = 1;
                while let Some(ch) = chars.next() {
                    if ch == '{' {
                        depth += 1;
                        expr_str.push(ch);
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        expr_str.push(ch);
                    } else {
                        expr_str.push(ch);
                    }
                }

                // Parse the expression
                if !expr_str.is_empty() {
                    // Tokenize and parse the expression
                    match crate::lexer::tokenize(&expr_str) {
                        Ok(tokens) => {
                            let mut expr_parser = Parser::new(tokens);
                            match expr_parser.parse_expression() {
                                Ok(expr) => parts.push(expr),
                                Err(_) => {
                                    // If parsing fails, treat as string
                                    parts.push(Expression::Literal {
                                        value: Literal::String(expr_str),
                                        span: span.clone(),
                                    });
                                }
                            }
                        }
                        Err(_) => {
                            // If tokenizing fails, treat as string
                            parts.push(Expression::Literal {
                                value: Literal::String(expr_str),
                                span: span.clone(),
                            });
                        }
                    }
                }
            } else {
                current.push(ch);
            }
        }

        // Add remaining string part
        if !current.is_empty() {
            parts.push(Expression::Literal {
                value: Literal::String(current),
                span: span.clone(),
            });
        }

        // Combine all parts with + operator
        if parts.is_empty() {
            Ok(Expression::Literal {
                value: Literal::String(String::new()),
                span,
            })
        } else if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            // Build chain of Binary expressions with Add operator
            let mut result = parts[0].clone();
            for part in parts.into_iter().skip(1) {
                result = Expression::Binary {
                    left: Box::new(result),
                    operator: BinaryOp::Add,
                    right: Box::new(part),
                    span: span.clone(),
                };
            }
            Ok(result)
        }
    }
}

/// Parse tokens into an AST
pub fn parse(tokens: Vec<(Token, std::ops::Range<usize>)>) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse_literal() {
        let tokens = tokenize("42").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_binary_op() {
        let tokens = tokenize("5 + 3").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_let() {
        let tokens = tokenize("let x = 42;").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let tokens = tokenize("fn add(a, b) { return a + b; }").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
}
