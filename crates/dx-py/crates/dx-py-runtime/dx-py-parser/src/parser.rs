//! Python parser

use crate::ast::*;
use crate::error::{Location, ParseError, ParseResult};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

/// Python parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given source
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            current: None,
        }
    }

    /// Get current location
    fn location(&self) -> Location {
        self.current
            .as_ref()
            .map(|t| t.location)
            .unwrap_or_default()
    }

    /// Advance to the next token
    fn advance(&mut self) -> ParseResult<Token> {
        let token = self.lexer.next_token()?;
        let prev = self.current.replace(token.clone());
        Ok(prev.unwrap_or(token))
    }

    /// Peek at the current token
    fn peek(&mut self) -> ParseResult<&Token> {
        if self.current.is_none() {
            self.current = Some(self.lexer.next_token()?);
        }
        Ok(self.current.as_ref().unwrap())
    }

    /// Check if current token matches
    fn check(&mut self, kind: &TokenKind) -> ParseResult<bool> {
        Ok(std::mem::discriminant(&self.peek()?.kind) == std::mem::discriminant(kind))
    }

    /// Consume token if it matches
    fn consume(&mut self, kind: &TokenKind) -> ParseResult<bool> {
        if self.check(kind)? {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Expect a specific token
    fn expect(&mut self, kind: &TokenKind) -> ParseResult<Token> {
        if self.check(kind)? {
            self.advance()
        } else {
            let token = self.peek()?;
            Err(ParseError::unexpected_token(
                token.location,
                &kind.to_string(),
                &token.kind.to_string(),
            ))
        }
    }

    /// Parse a module
    pub fn parse_module(&mut self) -> ParseResult<Module> {
        let mut body = Vec::new();

        while !self.check(&TokenKind::Eof)? {
            // Skip newlines at module level
            while self.consume(&TokenKind::Newline)? {}
            
            if self.check(&TokenKind::Eof)? {
                break;
            }

            body.push(self.parse_statement()?);
        }

        Ok(Module { body })
    }

    /// Parse a single statement
    pub fn parse_statement(&mut self) -> ParseResult<Statement> {
        let token = self.peek()?;
        let location = token.location;

        match &token.kind {
            TokenKind::Def => self.parse_function_def(false),
            TokenKind::Async => {
                self.advance()?;
                if self.check(&TokenKind::Def)? {
                    self.parse_function_def(true)
                } else if self.check(&TokenKind::For)? {
                    self.parse_for(true)
                } else if self.check(&TokenKind::With)? {
                    self.parse_with(true)
                } else {
                    Err(ParseError::invalid_syntax(
                        location,
                        "expected 'def', 'for', or 'with' after 'async'",
                    ))
                }
            }
            TokenKind::Class => self.parse_class_def(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Del => self.parse_del(),
            TokenKind::Pass => self.parse_pass(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(false),
            TokenKind::Try => self.parse_try(),
            TokenKind::With => self.parse_with(false),
            TokenKind::Raise => self.parse_raise(),
            TokenKind::Assert => self.parse_assert(),
            TokenKind::Import => self.parse_import(),
            TokenKind::From => self.parse_from_import(),
            TokenKind::Global => self.parse_global(),
            TokenKind::Nonlocal => self.parse_nonlocal(),
            TokenKind::Match => self.parse_match(),
            _ => self.parse_simple_stmt(),
        }
    }

    /// Parse a simple statement (expression or assignment)
    fn parse_simple_stmt(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        let expr = self.parse_expression()?;

        // Check for assignment
        if self.consume(&TokenKind::Assign)? {
            let value = self.parse_expression()?;
            self.consume(&TokenKind::Newline)?;
            return Ok(Statement::Assign {
                targets: vec![expr],
                value,
                location,
            });
        }

        // Check for augmented assignment
        if let Some(op) = self.check_aug_assign()? {
            self.advance()?;
            let value = self.parse_expression()?;
            self.consume(&TokenKind::Newline)?;
            return Ok(Statement::AugAssign {
                target: expr,
                op,
                value,
                location,
            });
        }

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Expr {
            value: expr,
            location,
        })
    }

    fn check_aug_assign(&mut self) -> ParseResult<Option<BinOp>> {
        let token = self.peek()?;
        Ok(match &token.kind {
            TokenKind::PlusEqual => Some(BinOp::Add),
            TokenKind::MinusEqual => Some(BinOp::Sub),
            TokenKind::StarEqual => Some(BinOp::Mult),
            TokenKind::SlashEqual => Some(BinOp::Div),
            TokenKind::DoubleSlashEqual => Some(BinOp::FloorDiv),
            TokenKind::PercentEqual => Some(BinOp::Mod),
            TokenKind::DoubleStarEqual => Some(BinOp::Pow),
            TokenKind::AmpersandEqual => Some(BinOp::BitAnd),
            TokenKind::PipeEqual => Some(BinOp::BitOr),
            TokenKind::CaretEqual => Some(BinOp::BitXor),
            TokenKind::LeftShiftEqual => Some(BinOp::LShift),
            TokenKind::RightShiftEqual => Some(BinOp::RShift),
            TokenKind::AtEqual => Some(BinOp::MatMult),
            _ => None,
        })
    }

    /// Parse function definition
    fn parse_function_def(&mut self, is_async: bool) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Def)?;
        
        let name = self.parse_identifier()?;
        self.expect(&TokenKind::LeftParen)?;
        let args = self.parse_arguments()?;
        self.expect(&TokenKind::RightParen)?;

        let returns = if self.consume(&TokenKind::Arrow)? {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        Ok(Statement::FunctionDef {
            name,
            args,
            body,
            decorators: Vec::new(),
            returns,
            is_async,
            location,
        })
    }

    /// Parse class definition
    fn parse_class_def(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Class)?;
        
        let name = self.parse_identifier()?;
        
        let (bases, keywords) = if self.consume(&TokenKind::LeftParen)? {
            let (bases, keywords) = self.parse_call_args()?;
            self.expect(&TokenKind::RightParen)?;
            (bases, keywords)
        } else {
            (Vec::new(), Vec::new())
        };

        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        Ok(Statement::ClassDef {
            name,
            bases,
            keywords,
            body,
            decorators: Vec::new(),
            location,
        })
    }

    /// Parse return statement
    fn parse_return(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Return)?;

        let value = if !self.check(&TokenKind::Newline)? && !self.check(&TokenKind::Eof)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Return { value, location })
    }

    /// Parse del statement
    fn parse_del(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Del)?;
        
        let mut targets = vec![self.parse_expression()?];
        while self.consume(&TokenKind::Comma)? {
            targets.push(self.parse_expression()?);
        }

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Delete { targets, location })
    }

    /// Parse pass statement
    fn parse_pass(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Pass)?;
        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Pass { location })
    }

    /// Parse break statement
    fn parse_break(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Break)?;
        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Break { location })
    }

    /// Parse continue statement
    fn parse_continue(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Continue)?;
        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Continue { location })
    }

    /// Parse if statement
    fn parse_if(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::If)?;
        
        let test = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        let orelse = if self.consume(&TokenKind::Elif)? {
            // Elif is syntactic sugar for else: if
            let elif_stmt = self.parse_elif()?;
            vec![elif_stmt]
        } else if self.consume(&TokenKind::Else)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::If {
            test,
            body,
            orelse,
            location,
        })
    }

    fn parse_elif(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        let test = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        let orelse = if self.consume(&TokenKind::Elif)? {
            vec![self.parse_elif()?]
        } else if self.consume(&TokenKind::Else)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::If {
            test,
            body,
            orelse,
            location,
        })
    }

    /// Parse while statement
    fn parse_while(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::While)?;
        
        let test = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        let orelse = if self.consume(&TokenKind::Else)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::While {
            test,
            body,
            orelse,
            location,
        })
    }

    /// Parse for statement
    fn parse_for(&mut self, is_async: bool) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::For)?;
        
        let target = self.parse_expression()?;
        self.expect(&TokenKind::In)?;
        let iter = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        let orelse = if self.consume(&TokenKind::Else)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::For {
            target,
            iter,
            body,
            orelse,
            is_async,
            location,
        })
    }

    /// Parse try statement
    fn parse_try(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Try)?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        let mut handlers = Vec::new();
        while self.consume(&TokenKind::Except)? {
            handlers.push(self.parse_except_handler()?);
        }

        let orelse = if self.consume(&TokenKind::Else)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        let finalbody = if self.consume(&TokenKind::Finally)? {
            self.expect(&TokenKind::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::Try {
            body,
            handlers,
            orelse,
            finalbody,
            location,
        })
    }

    fn parse_except_handler(&mut self) -> ParseResult<ExceptHandler> {
        let location = self.location();
        
        let typ = if !self.check(&TokenKind::Colon)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let name = if self.consume(&TokenKind::As)? {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        Ok(ExceptHandler {
            typ,
            name,
            body,
            location,
        })
    }

    /// Parse with statement
    fn parse_with(&mut self, is_async: bool) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::With)?;
        
        let mut items = vec![self.parse_with_item()?];
        while self.consume(&TokenKind::Comma)? {
            items.push(self.parse_with_item()?);
        }

        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;

        Ok(Statement::With {
            items,
            body,
            is_async,
            location,
        })
    }

    fn parse_with_item(&mut self) -> ParseResult<WithItem> {
        let context_expr = self.parse_expression()?;
        let optional_vars = if self.consume(&TokenKind::As)? {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(WithItem {
            context_expr,
            optional_vars,
        })
    }

    /// Parse raise statement
    fn parse_raise(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Raise)?;

        let exc = if !self.check(&TokenKind::Newline)? && !self.check(&TokenKind::Eof)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let cause = if self.consume(&TokenKind::From)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Raise { exc, cause, location })
    }

    /// Parse assert statement
    fn parse_assert(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Assert)?;
        
        let test = self.parse_expression()?;
        let msg = if self.consume(&TokenKind::Comma)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Assert { test, msg, location })
    }

    /// Parse import statement
    fn parse_import(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Import)?;
        
        let mut names = vec![self.parse_alias()?];
        while self.consume(&TokenKind::Comma)? {
            names.push(self.parse_alias()?);
        }

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Import { names, location })
    }

    /// Parse from import statement
    fn parse_from_import(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::From)?;
        
        let mut level = 0;
        while self.consume(&TokenKind::Dot)? {
            level += 1;
        }

        let module = if !self.check(&TokenKind::Import)? {
            Some(self.parse_dotted_name()?)
        } else {
            None
        };

        self.expect(&TokenKind::Import)?;

        let names = if self.consume(&TokenKind::Star)? {
            vec![Alias {
                name: "*".to_string(),
                asname: None,
                location,
            }]
        } else if self.consume(&TokenKind::LeftParen)? {
            let names = self.parse_import_names()?;
            self.expect(&TokenKind::RightParen)?;
            names
        } else {
            self.parse_import_names()?
        };

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::ImportFrom {
            module,
            names,
            level,
            location,
        })
    }

    fn parse_import_names(&mut self) -> ParseResult<Vec<Alias>> {
        let mut names = vec![self.parse_alias()?];
        while self.consume(&TokenKind::Comma)? {
            if self.check(&TokenKind::RightParen)? || self.check(&TokenKind::Newline)? {
                break;
            }
            names.push(self.parse_alias()?);
        }
        Ok(names)
    }

    fn parse_alias(&mut self) -> ParseResult<Alias> {
        let location = self.location();
        let name = self.parse_dotted_name()?;
        let asname = if self.consume(&TokenKind::As)? {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Alias { name, asname, location })
    }

    fn parse_dotted_name(&mut self) -> ParseResult<String> {
        let mut name = self.parse_identifier()?;
        while self.consume(&TokenKind::Dot)? {
            name.push('.');
            name.push_str(&self.parse_identifier()?);
        }
        Ok(name)
    }

    /// Parse global statement
    fn parse_global(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Global)?;
        
        let mut names = vec![self.parse_identifier()?];
        while self.consume(&TokenKind::Comma)? {
            names.push(self.parse_identifier()?);
        }

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Global { names, location })
    }

    /// Parse nonlocal statement
    fn parse_nonlocal(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Nonlocal)?;
        
        let mut names = vec![self.parse_identifier()?];
        while self.consume(&TokenKind::Comma)? {
            names.push(self.parse_identifier()?);
        }

        self.consume(&TokenKind::Newline)?;
        Ok(Statement::Nonlocal { names, location })
    }

    /// Parse match statement (stub)
    fn parse_match(&mut self) -> ParseResult<Statement> {
        let location = self.location();
        self.expect(&TokenKind::Match)?;
        let subject = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        self.expect(&TokenKind::Newline)?;
        self.expect(&TokenKind::Indent)?;
        
        // For now, just skip the match body
        let mut depth = 1;
        while depth > 0 {
            let token = self.advance()?;
            match token.kind {
                TokenKind::Indent => depth += 1,
                TokenKind::Dedent => depth -= 1,
                TokenKind::Eof => break,
                _ => {}
            }
        }

        Ok(Statement::Match {
            subject,
            cases: Vec::new(),
            location,
        })
    }
