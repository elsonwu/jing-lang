# Copilot Instructions for Jing Language Implementation

## Project Overview

Jing is a simple toy programming language implemented in Rust with a complete compilation pipeline: lexer → parser → compiler → virtual machine. This is an educational project designed to demonstrate language implementation concepts with clean, well-documented code.

## Architecture & Design Patterns

### Core Pipeline
```
Source Code → Lexer → Parser → Compiler → Bytecode → Virtual Machine → Output
```

### Key Components

1. **Lexer** (`src/lexer.rs`)
   - Tokenizes source code into structured tokens
   - Handles keywords, identifiers, literals, operators, and comments
   - Uses `Token` enum with position tracking for error reporting
   - **Pattern**: Iterator-based scanning with peek-ahead capability

2. **Parser** (`src/parser.rs`) 
   - Builds Abstract Syntax Tree (AST) from tokens
   - **Pattern**: Recursive descent parser with operator precedence
   - Separates statements (`Stmt` enum) from expressions (`Expr` enum)
   - Handles control flow: if/else, while loops, blocks, functions

3. **Compiler** (`src/compiler.rs`)
   - Converts AST to stack-based bytecode instructions
   - **Pattern**: Single-pass compilation with forward jump patching
   - Generates `OpCode` instructions and manages constant pool
   - Tracks local variables and scope depth for variable resolution

4. **Virtual Machine** (`src/vm.rs`)
   - **Pattern**: Stack-based VM with instruction pointer (IP)
   - Executes bytecode with persistent global environment
   - Manages call frames for function calls and recursion
   - Includes REPL functionality for interactive sessions

5. **Value System** (`src/value.rs`)
   - **Pattern**: Tagged union for dynamic typing
   - Four core types: `Number(f64)`, `String(String)`, `Bool(bool)`, `Nil`
   - Runtime type checking with operation-specific error handling
   - Environment for variable storage with scope chain

6. **Error Handling** (`src/error.rs`)
   - **Pattern**: Result-based error propagation with `JingResult<T>`
   - Comprehensive error types: syntax, runtime, type, undefined variable
   - Position-aware error reporting with line/column information

## Code Conventions

### Rust Patterns
- Use `Result<T, JingError>` aliased as `JingResult<T>` for all fallible operations
- Prefer `match` over `if let` for enum handling when exhaustive matching adds clarity
- Use `Vec<T>` for dynamic collections, `HashMap<String, T>` for string-keyed maps
- Follow Rust naming: `snake_case` for functions/variables, `PascalCase` for types

### Error Handling Philosophy
- **Fail Fast**: Return errors immediately rather than propagating invalid state
- **Context Preservation**: Include position information (line/column) in errors
- **User-Friendly Messages**: Error messages should be clear and actionable
- **No Panics**: Use `Result` types instead of `panic!` for recoverable errors

### Testing Strategy
- **Unit Tests**: Each module has embedded `#[cfg(test)]` tests for core functionality
- **Integration Tests**: The `tests/` directory contains end-to-end pipeline tests
- **Property Testing**: Test edge cases like empty input, boundary values, malformed syntax
- **Coverage Focus**: Prioritize error paths and edge cases alongside happy paths

## Implementation Guidelines

### When Adding New Features

1. **Language Features**: Follow the pipeline pattern
   - Add tokens to `lexer.rs` with appropriate `Token` variants
   - Add AST nodes to `parser.rs` with corresponding `Stmt`/`Expr` variants
   - Add compilation logic to `compiler.rs` with new `OpCode` instructions
   - Implement execution in `vm.rs` with stack manipulation

2. **Value Types**: Extend the type system carefully
   - Add variants to `Value` enum in `value.rs`
   - Implement type checking in relevant operations
   - Update error handling for type mismatches
   - Add conversion methods and display formatting

3. **Error Cases**: Comprehensive error handling
   - Define specific error variants in `JingError` enum
   - Include position information where applicable
   - Write tests that verify error conditions
   - Ensure error messages are helpful for debugging

### API Compatibility Notes

**Critical**: Several comprehensive integration test files were created but have API compatibility issues that require attention:

- `vm.run()` is **private** - use `vm.interpret(chunk)` instead  
- `compiler.compile()` expects `Vec<Stmt>` not `&Vec<Stmt>`
- Integration tests in `tests/` directory need API fixes before they can run
- Currently CI runs only embedded unit tests (10 tests) to avoid compilation errors

**Current Status**: 
- ✅ Core library compiles and passes unit tests
- ✅ 10 embedded unit tests pass (lexer, parser, compiler, VM)
- ⚠️ Integration tests need API signature fixes
- ✅ Formatting and basic linting pass

**Correct Pattern**:
```rust
let mut vm = VM::new();
vm.interpret(chunk)?; // Not vm.run(&chunk)

let mut compiler = Compiler::new();
let chunk = compiler.compile(statements)?; // Not compile(&statements)
```

### Testing Best Practices

1. **Compilation Pipeline Tests**:
   ```rust
   // Full pipeline test pattern
   let mut lexer = Lexer::new(input);
   let tokens = lexer.tokenize()?;
   let mut parser = Parser::new(tokens);
   let statements = parser.parse()?;
   let mut compiler = Compiler::new();
   let chunk = compiler.compile(statements)?;
   let mut vm = VM::new();
   vm.interpret(chunk)?;
   ```

2. **Error Testing**:
   ```rust
   // Test error conditions
   let result = operation_that_should_fail();
   assert!(result.is_err());
   assert_matches!(result.unwrap_err(), JingError::TypeError(_));
   ```

3. **Value Testing**:
   ```rust
   // Test value equality with proper types
   assert_eq!(result, Value::Number(42.0));
   assert_eq!(result, Value::String("hello".to_string()));
   ```

## Development Workflow

### Adding New Syntax
1. Update lexer with new tokens
2. Extend parser grammar rules
3. Add AST node types
4. Implement compilation logic
5. Add VM execution support
6. Write comprehensive tests
7. Update documentation

### Debugging Tips
- Use `println!` debugging in VM to trace execution
- Print AST structure after parsing to verify correctness
- Examine generated bytecode to understand compilation issues
- Check stack state during VM execution for runtime problems

### Common Pitfalls
- **Stack Underflow**: Always ensure stack has enough values before popping
- **Scope Issues**: Track variable scope depth correctly in compiler
- **Jump Patching**: Forward jumps need proper address resolution
- **Memory Safety**: Rust's ownership system prevents most memory issues, but be careful with indices

## File Organization

- `src/main.rs`: CLI entry point and REPL implementation
- `src/lib.rs`: Public API exports and module declarations  
- `src/*.rs`: Core implementation modules (lexer, parser, compiler, vm, value, error)
- `tests/*.rs`: Integration test suites organized by functionality
- `examples/*.jing`: Example programs demonstrating language features
- `*.md`: Documentation (README, HOW_IT_WORKS, VISUAL_GUIDE)

## Educational Resources

The codebase includes excellent learning materials:
- `docs/HOW_IT_WORKS.md`: Step-by-step explanation of language implementation
- `docs/VISUAL_GUIDE.md`: Diagrams showing compilation pipeline
- Embedded tests demonstrate usage patterns
- Clear separation of concerns makes each component understandable

## Performance Considerations

This is an educational implementation prioritizing clarity over performance:
- No optimizations implemented (intentionally simple)
- Stack-based VM (easier to understand than register-based)
- Single-pass compilation (no multi-phase optimization)
- Runtime type checking (no static analysis)

When suggesting improvements, maintain educational value and code clarity as primary goals.
