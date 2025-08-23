use jing::*;

#[test]
fn test_lexer_keywords() {
    let input = "let if else while fn return true false nil and or not";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let expected = vec![
        TokenType::Let,
        TokenType::If,
        TokenType::Else,
        TokenType::While,
        TokenType::Fn,
        TokenType::Return,
        TokenType::True,
        TokenType::False,
        TokenType::Nil,
        TokenType::And,
        TokenType::Or,
        TokenType::Not,
        TokenType::Eof,
    ];

    for (i, expected_type) in expected.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_lexer_operators() {
    let input = "+ - * / % = == ! != < <= > >= && ||";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let expected = vec![
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Percent,
        TokenType::Equal,
        TokenType::EqualEqual,
        TokenType::Bang,
        TokenType::BangEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::And,
        TokenType::Or,
        TokenType::Eof,
    ];

    for (i, expected_type) in expected.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_lexer_delimiters() {
    let input = "(){};,";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let expected = vec![
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Semicolon,
        TokenType::Comma,
        TokenType::Eof,
    ];

    for (i, expected_type) in expected.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_lexer_numbers() {
    let input = "42 3.14 0 0.0 123.456";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected Number token"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 3.14),
        _ => panic!("Expected Number token"),
    }

    match &tokens[2].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.0),
        _ => panic!("Expected Number token"),
    }

    match &tokens[3].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.0),
        _ => panic!("Expected Number token"),
    }

    match &tokens[4].token_type {
        TokenType::Number(n) => assert_eq!(*n, 123.456),
        _ => panic!("Expected Number token"),
    }
}

#[test]
fn test_lexer_strings() {
    let input = r#""hello" "world" "" "Hello, World!""#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected String token"),
    }

    match &tokens[1].token_type {
        TokenType::String(s) => assert_eq!(s, "world"),
        _ => panic!("Expected String token"),
    }

    match &tokens[2].token_type {
        TokenType::String(s) => assert_eq!(s, ""),
        _ => panic!("Expected String token"),
    }

    match &tokens[3].token_type {
        TokenType::String(s) => assert_eq!(s, "Hello, World!"),
        _ => panic!("Expected String token"),
    }
}

#[test]
fn test_lexer_string_escapes() {
    let input = r#""Hello\nWorld" "Tab\tSeparated" "Quote: \"Hi\"" "Backslash: \\""#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "Hello\nWorld"),
        _ => panic!("Expected String token"),
    }

    match &tokens[1].token_type {
        TokenType::String(s) => assert_eq!(s, "Tab\tSeparated"),
        _ => panic!("Expected String token"),
    }

    match &tokens[2].token_type {
        TokenType::String(s) => assert_eq!(s, "Quote: \"Hi\""),
        _ => panic!("Expected String token"),
    }

    match &tokens[3].token_type {
        TokenType::String(s) => assert_eq!(s, "Backslash: \\"),
        _ => panic!("Expected String token"),
    }
}

#[test]
fn test_lexer_identifiers() {
    let input = "variable _private __special test123 _test_var";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "variable"),
        _ => panic!("Expected Identifier token"),
    }

    match &tokens[1].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "_private"),
        _ => panic!("Expected Identifier token"),
    }

    match &tokens[2].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "__special"),
        _ => panic!("Expected Identifier token"),
    }

    match &tokens[3].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "test123"),
        _ => panic!("Expected Identifier token"),
    }

    match &tokens[4].token_type {
        TokenType::Identifier(name) => assert_eq!(name, "_test_var"),
        _ => panic!("Expected Identifier token"),
    }
}

