use jing::*;

#[test]
fn test_basic_arithmetic() {
    let input = "42 + 13;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(55.0));
}

#[test]
fn test_variables() {
    let input = r#"
        let x = 10;
        let y = 20;
        x + y;
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_string_operations() {
    let input = r#"
        let name = "World";
        "Hello, " + name + "!";
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::String("Hello, World!".to_string()));
}

#[test]
fn test_boolean_operations() {
    let input = r#"
        let x = true;
        let y = false;
        x && !y;
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_comparisons() {
    let input = r#"
        let x = 10;
        let y = 5;
        x > y;
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_if_statement() {
    let input = r#"
        let x = 10;
        if (x > 5) {
            "greater";
        } else {
            "lesser";
        }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::String("greater".to_string()));
}

#[test]
fn test_while_loop() {
    let input = r#"
        let i = 0;
        let sum = 0;
        while (i < 5) {
            sum = sum + i;
            i = i + 1;
        }
        sum;
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 0+1+2+3+4
}

#[test]
fn test_function_definition_and_call() {
    let input = r#"
        fn add(a, b) {
            return a + b;
        }
        add(5, 3);
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_recursive_function() {
    let input = r#"
        fn factorial(n) {
            if (n <= 1) {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
        factorial(5);
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_complex_expressions() {
    let input = r#"
        let a = 2;
        let b = 3;
        let c = 4;
        (a + b) * c - a;
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).unwrap();
    let mut vm = VM::new();
    let result = vm.run(&chunk).unwrap();
    assert_eq!(result, Value::Number(18.0)); // (2+3)*4-2 = 5*4-2 = 18
}
