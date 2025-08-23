use jing::*;

#[test]
fn test_lexical_errors() {
    // Unterminated string
    let input = r#""Hello, World"#;
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err());
    match result.unwrap_err() {
        JingError::LexError { message, line } => {
            assert!(message.contains("Unterminated string"));
            assert_eq!(line, 1);
        }
        _ => panic!("Expected LexError"),
    }

    // Unexpected character
    let input = "@#$";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err());
    match result.unwrap_err() {
        JingError::LexError { message, .. } => {
            assert!(message.contains("Unexpected character"));
        }
        _ => panic!("Expected LexError"),
    }

    // Invalid logical operators
    let input = "&";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err());

    let input = "|";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_parse_errors() {
    // Missing semicolon
    let input = "let x = 42";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    match result.unwrap_err() {
        JingError::ParseError { message, .. } => {
            assert!(message.contains("Expected ';'"));
        }
        _ => panic!("Expected ParseError"),
    }

    // Missing variable name
    let input = "let = 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());

    // Missing function body
    let input = "fn test() ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_runtime_errors() {
    // Division by zero
    let input = "10 / 0;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());
    match result.unwrap_err() {
        JingError::RuntimeError { message } => {
            assert!(message.contains("Division by zero"));
        }
        _ => panic!("Expected RuntimeError"),
    }

    // Modulo by zero
    let input = "10 % 0;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());

    // Undefined variable
    let input = "undefined_var;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());
    match result.unwrap_err() {
        JingError::RuntimeError { message } => {
            assert!(message.contains("Undefined variable"));
        }
        _ => panic!("Expected RuntimeError"),
    }
}

#[test]
fn test_type_errors() {
    // Cannot add number and bool
    let input = "5 + true;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());

    // Cannot subtract strings
    let input = r#""hello" - "world";"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());

    // Cannot multiply strings
    let input = r#""hello" * "world";"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());

    // Cannot negate string
    let input = r#"-"hello";"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.is_err());
}

#[test]
fn test_error_display() {
    let lex_error = JingError::lex_error("Test lexical error", 5);
    let display = format!("{}", lex_error);
    assert!(display.contains("Lexical error at line 5"));
    assert!(display.contains("Test lexical error"));

    let parse_error = JingError::parse_error("Test parse error", 10);
    let display = format!("{}", parse_error);
    assert!(display.contains("Parse error at line 10"));
    assert!(display.contains("Test parse error"));

    let compile_error = JingError::compile_error("Test compile error");
    let display = format!("{}", compile_error);
    assert!(display.contains("Compilation error"));
    assert!(display.contains("Test compile error"));

    let runtime_error = JingError::runtime_error("Test runtime error");
    let display = format!("{}", runtime_error);
    assert!(display.contains("Runtime error"));
    assert!(display.contains("Test runtime error"));

    let type_error = JingError::type_error("Test type error");
    let display = format!("{}", type_error);
    assert!(display.contains("Type error"));
    assert!(display.contains("Test type error"));

    let io_error = JingError::io_error("Test I/O error");
    let display = format!("{}", io_error);
    assert!(display.contains("I/O error"));
    assert!(display.contains("Test I/O error"));
}

#[test]
fn test_error_equality() {
    let error1 = JingError::runtime_error("Test error");
    let error2 = JingError::runtime_error("Test error");
    let error3 = JingError::runtime_error("Different error");
    
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}
