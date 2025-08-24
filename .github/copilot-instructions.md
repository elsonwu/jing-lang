# Copilot Instructions for Jing Language Implementation

## Project Overview

Jing is a simple toy programming language implemented in Rust with a complete compilation pipeline: lexer → parser → compiler → virtual machine. This is an educational project designed to demonstrate language implementation concepts with clean, well-documented code.

## 🚨 CRITICAL RULES - NEVER FORGET THESE

### 1. Documentation Organization Rules
- **Technical documentation goes in `docs/` folder** - Implementation guides, language references, etc.
- **GitHub/Release automation files stay in root** - CONTRIBUTING.md, CHANGELOG.md (required by tools)
- `docs/` folder contains: DEVELOPMENT_GUIDELINES.md, technical documentation, etc.
- Root contains: README.md, CONTRIBUTING.md, CHANGELOG.md (for GitHub/release-please)
- README.md in root should reference docs/ with proper links
- Only ONE README.md in project root - use INDEX.md for subdirectory documentation
- When creating new docs, ALWAYS put them in docs/ folder first

### 2. Mandatory Development Standards
**CRITICAL**: All development must follow [docs/DEVELOPMENT_GUIDELINES.md](../docs/DEVELOPMENT_GUIDELINES.md)

#### Documentation-First Development (NON-NEGOTIABLE)
- **UPDATE DOCUMENTATION BEFORE OR WITH EVERY CHANGE**
- New features require documentation updates in README.md, docs/LANGUAGE_REFERENCE.md, and examples/
- All API changes must update relevant reference documentation
- Every new builtin function needs help text and usage examples
- All new capabilities must be documented in CHANGELOG.md

#### Quality Gates (ALL MUST PASS BEFORE COMMITS)
```bash
cargo fmt     # Code formatting - MANDATORY
cargo clippy  # Linting - MANDATORY  
cargo build   # Compilation - MANDATORY
cargo test    # All tests - MANDATORY
```

#### Testing Requirements (NON-NEGOTIABLE)
- Write tests for ALL new features and bug fixes
- Add both unit tests and integration tests
- Test edge cases and error conditions
- Maintain comprehensive test coverage
- Update tests when implementation changes

#### Conventional Commits (MANDATORY)
All commits MUST follow conventional commit format:
```
<type>(scope): <description>

Examples:
feat(io): add file I/O builtin functions
fix(parser): handle empty function parameters
docs(readme): update I/O capabilities section
test(vm): add recursive function tests
```

### 3. Architecture Patterns (FOLLOW EXACTLY)

#### Current Implementation Status
- ✅ **79 tests passing** including I/O functions and recursive functions
- ✅ Recursive function support with proper local scope management
- ✅ File I/O functions: read_file(), write_file(), file_exists()
- ✅ Interactive I/O: input(), readline()
- ✅ Modular builtin system with trait-based architecture
- ✅ Pre-commit hooks enforcing quality gates

#### Core Pipeline Pattern
```
Source Code → Lexer → Parser → Compiler → Bytecode → Virtual Machine → Output
```

#### Key Components Architecture

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
   - Manages call frames for function calls and recursion (IMPLEMENTED)
   - Includes REPL functionality for interactive sessions
   - **IMPORTANT**: Local scope management via CallFrame.locals for recursion

5. **Builtin System** (`src/builtins/`)
   - **Pattern**: Trait-based modular system using BuiltinFunction trait
   - Central registry in `src/registry/` for function registration
   - Categories: core, math, string, io
   - **CRITICAL**: All new functions MUST follow this pattern
   - **I/O Functions Available**: read_file, write_file, file_exists, input, readline

### 4. Current Language Features (IMPLEMENTED)

#### Core Language
- Variables: `let x = 42;`
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logic: `&&`, `||`, `!`
- Control Flow: `if/else`, `while`
- Functions: `fn name(params) { body }` 
- **Recursive Functions**: ✅ Fully supported with proper scoping

#### Builtin Functions (CURRENT)
- **Core**: `print(value)`
- **I/O**: `read_file(path)`, `write_file(path, content)`, `file_exists(path)`, `input(prompt)`, `readline()`
- **Math**: `sqrt()`, `abs()`, `max()`, `min()`  
- **String**: `len()`, `upper()`, `lower()`, `reverse()`

### 5. Code Patterns (ALWAYS USE THESE)

#### Rust Conventions
- Use `Result<T, JingError>` aliased as `JingResult<T>` for all fallible operations
- Use `jing::init()` in tests to initialize builtin functions
- Follow Rust naming: `snake_case` for functions/variables, `PascalCase` for types

#### Error Handling Philosophy
- **Fail Fast**: Return errors immediately rather than propagating invalid state
- **Context Preservation**: Include position information (line/column) in errors
- **User-Friendly Messages**: Error messages should be clear and actionable
- **No Panics**: Use `Result` types instead of `panic!` for recoverable errors

