use crate::error::{JingError, JingResult};
use crate::lexer::{Token, TokenType};

/// Abstract Syntax Tree node types
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(LiteralExpr),
    Variable(VariableExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Logical(LogicalExpr),
    Assign(AssignExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignExpr {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpr {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: LogicalOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

/// Statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Let(LetStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
    Print(PrintStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub name: String,
    pub initializer: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStmt {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintStmt {
    pub expr: Expr,
}

/// Parser for Jing
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse a program (list of statements)
    pub fn parse(&mut self) -> JingResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.match_token(&TokenType::Newline) {
                continue;
            }

            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    /// Parse a declaration
    fn declaration(&mut self) -> JingResult<Stmt> {
        if self.match_token(&TokenType::Let) {
            self.let_declaration()
        } else if self.match_token(&TokenType::Fn) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    /// Parse a let declaration
    fn let_declaration(&mut self) -> JingResult<Stmt> {
        let name = self.consume_identifier("Expected variable name")?;

        self.consume(&TokenType::Equal, "Expected '=' after variable name")?;

        let initializer = self.expression()?;

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;

        Ok(Stmt::Let(LetStmt { name, initializer }))
    }

    /// Parse a function declaration
    fn function_declaration(&mut self) -> JingResult<Stmt> {
        let name = self.consume_identifier("Expected function name")?;

        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;

        let body = Box::new(self.block_statement()?);

        Ok(Stmt::Function(FunctionStmt { name, params, body }))
    }

    /// Parse a statement
    fn statement(&mut self) -> JingResult<Stmt> {
        if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While) {
            self.while_statement()
        } else if self.match_token(&TokenType::Return) {
            self.return_statement()
        } else if self.match_token(&TokenType::LeftBrace) {
            Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }))
        } else {
            self.expression_statement()
        }
    }

    /// Parse an if statement
    fn if_statement(&mut self) -> JingResult<Stmt> {
        let condition = self.expression()?;
        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    /// Parse a while statement
    fn while_statement(&mut self) -> JingResult<Stmt> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While(WhileStmt { condition, body }))
    }

    /// Parse a return statement
    fn return_statement(&mut self) -> JingResult<Stmt> {
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::Semicolon, "Expected ';' after return value")?;

        Ok(Stmt::Return(ReturnStmt { value }))
    }

    /// Parse a block statement
    fn block_statement(&mut self) -> JingResult<Stmt> {
        self.consume(&TokenType::LeftBrace, "Expected '{'")?;
        let statements = self.block()?;
        Ok(Stmt::Block(BlockStmt { statements }))
    }

    /// Parse statements inside a block
    fn block(&mut self) -> JingResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    /// Parse an expression statement
    fn expression_statement(&mut self) -> JingResult<Stmt> {
        let expr = self.expression()?;

        // Check for print function calls and convert to print statements
        if let Expr::Call(call_expr) = &expr {
            if let Expr::Variable(var) = call_expr.callee.as_ref() {
                if var.name == "print" && call_expr.args.len() == 1 {
                    self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
                    return Ok(Stmt::Print(PrintStmt {
                        expr: call_expr.args[0].clone(),
                    }));
                }
            }
        }

        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expr }))
    }

    /// Parse an expression
    fn expression(&mut self) -> JingResult<Expr> {
        self.assignment()
    }

    /// Parse assignment expressions
    fn assignment(&mut self) -> JingResult<Expr> {
        let expr = self.logical_or()?;

        if self.match_token(&TokenType::Equal) {
            let value = self.assignment()?;
            
            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: var.name,
                    value: Box::new(value),
                }));
            } else {
                return Err(JingError::parse_error("Invalid assignment target", 0));
            }
        }

        Ok(expr)
    }

    /// Parse logical OR
    fn logical_or(&mut self) -> JingResult<Expr> {
        let mut expr = self.logical_and()?;

        while self.match_token(&TokenType::Or) {
            let right = self.logical_and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator: LogicalOperator::Or,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse logical AND
    fn logical_and(&mut self) -> JingResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::And) {
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator: LogicalOperator::And,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse equality operations
    fn equality(&mut self) -> JingResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_equality_operator() {
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse comparison operations
    fn comparison(&mut self) -> JingResult<Expr> {
        let mut expr = self.term()?;

        while let Some(operator) = self.match_comparison_operator() {
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse addition and subtraction
    fn term(&mut self) -> JingResult<Expr> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_term_operator() {
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse multiplication, division, and modulo
    fn factor(&mut self) -> JingResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_factor_operator() {
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse unary operations
    fn unary(&mut self) -> JingResult<Expr> {
        if let Some(operator) = self.match_unary_operator() {
            let expr = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                operand: Box::new(expr),
            }));
        }

        self.call()
    }

    /// Parse function calls
    fn call(&mut self) -> JingResult<Expr> {
        let mut expr = self.primary()?;

        while self.match_token(&TokenType::LeftParen) {
            let mut args = Vec::new();

            if !self.check(&TokenType::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;

            expr = Expr::Call(CallExpr {
                callee: Box::new(expr),
                args,
            });
        }

        Ok(expr)
    }

    /// Parse primary expressions
    fn primary(&mut self) -> JingResult<Expr> {
        if self.match_token(&TokenType::True) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LiteralValue::Bool(true),
            }));
        }

        if self.match_token(&TokenType::False) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LiteralValue::Bool(false),
            }));
        }

        if self.match_token(&TokenType::Nil) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LiteralValue::Nil,
            }));
        }

        if let TokenType::Number(value) = &self.peek().token_type {
            let value = *value;
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                value: LiteralValue::Number(value),
            }));
        }

        if let TokenType::String(value) = &self.peek().token_type {
            let value = value.clone();
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                value: LiteralValue::String(value),
            }));
        }

        if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            return Ok(Expr::Variable(VariableExpr { name }));
        }

        if self.match_token(&TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }

        Err(JingError::parse_error(
            "Expected expression",
            self.current_line(),
        ))
    }

    // Helper methods for operator matching
    fn match_equality_operator(&mut self) -> Option<BinaryOperator> {
        if self.match_token(&TokenType::BangEqual) {
            Some(BinaryOperator::NotEqual)
        } else if self.match_token(&TokenType::EqualEqual) {
            Some(BinaryOperator::Equal)
        } else {
            None
        }
    }

    fn match_comparison_operator(&mut self) -> Option<BinaryOperator> {
        if self.match_token(&TokenType::Greater) {
            Some(BinaryOperator::Greater)
        } else if self.match_token(&TokenType::GreaterEqual) {
            Some(BinaryOperator::GreaterEqual)
        } else if self.match_token(&TokenType::Less) {
            Some(BinaryOperator::Less)
        } else if self.match_token(&TokenType::LessEqual) {
            Some(BinaryOperator::LessEqual)
        } else {
            None
        }
    }

    fn match_term_operator(&mut self) -> Option<BinaryOperator> {
        if self.match_token(&TokenType::Minus) {
            Some(BinaryOperator::Subtract)
        } else if self.match_token(&TokenType::Plus) {
            Some(BinaryOperator::Add)
        } else {
            None
        }
    }

    fn match_factor_operator(&mut self) -> Option<BinaryOperator> {
        if self.match_token(&TokenType::Slash) {
            Some(BinaryOperator::Divide)
        } else if self.match_token(&TokenType::Star) {
            Some(BinaryOperator::Multiply)
        } else if self.match_token(&TokenType::Percent) {
            Some(BinaryOperator::Modulo)
        } else {
            None
        }
    }

    fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        if self.match_token(&TokenType::Bang) || self.match_token(&TokenType::Not) {
            Some(UnaryOperator::Not)
        } else if self.match_token(&TokenType::Minus) {
            Some(UnaryOperator::Minus)
        } else {
            None
        }
    }

    // Utility methods
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check_token_type(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }

    fn check_token_type(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }

    #[allow(dead_code)]
    fn advance_if_match(&mut self, token_type: &TokenType) -> Option<Token> {
        if self.check_token_type(token_type) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Token {
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

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> JingResult<Token> {
        if self.check_token_type(token_type) {
            Ok(self.advance())
        } else {
            Err(JingError::parse_error(message, self.current_line()))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> JingResult<String> {
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(JingError::parse_error(message, self.current_line()))
        }
    }

    fn current_line(&self) -> usize {
        if self.is_at_end() {
            if self.tokens.is_empty() {
                1
            } else {
                self.tokens[self.tokens.len() - 1].line
            }
        } else {
            self.peek().line
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_let_statement() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Stmt::Let(let_stmt) => {
                assert_eq!(let_stmt.name, "x");
                match &let_stmt.initializer {
                    Expr::Literal(lit) => match &lit.value {
                        LiteralValue::Number(n) => assert_eq!(*n, 42.0),
                        _ => panic!("Expected number literal"),
                    },
                    _ => panic!("Expected literal expression"),
                }
            }
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let mut lexer = Lexer::new("let result = 10 + 5 * 2;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        // This should parse as 10 + (5 * 2) due to operator precedence
        match &statements[0] {
            Stmt::Let(let_stmt) => {
                match &let_stmt.initializer {
                    Expr::Binary(binary) => {
                        assert_eq!(binary.operator, BinaryOperator::Add);
                        // Left should be 10
                        match binary.left.as_ref() {
                            Expr::Literal(lit) => match &lit.value {
                                LiteralValue::Number(n) => assert_eq!(*n, 10.0),
                                _ => panic!("Expected number literal"),
                            },
                            _ => panic!("Expected literal expression"),
                        }
                        // Right should be (5 * 2)
                        match binary.right.as_ref() {
                            Expr::Binary(right_binary) => {
                                assert_eq!(right_binary.operator, BinaryOperator::Multiply);
                            }
                            _ => panic!("Expected binary expression on right"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected let statement"),
        }
    }
}
