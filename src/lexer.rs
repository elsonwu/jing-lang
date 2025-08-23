use crate::error::{JingError, JingResult};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    If,
    Else,
    While,
    Fn,
    Return,
    True,
    False,
    Nil,
    And,
    Or,
    Not,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,

    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Token { token_type, line }
    }
}

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            current: 0,
            line: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> JingResult<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line));
        Ok(tokens)
    }

    /// Get the next token
    fn next_token(&mut self) -> JingResult<Option<Token>> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(None);
        }

        let start_line = self.line;
        let c = self.advance();

        match c {
            '(' => Ok(Some(Token::new(TokenType::LeftParen, start_line))),
            ')' => Ok(Some(Token::new(TokenType::RightParen, start_line))),
            '{' => Ok(Some(Token::new(TokenType::LeftBrace, start_line))),
            '}' => Ok(Some(Token::new(TokenType::RightBrace, start_line))),
            ';' => Ok(Some(Token::new(TokenType::Semicolon, start_line))),
            ',' => Ok(Some(Token::new(TokenType::Comma, start_line))),
            '+' => Ok(Some(Token::new(TokenType::Plus, start_line))),
            '-' => Ok(Some(Token::new(TokenType::Minus, start_line))),
            '*' => Ok(Some(Token::new(TokenType::Star, start_line))),
            '/' => {
                if self.match_char('/') {
                    // Single-line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.next_token()
                } else {
                    Ok(Some(Token::new(TokenType::Slash, start_line)))
                }
            }
            '%' => Ok(Some(Token::new(TokenType::Percent, start_line))),
            '!' => {
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::BangEqual, start_line)))
                } else {
                    Ok(Some(Token::new(TokenType::Bang, start_line)))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::EqualEqual, start_line)))
                } else {
                    Ok(Some(Token::new(TokenType::Equal, start_line)))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::LessEqual, start_line)))
                } else {
                    Ok(Some(Token::new(TokenType::Less, start_line)))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::GreaterEqual, start_line)))
                } else {
                    Ok(Some(Token::new(TokenType::Greater, start_line)))
                }
            }
            '&' => {
                if self.match_char('&') {
                    Ok(Some(Token::new(TokenType::And, start_line)))
                } else {
                    Err(JingError::lex_error(
                        format!("Unexpected character: '{}'", c),
                        start_line,
                    ))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Ok(Some(Token::new(TokenType::Or, start_line)))
                } else {
                    Err(JingError::lex_error(
                        format!("Unexpected character: '{}'", c),
                        start_line,
                    ))
                }
            }
            '"' => self.string(start_line),
            '\n' => {
                self.line += 1;
                Ok(Some(Token::new(TokenType::Newline, start_line)))
            }
            c if c.is_ascii_digit() => self.number(start_line),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(start_line),
            _ => Err(JingError::lex_error(
                format!("Unexpected character: '{}'", c),
                start_line,
            )),
        }
    }

    /// Parse a string literal
    fn string(&mut self, start_line: usize) -> JingResult<Option<Token>> {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            let c = self.advance();
            if c == '\\' && !self.is_at_end() {
                // Handle escape sequences
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    c => {
                        value.push('\\');
                        value.push(c);
                    }
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return Err(JingError::lex_error("Unterminated string", start_line));
        }

        // Consume the closing "
        self.advance();

        Ok(Some(Token::new(TokenType::String(value), start_line)))
    }

    /// Parse a number literal
    fn number(&mut self, start_line: usize) -> JingResult<Option<Token>> {
        let start = self.current - 1;

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for decimal part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consume the '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_str: String = self.input[start..self.current].iter().collect();
        let value = number_str.parse::<f64>().map_err(|_| {
            JingError::lex_error(format!("Invalid number: {}", number_str), start_line)
        })?;

        Ok(Some(Token::new(TokenType::Number(value), start_line)))
    }

    /// Parse an identifier or keyword
    fn identifier(&mut self, start_line: usize) -> JingResult<Option<Token>> {
        let start = self.current - 1;

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.input[start..self.current].iter().collect();

        let token_type = match text.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            _ => TokenType::Identifier(text),
        };

        Ok(Some(Token::new(token_type, start_line)))
    }

    /// Skip whitespace characters (except newlines)
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Check if we're at the end of input
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    /// Get the current character without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }

    /// Get the next character without advancing
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.current + 1]
        }
    }

    /// Advance to the next character
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let c = self.input[self.current];
            self.current += 1;
            c
        }
    }

    /// Check if the current character matches and advance if it does
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 6); // let, x, =, 42, ;, EOF

        match &tokens[0].token_type {
            TokenType::Let => (),
            _ => panic!("Expected Let token"),
        }

        match &tokens[1].token_type {
            TokenType::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected Identifier token"),
        }
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"Hello, World!\"");
        let tokens = lexer.tokenize().unwrap();

        match &tokens[0].token_type {
            TokenType::String(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn test_number_literal() {
        let mut lexer = Lexer::new("123.45");
        let tokens = lexer.tokenize().unwrap();

        match &tokens[0].token_type {
            TokenType::Number(n) => assert_eq!(*n, 123.45),
            _ => panic!("Expected Number token"),
        }
    }
}