#### Testing Patterns
```rust
// Integration test pattern - ALWAYS call jing::init() first
#[test]
fn test_feature() {
    jing::init(); // CRITICAL: Initialize builtins
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    let mut vm = VM::new();
    vm.interpret(chunk)?; // NOT vm.run()
}
```

### 6. Implementation Guidelines

#### When Adding New Language Features
1. **FIRST**: Update docs/LANGUAGE_REFERENCE.md with the feature specification
2. Add tokens to `lexer.rs` with appropriate `Token` variants
3. Add AST nodes to `parser.rs` with corresponding `Stmt`/`Expr` variants
4. Add compilation logic to `compiler.rs` with new `OpCode` instructions
5. Implement execution in `vm.rs` with stack manipulation
6. Write comprehensive tests (unit + integration)
7. Add examples in `examples/` directory
8. Update README.md if it's a major feature

#### When Adding New Builtin Functions
1. **FIRST**: Update docs/LANGUAGE_REFERENCE.md with function specification
2. Implement BuiltinFunction trait in appropriate `src/builtins/*.rs` file
3. Register in `src/builtins/mod.rs::init_builtins()`
4. Write tests in `tests/` directory
5. Add usage examples in `examples/` directory
6. Update help text with clear description

### 7. Development Workflow Enforcement

#### Pre-Commit Checklist (USE AS TEMPLATE)
```bash
# MANDATORY before every commit:
cargo fmt      # Fix formatting
cargo clippy   # Fix linting issues  
cargo build    # Ensure compilation
cargo test     # Verify all tests pass

# Documentation updates:
# - docs/LANGUAGE_REFERENCE.md (for new features)
# - README.md (for major features)
# - CHANGELOG.md (for user-facing changes)
# - examples/ (for new capabilities)

# Commit with conventional format:
git commit -m "feat(scope): description"
```

#### File Organization (MAINTAIN EXACTLY)
```
src/
├── main.rs              # Entry point and REPL
├── lib.rs              # Public API exports and jing::init()
├── lexer.rs            # Tokenization
├── parser.rs           # AST construction
├── compiler.rs         # Bytecode generation  
├── vm.rs               # Virtual machine with CallFrame recursion
├── value.rs            # Value types and operations
├── error.rs            # Error handling
├── features/mod.rs     # BuiltinFunction trait
├── registry/mod.rs     # Function registration system
└── builtins/
    ├── mod.rs          # init_builtins() and registration
    ├── core.rs         # print, type functions
    ├── math.rs         # Math functions
    ├── string.rs       # String functions
    └── io.rs           # I/O functions (NEW)

docs/
├── DEVELOPMENT_GUIDELINES.md # MANDATORY standards
├── DEVELOPMENT.md      # Development setup
├── GETTING_STARTED.md  # Beginner tutorial
├── LANGUAGE_REFERENCE.md # Complete spec with I/O functions
├── HOW_IT_WORKS.md     # Implementation deep dive
├── VISUAL_GUIDE.md     # Diagrams
└── IO_IMPLEMENTATION_SUMMARY.md # I/O details

examples/
├── README.md           # Example documentation
├── hello.jing          # Basic example
├── recursive.jing      # Recursive functions demo
├── file_io.jing        # I/O operations demo
└── [other examples]
```

## 8. NEVER FORGET THESE PATTERNS

### Documentation Creation
- ✅ Create new docs in `docs/` folder
- ✅ Update README.md to reference new docs
- ❌ Never create .md files in project root (except README.md)
- ❌ Never create multiple README.md files - use INDEX.md for subdirectories

### Testing New Features  
- ✅ Call `jing::init()` before running any Jing code in tests
- ✅ Use `vm.interpret(chunk)` not `vm.run()`
- ✅ Test both success and error cases
- ❌ Never skip writing tests

### Builtin Functions
- ✅ Implement BuiltinFunction trait
- ✅ Register in init_builtins()
- ✅ Include help text
- ✅ Write comprehensive tests
- ❌ Never add functions without following the trait pattern

### Quality Standards
- ✅ Update documentation WITH every change
- ✅ Run all quality gates before commits
- ✅ Use conventional commit messages
- ❌ Never commit without passing all quality gates
- ❌ Never add features without updating docs

## Educational Philosophy

This is an educational implementation prioritizing:
- **Clarity over Performance**: Code should be easy to understand
- **Comprehensive Documentation**: Every feature must be well-documented
- **Professional Standards**: Development practices should be exemplary
- **Modular Architecture**: Easy to extend and modify

**When suggesting improvements, ALWAYS maintain educational value and code clarity as primary goals while following these mandatory development standards.**
