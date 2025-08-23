use crate::token::{Token, TokenType};
use crate::ast::{Expr, Stmt, LiteralValue, BinaryOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        
        Ok(statements)
    }
    
    // For REPL mode - parse single expressions without requiring semicolons
    pub fn parse_repl_expression(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Let) {
            self.let_declaration()
        } else if self.match_token(&TokenType::Function) {
            self.function_declaration()
        } else if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While) {
            self.while_statement()
        } else if self.match_token(&TokenType::Return) {
            self.return_statement()
        } else if self.match_token(&TokenType::LeftBrace) {
            self.block_statement()
        } else {
            // Parse as expression statement, but don't require semicolon
            let expr = self.expression()?;
            // Consume optional semicolon or newline if present
            if self.check(&TokenType::Semicolon) || self.check(&TokenType::Newline) {
                self.advance();
            }
            Ok(Stmt::Expression(expr))
        }
    }
    
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Let) {
            self.let_declaration()
        } else if self.match_token(&TokenType::Function) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }
    
    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err("Expected variable name".to_string());
        };
        
        let initializer = if self.match_token(&TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume_semicolon()?;
        Ok(Stmt::Let { name, initializer })
    }
    
    fn function_declaration(&mut self) -> Result<Stmt, String> {
        let name = if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err("Expected function name".to_string());
        };
        
        if !self.match_token(&TokenType::LeftParen) {
            return Err("Expected '(' after function name".to_string());
        }
        
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if let TokenType::Identifier(param) = &self.peek().token_type {
                    params.push(param.clone());
                    self.advance();
                } else {
                    return Err("Expected parameter name".to_string());
                }
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        if !self.match_token(&TokenType::RightParen) {
            return Err("Expected ')' after parameters".to_string());
        }
        
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::Function { name, params, body })
    }
    
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While) {
            self.while_statement()
        } else if self.match_token(&TokenType::LeftBrace) {
            self.block_statement()
        } else if self.match_token(&TokenType::Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }
    
    fn if_statement(&mut self) -> Result<Stmt, String> {
        if !self.match_token(&TokenType::LeftParen) {
            return Err("Expected '(' after 'if'".to_string());
        }
        
        let condition = self.expression()?;
        
        if !self.match_token(&TokenType::RightParen) {
            return Err("Expected ')' after if condition".to_string());
        }
        
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        if !self.match_token(&TokenType::LeftParen) {
            return Err("Expected '(' after 'while'".to_string());
        }
        
        let condition = self.expression()?;
        
        if !self.match_token(&TokenType::RightParen) {
            return Err("Expected ')' after while condition".to_string());
        }
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While { condition, body })
    }
    
    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        
        if !self.match_token(&TokenType::RightBrace) {
            return Err("Expected '}' after block".to_string());
        }
        
        Ok(Stmt::Block(statements))
    }
    
    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume_semicolon()?;
        Ok(Stmt::Return(value))
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;
        
        if self.match_token(&TokenType::Equal) {
            let value = self.assignment()?;
            
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assignment {
                    name,
                    value: Box::new(value),
                });
            }
            
            return Err("Invalid assignment target".to_string());
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while self.match_tokens(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = match &self.previous().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = match &self.previous().token_type {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = match &self.previous().token_type {
                TokenType::Minus => BinaryOp::Subtract,
                TokenType::Plus => BinaryOp::Add,
                _ => unreachable!(),
            };
            
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while self.match_tokens(&[TokenType::Divide, TokenType::Multiply]) {
            let operator = match &self.previous().token_type {
                TokenType::Divide => BinaryOp::Divide,
                TokenType::Multiply => BinaryOp::Multiply,
                _ => unreachable!(),
            };
            
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = match &self.previous().token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                operand: Box::new(right),
            });
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        
        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        if !self.match_token(&TokenType::RightParen) {
            return Err("Expected ')' after arguments".to_string());
        }
        
        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        })
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token_type {
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(false)))
            }
            TokenType::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(value)))
            }
            TokenType::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::Literal(LiteralValue::String(value)))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Variable(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if !self.match_token(&TokenType::RightParen) {
                    return Err("Expected ')' after expression".to_string());
                }
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume_semicolon(&mut self) -> Result<(), String> {
        if self.match_token(&TokenType::Semicolon) || self.check(&TokenType::Newline) {
            Ok(())
        } else {
            Err("Expected ';' or newline".to_string())
        }
    }
}