#[test]
fn test_lexer_comments() {
    let input = r#"
    let x = 42; // This is a comment
    let y = 13; // Another comment
    // Full line comment
    let z = x + y;
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Comments should be completely ignored
    // We should only have tokens for the actual code
    let non_newline_tokens: Vec<TokenType> = tokens
        .iter()
        .map(|t| t.token_type.clone())
        .filter(|t| !matches!(t, TokenType::Newline | TokenType::Eof))
        .collect();

    let expected = vec![
        TokenType::Let,
        TokenType::Identifier("x".to_string()),
        TokenType::Equal,
        TokenType::Number(42.0),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Identifier("y".to_string()),
        TokenType::Equal,
        TokenType::Number(13.0),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Identifier("z".to_string()),
        TokenType::Equal,
        TokenType::Identifier("x".to_string()),
        TokenType::Plus,
        TokenType::Identifier("y".to_string()),
        TokenType::Semicolon,
    ];

    assert_eq!(non_newline_tokens.len(), expected.len());
    for (i, expected_token) in expected.iter().enumerate() {
        assert_eq!(non_newline_tokens[i], *expected_token);
    }
}

#[test]
fn test_lexer_newlines() {
    let input = "let x = 42;\nlet y = 13;\n";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Should have newline tokens
    assert!(tokens
        .iter()
        .any(|t| matches!(t.token_type, TokenType::Newline)));

    // Check line numbers
    let mut line = 1;
    for token in &tokens {
        if matches!(token.token_type, TokenType::Newline) {
            assert_eq!(token.line, line);
            line += 1;
        }
    }
}

#[test]
fn test_lexer_whitespace() {
    let input = "  let   x    =     42  ;  ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Whitespace should be ignored, only meaningful tokens should remain
    let expected = vec![
        TokenType::Let,
        TokenType::Identifier("x".to_string()),
        TokenType::Equal,
        TokenType::Number(42.0),
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (i, expected_type) in expected.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_lexer_empty_input() {
    let input = "";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Eof);
}

#[test]
fn test_lexer_only_whitespace() {
    let input = "   \n\t\r   \n  ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Should have newlines and EOF
    let newline_count = tokens
        .iter()
        .filter(|t| matches!(t.token_type, TokenType::Newline))
        .count();
    assert_eq!(newline_count, 2);
    assert!(tokens
        .iter()
        .any(|t| matches!(t.token_type, TokenType::Eof)));
}

#[test]
fn test_lexer_line_tracking() {
    let input = "let x = 1;\nlet y = 2;\nlet z = 3;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // First line tokens
    assert_eq!(tokens[0].line, 1); // let
    assert_eq!(tokens[1].line, 1); // x
    assert_eq!(tokens[2].line, 1); // =
    assert_eq!(tokens[3].line, 1); // 1
    assert_eq!(tokens[4].line, 1); // ;
    assert_eq!(tokens[5].line, 1); // \n

    // Second line tokens
    assert_eq!(tokens[6].line, 2); // let
    assert_eq!(tokens[7].line, 2); // y

    // Third line tokens should be on line 3
    let third_line_tokens: Vec<&Token> = tokens.iter().filter(|t| t.line == 3).collect();
    assert!(third_line_tokens.len() > 0);
}

#[test]
fn test_token_creation() {
    let token = Token::new(TokenType::Number(42.0), 5);
    assert_eq!(token.line, 5);
    match token.token_type {
        TokenType::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected Number token"),
    }
}

#[test]
fn test_lexer_complex_expression() {
    let input = r#"
    fn factorial(n) {
        if (n <= 1) {
            return 1;
        } else {
            return n * factorial(n - 1);
        }
    }
    
    let result = factorial(5);
    print("Result: " + result);
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Should successfully tokenize without errors
    assert!(tokens.len() > 20); // Should have many tokens
    assert!(tokens
        .iter()
        .any(|t| matches!(t.token_type, TokenType::Eof)));

    // Check that we have the expected function-related tokens
    assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::Fn)));
    assert!(tokens
        .iter()
        .any(|t| matches!(t.token_type, TokenType::Return)));
    assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::If)));
    assert!(tokens
        .iter()
        .any(|t| matches!(t.token_type, TokenType::Else)));

    // Check for specific identifiers
    assert!(tokens.iter().any(|t| {
        if let TokenType::Identifier(name) = &t.token_type {
            name == "factorial"
        } else {
            false
        }
    }));
}
