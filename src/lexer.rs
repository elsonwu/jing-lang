use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            let start_line = self.line;
            let start_column = self.column;
            
            if let Some(token) = self.next_token() {
                tokens.push(Token::new(token, self.get_lexeme_from_position(), start_line, start_column));
            }
        }
        
        tokens.push(Token::new(TokenType::Eof, String::new(), self.line, self.column));
        tokens
    }
    
    fn next_token(&mut self) -> Option<TokenType> {
        let ch = self.current_char()?;
        self.advance();
        
        match ch {
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Multiply),
            '/' => Some(TokenType::Divide),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ';' => Some(TokenType::Semicolon),
            ',' => Some(TokenType::Comma),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Some(TokenType::Newline)
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '"' => self.string(),
            _ if ch.is_ascii_digit() => self.number(),
            _ if ch.is_ascii_alphabetic() || ch == '_' => self.identifier(),
            _ => None, // Skip unknown characters
        }
    }
    
    fn string(&mut self) -> Option<TokenType> {
        let mut value = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                return Some(TokenType::String(value));
            }
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(ch);
            self.advance();
        }
        
        None // Unterminated string
    }
    
    fn number(&mut self) -> Option<TokenType> {
        self.position -= 1; // Step back to include the first digit
        self.column -= 1;
        
        let start = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        
        // Check for decimal point
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // consume '.'
            
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        let number_str: String = self.input[start..self.position].iter().collect();
        if let Ok(value) = number_str.parse::<f64>() {
            Some(TokenType::Number(value))
        } else {
            None
        }
    }
    
    fn identifier(&mut self) -> Option<TokenType> {
        self.position -= 1; // Step back to include the first character
        self.column -= 1;
        
        let start = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let identifier: String = self.input[start..self.position].iter().collect();
        
        // Check for keywords
        let token_type = match identifier.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "fn" => TokenType::Function,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier(identifier),
        };
        
        Some(token_type)
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn get_lexeme_from_position(&self) -> String {
        // This is a simplified version - in practice you'd want to track the start position
        String::new()
    }
}
