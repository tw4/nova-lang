use crate::ast::*;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof,
    UnexpectedTokenAt { message: String, line: usize, column: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected token: {}", msg),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::UnexpectedTokenAt { message, line, column } => {
                write!(f, "Error at line {}, column {}: {}", line, column, message)
            }
        }
    }
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    fn check(&self, token_type: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token_type)
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: Token, message: &str) -> ParseResult<()> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(message.to_string()))
        }
    }

    pub fn parse(&mut self) -> ParseResult<Program> {
        let mut statements = Vec::new();
        
        while !self.check(&Token::Eof) {
            if self.match_token(&Token::Newline) {
                continue;
            }
            statements.push(self.statement()?);
        }
        
        Ok(Program { statements })
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.match_token(&Token::Let) {
            self.let_statement()
        } else if self.match_token(&Token::Fn) {
            self.function_statement()
        } else if self.match_token(&Token::Class) {
            self.class_statement()
        } else if self.match_token(&Token::Return) {
            self.return_statement()
        } else if self.match_token(&Token::Break) {
            self.match_token(&Token::Semicolon);
            Ok(Stmt::Break)
        } else if self.match_token(&Token::Continue) {
            self.match_token(&Token::Semicolon);
            Ok(Stmt::Continue)
        } else if self.match_token(&Token::Import) {
            self.import_statement()
        } else {
            let expr = self.expression()?;
            self.match_token(&Token::Semicolon);
            Ok(Stmt::Expression(expr))
        }
    }

    fn let_statement(&mut self) -> ParseResult<Stmt> {
        let name = match self.current_token() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::UnexpectedToken("Expected identifier".to_string())),
        };
        self.advance();
        
        self.consume(Token::Equal, "Expected '=' after variable name")?;
        let value = self.expression()?;
        self.match_token(&Token::Semicolon);
        
        Ok(Stmt::Let { name, value })
    }

    fn function_statement(&mut self) -> ParseResult<Stmt> {
        let name = match self.current_token() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::UnexpectedToken("Expected function name".to_string())),
        };
        self.advance();
        
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        while !self.check(&Token::RightParen) {
            match self.current_token() {
                Token::Identifier(param) => {
                    params.push(param.clone());
                    self.advance();
                }
                _ => return Err(ParseError::UnexpectedToken("Expected parameter name".to_string())),
            }
            
            if !self.check(&Token::RightParen) {
                self.consume(Token::Comma, "Expected ',' between parameters")?;
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        let body = self.expression()?;
        
        Ok(Stmt::Function { name, params, body })
    }

    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let value = if self.check(&Token::Semicolon) || self.check(&Token::Newline) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.match_token(&Token::Semicolon);
        Ok(Stmt::Return(value))
    }

    fn import_statement(&mut self) -> ParseResult<Stmt> {
        let module = match self.current_token() {
            Token::String(module_path) => module_path.clone(),
            Token::Identifier(module_name) => module_name.clone(),
            _ => return Err(ParseError::UnexpectedToken("Expected module name or path".to_string())),
        };
        self.advance();
        
        let mut alias = None;
        if self.match_token(&Token::As) {
            match self.current_token() {
                Token::Identifier(alias_name) => {
                    alias = Some(alias_name.clone());
                    self.advance();
                }
                _ => return Err(ParseError::UnexpectedToken("Expected identifier after 'as'".to_string())),
            }
        }
        
        self.match_token(&Token::Semicolon);
        Ok(Stmt::Import { module, alias })
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;
        
        if self.match_token(&Token::Equal) {
            let value = self.assignment()?;
            return Ok(Expr::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }
        
        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;
        
        while self.match_token(&Token::Or) {
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.bitwise_or()?;
        
        while self.match_token(&Token::And) {
            let right = self.bitwise_or()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn bitwise_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.bitwise_xor()?;
        
        while self.match_token(&Token::Pipe) {
            let right = self.bitwise_xor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::BitwiseOr,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.bitwise_and()?;
        
        while self.match_token(&Token::Caret) {
            let right = self.bitwise_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::BitwiseXor,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn bitwise_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;
        
        while self.match_token(&Token::Ampersand) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::BitwiseAnd,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;
        
        loop {
            let token = self.current_token().clone();
            match token {
                Token::EqualEqual => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOp::Equal,
                        right: Box::new(right),
                    };
                }
                Token::BangEqual => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: BinaryOp::NotEqual,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.shift()?;
        
        loop {
            let token = self.current_token().clone();
            let operator = match token {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };
            
            self.advance();
            let right = self.shift()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn shift(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;
        
        loop {
            let token = self.current_token().clone();
            let operator = match token {
                Token::LeftShift => BinaryOp::LeftShift,
                Token::RightShift => BinaryOp::RightShift,
                _ => break,
            };
            
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;
        
        loop {
            let token = self.current_token().clone();
            let operator = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };
            
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.power()?;
        
        loop {
            let token = self.current_token().clone();
            let operator = match token {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => break,
            };
            
            self.advance();
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn power(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;
        
        if self.match_token(&Token::DoubleStar) {
            let right = self.power()?; // Right associative
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Power,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        match self.current_token().clone() {
            Token::Bang => {
                self.advance();
                let operand = self.unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Minus => {
                self.advance();
                let operand = self.unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Minus,
                    operand: Box::new(operand),
                })
            }
            Token::Plus => {
                self.advance();
                let operand = self.unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Plus,
                    operand: Box::new(operand),
                })
            }
            Token::Tilde => {
                self.advance();
                let operand = self.unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::BitwiseNot,
                    operand: Box::new(operand),
                })
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&Token::LeftParen) {
                let mut args = Vec::new();
                
                if !self.check(&Token::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::RightParen, "Expected ')' after arguments")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.match_token(&Token::LeftBracket) {
                let index = self.expression()?;
                self.consume(Token::RightBracket, "Expected ']' after array index")?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&Token::Dot) {
                match self.current_token() {
                    Token::Identifier(property) => {
                        let prop_name = property.clone();
                        self.advance();
                        expr = Expr::Property {
                            object: Box::new(expr),
                            property: prop_name,
                        };
                    }
                    _ => return Err(ParseError::UnexpectedToken("Expected property name after '.'".to_string())),
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        match self.current_token().clone() {
            Token::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(false)))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Number(n)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Token::This => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Super => {
                self.advance();
                Ok(Expr::Super)
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            Token::LeftBrace => {
                self.advance();
                
                // Check if this is an object literal or a block
                // If next token is a string/identifier followed by colon, it's an object
                let is_object = match (self.tokens.get(self.current), self.tokens.get(self.current + 1)) {
                    (Some(Token::String(_)), Some(Token::Colon)) => true,
                    (Some(Token::Identifier(_)), Some(Token::Colon)) => true,
                    (Some(Token::RightBrace), _) => true, // Empty object
                    _ => false,
                };
                
                if is_object {
                    // Parse object literal
                    let mut pairs = Vec::new();
                    
                    while !self.check(&Token::RightBrace) && !self.check(&Token::Eof) {
                        let key = match self.current_token() {
                            Token::String(s) => {
                                let key = s.clone();
                                self.advance();
                                key
                            }
                            Token::Identifier(s) => {
                                let key = s.clone();
                                self.advance();
                                key
                            }
                            _ => return Err(ParseError::UnexpectedToken("Expected string or identifier for object key".to_string())),
                        };
                        
                        self.consume(Token::Colon, "Expected ':' after object key")?;
                        let value = self.expression()?;
                        pairs.push((key, value));
                        
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    
                    self.consume(Token::RightBrace, "Expected '}' after object literal")?;
                    Ok(Expr::Object(pairs))
                } else {
                    // Parse block
                    let mut statements = Vec::new();
                    
                    while !self.check(&Token::RightBrace) && !self.check(&Token::Eof) {
                        if self.match_token(&Token::Newline) {
                            continue;
                        }
                        statements.push(self.statement()?);
                    }
                    
                    self.consume(Token::RightBrace, "Expected '}' after block")?;
                    Ok(Expr::Block(statements))
                }
            }
            Token::Try => {
                self.advance();
                let body = self.expression()?;
                
                let mut catch = None;
                if self.match_token(&Token::Catch) {
                    self.consume(Token::LeftParen, "Expected '(' after 'catch'")?;
                    let catch_var = match self.current_token() {
                        Token::Identifier(name) => name.clone(),
                        _ => return Err(ParseError::UnexpectedToken("Expected identifier in catch clause".to_string())),
                    };
                    self.advance();
                    self.consume(Token::RightParen, "Expected ')' after catch variable")?;
                    let catch_block = self.expression()?;
                    catch = Some((catch_var, Box::new(catch_block)));
                }
                
                let mut finally = None;
                if self.match_token(&Token::Finally) {
                    finally = Some(Box::new(self.expression()?));
                }
                
                Ok(Expr::Try {
                    body: Box::new(body),
                    catch,
                    finally,
                })
            }
            Token::Throw => {
                self.advance();
                let expr = self.expression()?;
                Ok(Expr::Throw(Box::new(expr)))
            }
            Token::If => {
                self.advance();
                let condition = self.expression()?;
                let then_branch = self.expression()?;
                let else_branch = if self.match_token(&Token::Else) {
                    Some(Box::new(self.expression()?))
                } else {
                    None
                };
                
                Ok(Expr::If {
                    condition: Box::new(condition),
                    then_branch: Box::new(then_branch),
                    else_branch,
                })
            }
            Token::While => {
                self.advance();
                let condition = self.expression()?;
                let body = self.expression()?;
                
                Ok(Expr::While {
                    condition: Box::new(condition),
                    body: Box::new(body),
                })
            }
            Token::For => {
                self.advance();
                let variable = match self.current_token() {
                    Token::Identifier(name) => name.clone(),
                    _ => return Err(ParseError::UnexpectedToken("Expected variable name".to_string())),
                };
                self.advance();
                
                self.consume(Token::In, "Expected 'in' after for variable")?;
                let iterable = self.expression()?;
                let body = self.expression()?;
                
                Ok(Expr::For {
                    variable,
                    iterable: Box::new(iterable),
                    body: Box::new(body),
                })
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::RightBracket, "Expected ']' after array elements")?;
                Ok(Expr::Array(elements))
            }
            Token::New => {
                self.advance();
                let class = self.primary()?;
                
                // Parse constructor arguments
                let args = if self.match_token(&Token::LeftParen) {
                    let mut args = Vec::new();
                    if !self.check(&Token::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RightParen, "Expected ')' after constructor arguments")?;
                    args
                } else {
                    Vec::new()
                };
                
                Ok(Expr::New {
                    class: Box::new(class),
                    args,
                })
            }
            Token::StringStart => {
                self.advance();
                let mut parts = Vec::new();
                
                loop {
                    match self.current_token().clone() {
                        Token::StringMiddle(s) => {
                            parts.push(Expr::Literal(Literal::String(s)));
                            self.advance();
                        }
                        Token::StringEnd(s) => {
                            if !s.is_empty() {
                                parts.push(Expr::Literal(Literal::String(s)));
                            }
                            self.advance();
                            break;
                        }
                        Token::InterpolationStart => {
                            self.advance();
                            let expr = self.expression()?;
                            parts.push(expr);
                            self.consume(Token::InterpolationEnd, "Expected '}' after interpolation")?;
                        }
                        _ => break,
                    }
                }
                
                Ok(Expr::StringInterpolation(parts))
            }
            _ => {
                // Check if this might be a lambda function (param1, param2) => expr
                if let Token::Identifier(_) = self.current_token() {
                    if let Some(Token::DoubleArrow) = self.tokens.get(self.current + 1) {
                        // Single parameter lambda: param => expr
                        let param = match self.current_token() {
                            Token::Identifier(name) => name.clone(),
                            _ => return Err(ParseError::UnexpectedToken("Expected parameter name".to_string())),
                        };
                        self.advance(); // consume parameter
                        self.advance(); // consume =>
                        let body = self.expression()?;
                        return Ok(Expr::Lambda {
                            params: vec![param],
                            body: Box::new(body),
                        });
                    }
                }
                
                Err(ParseError::UnexpectedToken("Unexpected token".to_string()))
            }
        }
    }
    
    fn parse_lambda_params(&mut self) -> ParseResult<Vec<String>> {
        let mut params = Vec::new();
        
        if self.match_token(&Token::LeftParen) {
            // Multi-parameter lambda: (param1, param2) => expr
            while !self.check(&Token::RightParen) {
                match self.current_token() {
                    Token::Identifier(param) => {
                        params.push(param.clone());
                        self.advance();
                    }
                    _ => return Err(ParseError::UnexpectedToken("Expected parameter name".to_string())),
                }
                
                if !self.check(&Token::RightParen) {
                    self.consume(Token::Comma, "Expected ',' between parameters")?;
                }
            }
            self.consume(Token::RightParen, "Expected ')' after parameters")?;
        } else if let Token::Identifier(param) = self.current_token() {
            // Single parameter lambda: param => expr
            params.push(param.clone());
            self.advance();
        }
        
        Ok(params)
    }

    fn class_statement(&mut self) -> ParseResult<Stmt> {
        let name = match self.current_token() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::UnexpectedToken("Expected class name".to_string())),
        };
        self.advance();

        // Check for extends clause
        let superclass = if self.match_token(&Token::Extends) {
            match self.current_token() {
                Token::Identifier(superclass_name) => {
                    let superclass = superclass_name.clone();
                    self.advance();
                    Some(superclass)
                }
                _ => return Err(ParseError::UnexpectedToken("Expected superclass name".to_string())),
            }
        } else {
            None
        };

        self.consume(Token::LeftBrace, "Expected '{' before class body")?;

        let mut methods = Vec::new();
        
        while !self.check(&Token::RightBrace) && !self.check(&Token::Eof) {
            // Skip newlines in class body
            if self.match_token(&Token::Newline) {
                continue;
            }

            // Parse method (function inside class)
            let is_static = self.match_token(&Token::Static);
            
            if self.match_token(&Token::Fn) {
                let method_name = match self.current_token() {
                    Token::Identifier(name) => name.clone(),
                    Token::Constructor => "constructor".to_string(),
                    _ => return Err(ParseError::UnexpectedToken("Expected method name".to_string())),
                };
                self.advance();

                self.consume(Token::LeftParen, "Expected '(' after method name")?;

                let mut params = Vec::new();
                while !self.check(&Token::RightParen) {
                    match self.current_token() {
                        Token::Identifier(param) => {
                            params.push(param.clone());
                            self.advance();
                        }
                        _ => return Err(ParseError::UnexpectedToken("Expected parameter name".to_string())),
                    }

                    if !self.check(&Token::RightParen) {
                        self.consume(Token::Comma, "Expected ',' between parameters")?;
                    }
                }

                self.consume(Token::RightParen, "Expected ')' after parameters")?;
                let body = self.expression()?;

                let method = if is_static {
                    Stmt::Function {
                        name: format!("static_{}", method_name),
                        params,
                        body,
                    }
                } else {
                    Stmt::Function {
                        name: method_name,
                        params,
                        body,
                    }
                };

                methods.push(method);
            }
        }

        self.consume(Token::RightBrace, "Expected '}' after class body")?;

        Ok(Stmt::Class {
            name,
            superclass,
            methods,
        })
    }
}