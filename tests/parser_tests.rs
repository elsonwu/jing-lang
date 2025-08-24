use jing::lexer::Lexer;
use jing::parser::{
    BinaryOperator, Expr, LiteralValue, LogicalOperator, Parser, Stmt, UnaryOperator,
};

#[test]
fn test_parse_literals() {
    // Number literal
    let input = "42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::Number(n) => assert_eq!(*n, 42.0),
                _ => panic!("Expected number literal"),
            },
            _ => panic!("Expected literal expression"),
        },
        _ => panic!("Expected expression statement"),
    }

    // String literal
    let input = r#""hello";"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::String(s) => assert_eq!(s, "hello"),
                _ => panic!("Expected string literal"),
            },
            _ => panic!("Expected literal expression"),
        },
        _ => panic!("Expected expression statement"),
    }

    // Boolean literals
    let input = "true;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::Bool(b) => assert_eq!(*b, true),
                _ => panic!("Expected bool literal"),
            },
            _ => panic!("Expected literal expression"),
        },
        _ => panic!("Expected expression statement"),
    }

    // Nil literal
    let input = "nil;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::Nil => (),
                _ => panic!("Expected nil literal"),
            },
            _ => panic!("Expected literal expression"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_variables() {
    let input = "x;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Variable(var) => {
                assert_eq!(var.name, "x");
            }
            _ => panic!("Expected variable expression"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_binary_expressions() {
    let test_cases = vec![
        ("1 + 2;", BinaryOperator::Add),
        ("1 - 2;", BinaryOperator::Subtract),
        ("1 * 2;", BinaryOperator::Multiply),
        ("1 / 2;", BinaryOperator::Divide),
        ("1 % 2;", BinaryOperator::Modulo),
        ("1 == 2;", BinaryOperator::Equal),
        ("1 != 2;", BinaryOperator::NotEqual),
        ("1 < 2;", BinaryOperator::Less),
        ("1 <= 2;", BinaryOperator::LessEqual),
        ("1 > 2;", BinaryOperator::Greater),
        ("1 >= 2;", BinaryOperator::GreaterEqual),
    ];

    for (input, expected_op) in test_cases {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        match &statements[0] {
            Stmt::Expression(expr_stmt) => {
                match &expr_stmt.expr {
                    Expr::Binary(binary) => {
                        assert_eq!(binary.operator, expected_op);

                        // Check left operand
                        match &*binary.left {
                            Expr::Literal(literal) => match &literal.value {
                                LiteralValue::Number(n) => assert_eq!(*n, 1.0),
                                _ => panic!("Expected number literal"),
                            },
                            _ => panic!("Expected literal expression"),
                        }

                        // Check right operand
                        match &*binary.right {
                            Expr::Literal(literal) => match &literal.value {
                                LiteralValue::Number(n) => assert_eq!(*n, 2.0),
                                _ => panic!("Expected number literal"),
                            },
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }
}

#[test]
fn test_parse_unary_expressions() {
    // Unary minus
    let input = "-42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Unary(unary) => {
                assert_eq!(unary.operator, UnaryOperator::Minus);
                match &*unary.operand {
                    Expr::Literal(literal) => match &literal.value {
                        LiteralValue::Number(n) => assert_eq!(*n, 42.0),
                        _ => panic!("Expected number literal"),
                    },
                    _ => panic!("Expected literal expression"),
                }
            }
            _ => panic!("Expected unary expression"),
        },
        _ => panic!("Expected expression statement"),
    }

    // Logical NOT
    let input = "!true;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Unary(unary) => {
                assert_eq!(unary.operator, UnaryOperator::Not);
            }
            _ => panic!("Expected unary expression"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_logical_expressions() {
    let input = "true && false;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Logical(logical) => {
                assert_eq!(logical.operator, LogicalOperator::And);
            }
            _ => panic!("Expected logical expression"),
        },
        _ => panic!("Expected expression statement"),
    }

    let input = "true || false;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
            Expr::Logical(logical) => {
                assert_eq!(logical.operator, LogicalOperator::Or);
            }
            _ => panic!("Expected logical expression"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_let_statement() {
    let input = "let x = 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Let(let_stmt) => {
            assert_eq!(let_stmt.name, "x");
            match &let_stmt.initializer {
                Expr::Literal(literal) => match &literal.value {
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
fn test_parse_if_statement() {
    let input = r#"
    if (x > 5) {
        print("greater");
    }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::If(if_stmt) => {
            // Check condition
            match &if_stmt.condition {
                Expr::Binary(binary) => {
                    assert_eq!(binary.operator, BinaryOperator::Greater);
                }
                _ => panic!("Expected binary expression"),
            }

            // Check then branch
            match &*if_stmt.then_branch {
                Stmt::Block(_) => (),
                _ => panic!("Expected block statement"),
            }

            // No else branch
            assert!(if_stmt.else_branch.is_none());
        }
        _ => panic!("Expected if statement"),
    }

    // Test if-else
    let input = r#"
    if (x > 5) {
        print("greater");
    } else {
        print("lesser");
    }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::If(if_stmt) => {
            assert!(if_stmt.else_branch.is_some());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_while_statement() {
    let input = r#"
    while (i < 10) {
        i = i + 1;
    }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::While(while_stmt) => {
            // Check condition
            match &while_stmt.condition {
                Expr::Binary(binary) => {
                    assert_eq!(binary.operator, BinaryOperator::Less);
                }
                _ => panic!("Expected binary expression"),
            }

            // Check body
            match &*while_stmt.body {
                Stmt::Block(_) => (),
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_parse_function_statement() {
    let input = r#"
    fn add(a, b) {
        return a + b;
    }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Function(fn_stmt) => {
            assert_eq!(fn_stmt.name, "add");
            assert_eq!(fn_stmt.params.len(), 2);
            assert_eq!(fn_stmt.params[0], "a");
            assert_eq!(fn_stmt.params[1], "b");

            match &*fn_stmt.body {
                Stmt::Block(_) => (),
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected function statement"),
    }
}

#[test]
fn test_parse_return_statement() {
    // Return with value
    let input = "return 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Return(return_stmt) => {
            assert!(return_stmt.value.is_some());
            match return_stmt.value.as_ref().unwrap() {
                Expr::Literal(literal) => match &literal.value {
                    LiteralValue::Number(n) => assert_eq!(*n, 42.0),
                    _ => panic!("Expected number literal"),
                },
                _ => panic!("Expected literal expression"),
            }
        }
        _ => panic!("Expected return statement"),
    }

    // Return without value
    let input = "return;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Return(return_stmt) => {
            assert!(return_stmt.value.is_none());
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_parse_print_statement() {
    let input = r#"print("Hello, World!");"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Print(print_stmt) => match &print_stmt.expr {
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::String(s) => assert_eq!(s, "Hello, World!"),
                _ => panic!("Expected string literal"),
            },
            _ => panic!("Expected literal expression"),
        },
        _ => panic!("Expected print statement"),
    }
}

#[test]
fn test_parse_block_statement() {
    let input = r#"
    {
        let x = 42;
        let y = 13;
        x + y;
    }
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Block(block_stmt) => {
            assert_eq!(block_stmt.statements.len(), 3);

            // Check first statement (let x = 42;)
            match &block_stmt.statements[0] {
                Stmt::Let(let_stmt) => {
                    assert_eq!(let_stmt.name, "x");
                }
                _ => panic!("Expected let statement"),
            }

            // Check second statement (let y = 13;)
            match &block_stmt.statements[1] {
                Stmt::Let(let_stmt) => {
                    assert_eq!(let_stmt.name, "y");
                }
                _ => panic!("Expected let statement"),
            }

            // Check third statement (x + y;)
            match &block_stmt.statements[2] {
                Stmt::Expression(expr_stmt) => match &expr_stmt.expr {
                    Expr::Binary(binary) => {
                        assert_eq!(binary.operator, BinaryOperator::Add);
                    }
                    _ => panic!("Expected binary expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected block statement"),
    }
}

#[test]
fn test_parse_call_expression() {
    let input = "add(1, 2);";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => {
            match &expr_stmt.expr {
                Expr::Call(call) => {
                    // Check callee
                    match &*call.callee {
                        Expr::Variable(var) => {
                            assert_eq!(var.name, "add");
                        }
                        _ => panic!("Expected variable expression"),
                    }

                    // Check arguments
                    assert_eq!(call.args.len(), 2);

                    match &call.args[0] {
                        Expr::Literal(literal) => match &literal.value {
                            LiteralValue::Number(n) => assert_eq!(*n, 1.0),
                            _ => panic!("Expected number literal"),
                        },
                        _ => panic!("Expected literal expression"),
                    }

                    match &call.args[1] {
                        Expr::Literal(literal) => match &literal.value {
                            LiteralValue::Number(n) => assert_eq!(*n, 2.0),
                            _ => panic!("Expected number literal"),
                        },
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected call expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_precedence() {
    // Test that 2 + 3 * 4 is parsed as 2 + (3 * 4)
    let input = "2 + 3 * 4;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => {
            match &expr_stmt.expr {
                Expr::Binary(add_binary) => {
                    assert_eq!(add_binary.operator, BinaryOperator::Add);

                    // Left side should be 2
                    match &*add_binary.left {
                        Expr::Literal(literal) => match &literal.value {
                            LiteralValue::Number(n) => assert_eq!(*n, 2.0),
                            _ => panic!("Expected number literal"),
                        },
                        _ => panic!("Expected literal expression"),
                    }

                    // Right side should be (3 * 4)
                    match &*add_binary.right {
                        Expr::Binary(mul_binary) => {
                            assert_eq!(mul_binary.operator, BinaryOperator::Multiply);
                        }
                        _ => panic!("Expected binary expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_parenthesized_expressions() {
    let input = "(2 + 3) * 4;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().unwrap();

    match &statements[0] {
        Stmt::Expression(expr_stmt) => {
            match &expr_stmt.expr {
                Expr::Binary(mul_binary) => {
                    assert_eq!(mul_binary.operator, BinaryOperator::Multiply);

                    // Left side should be (2 + 3)
                    match &*mul_binary.left {
                        Expr::Binary(add_binary) => {
                            assert_eq!(add_binary.operator, BinaryOperator::Add);
                        }
                        _ => panic!("Expected binary expression"),
                    }

                    // Right side should be 4
                    match &*mul_binary.right {
                        Expr::Literal(literal) => match &literal.value {
                            LiteralValue::Number(n) => assert_eq!(*n, 4.0),
                            _ => panic!("Expected number literal"),
                        },
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}
