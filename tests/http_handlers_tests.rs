//! Tests for HTTP handler registration functionality

use jing::{init, Compiler, JingResult, Lexer, Parser, VM};

#[test]
fn test_http_register_handler() -> JingResult<()> {
    init();

    // Test complete workflow in single execution
    let code = r#"
        let server = start_http_server(8080);
        print(server);
        
        let result1 = http_register_handler(server, "GET", "/api/users", "get_users");
        print(result1);
        
        let result2 = http_register_handler(server, "POST", "/api/users", "create_user");
        print(result2);
        
        let result3 = http_register_handler(server, "put", "/api/users/123", "update_user");
        print(result3);
        
        let stop = stop_http_server(server);
        print(stop);
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    let mut vm = VM::new();
    vm.interpret(chunk)?;

    Ok(())
}

#[test]
fn test_http_register_handler_errors() -> JingResult<()> {
    init();

    // Test invalid HTTP method
    let code = r#"
        let server = start_http_server(8080);
        let result = http_register_handler(server, "INVALID", "/test", "handler");
        print(result);
        let stop = stop_http_server(server);
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    let mut vm = VM::new();

    // This should return an error result via JingError
    let result = vm.interpret(chunk);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_http_register_handler_wrong_arity() -> JingResult<()> {
    init();

    // Test wrong number of arguments
    let code = r#"
        let result = http_register_handler("server_handle", "GET", "/test");
        print(result);
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    let mut vm = VM::new();

    // This should fail due to wrong arity
    let result = vm.interpret(chunk);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_http_register_handler_no_server() -> JingResult<()> {
    init();

    // Test registering handler on non-existent server
    let code = r#"
        let result = http_register_handler("nonexistent_handle", "GET", "/test", "handler");
        print(result);
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    let mut vm = VM::new();

    // This should fail because no server is running on port 8888
    let result = vm.interpret(chunk);
    assert!(result.is_err());

    Ok(())
}
