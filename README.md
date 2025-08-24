# Jing - A Simple Toy Language with Virtual Machine

Jing is a simple, extensible toy programming language implemented in Rust with its own virtual machine. It's designed to be easy to understand, modify, and extend for educational purposes and experimentation.

## 🎯 Design Goals

- **Simplicity**: Easy to understand implementation with clear separation of concerns
- **Extensibility**: Modular design that allows easy addition of new features
- **Educational**: Well-documented code that serves as a learning resource
- **Playground**: Perfect for experimenting with language design concepts

## 🏗️ Architecture

The Jing implementation consists of several key components:

```
Source Code → Lexer → Parser → Compiler → Bytecode → Virtual Machine → Output
```

### 1. **Lexer** (`lexer.rs`)
- Converts source code into tokens
- Handles keywords, identifiers, numbers, strings, and operators
- Supports comments and whitespace handling

### 2. **Parser** (`parser.rs`)
- Builds an Abstract Syntax Tree (AST) from tokens
- Implements recursive descent parsing
- Handles operator precedence and associativity

### 3. **Compiler** (`compiler.rs`)
- Converts AST to bytecode instructions
- Performs basic optimizations
- Generates jump targets and handles control flow

### 4. **Virtual Machine** (`vm.rs`)
- Stack-based execution engine
- Executes bytecode instructions
- Manages memory and variable storage

### 5. **Value System** (`value.rs`)
- Dynamic typing with runtime type checking
- Supports numbers, strings, booleans, and nil
- Extensible for adding new data types

## 📝 Language Syntax

Jing supports the following features:

### Variables and Assignment
```jing
let x = 42;          // Variable declaration
let name = "World";  // String variable
let flag = true;     // Boolean variable

x = x + 10;         // Variable assignment
name = "Jing";      // Reassign string
```

### Arithmetic Operations
```jing
let result = (10 + 5) * 2 - 3;
let quotient = 20 / 4;
let remainder = 17 % 5;
```

### Comparison and Logical Operations
```jing
let is_equal = x == 42;
let is_greater = x > 10;
let both_true = flag && is_equal;
let either_true = flag || false;
```

### Control Flow
```jing
if x > 0 {
    print("Positive");
} else {
    print("Non-positive");
}

while x > 0 {
    x = x - 1;
}
```

### Functions
```jing
fn greet(name) {
    return "Hello, " + name + "!";
}

let message = greet("Jing");
```

### Built-in Functions

#### Core Functions
```jing
print("Hello, World!");           // Output to console
```

#### I/O Functions
```jing
// File operations
write_file("hello.txt", "Hello, World!");
let content = read_file("hello.txt");
let exists = file_exists("hello.txt");

// Interactive input
let name = input("What's your name? ");
let line = readline();            // Read line without prompt
```

#### HTTP Server Functions
```jing
// Start multiple servers on different ports (returns server handles)
let api_server = start_http_server(8080);
let admin_server = start_http_server(9000);

// Register server-specific route handlers using handles
http_register_handler(api_server, "GET", "/api/users", "get_users_handler");
http_register_handler(api_server, "POST", "/api/users", "create_user_handler");
http_register_handler(api_server, "DELETE", "/api/users/123", "delete_user_handler");

// Different handlers for admin server
http_register_handler(admin_server, "GET", "/admin/stats", "get_admin_stats");
http_register_handler(admin_server, "POST", "/admin/reset", "reset_system");

// List all running servers
let servers = list_http_servers();
print(servers);

// Create HTTP responses (for use in handlers)
let response = http_response(200, "application/json", "{\"status\": \"ok\"}");

// Stop servers when done using handles
let stopped_api = stop_http_server(api_server);
let stopped_admin = stop_http_server(admin_server);
```

**Built-in Routes (available on all servers):**
- `GET /` - Welcome message with server info
- `GET /status` - JSON server status with timestamp
- `POST /echo` - Echo service returning request body

**Custom Handler Registration:**
- Register Jing functions to handle specific HTTP routes using server handles
- Each server maintains its own independent set of route handlers
- Supports GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS methods
- Must use server handle returned from `start_http_server()` to associate handlers with specific servers
- Custom handlers currently return placeholder responses (handler execution coming in future update)

## 🔧 Bytecode Instructions

The VM uses a simple instruction set:

| Instruction | Description |
|-------------|-------------|
| `CONSTANT`  | Push constant to stack |
| `LOAD`      | Load variable to stack |
| `STORE`     | Store top of stack to variable |
| `ADD`       | Binary addition |
| `SUB`       | Binary subtraction |
| `MUL`       | Binary multiplication |
| `DIV`       | Binary division |
| `MOD`       | Binary modulo |
| `EQ`        | Equality comparison |
| `NE`        | Not equal comparison |
| `LT`        | Less than comparison |
| `LE`        | Less than or equal |
| `GT`        | Greater than comparison |
| `GE`        | Greater than or equal |
| `AND`       | Logical AND |
| `OR`        | Logical OR |
| `NOT`       | Logical NOT |
| `NEG`       | Arithmetic negation |
| `JUMP`      | Unconditional jump |
| `JUMP_IF_FALSE` | Conditional jump |
| `CALL`      | Function call |
| `RETURN`    | Return from function |
| `PRINT`     | Print top of stack |
| `HALT`      | Stop execution |

## 🚀 Getting Started

### Building and Running

```bash
# Build the project
cargo build

# Run with a source file
cargo run examples/hello.jing

# Run the REPL
cargo run
```

**New to Jing?** Check out the [`docs/GETTING_STARTED.md`](docs/GETTING_STARTED.md) guide for a step-by-step tutorial!

**Complete Documentation:** Browse the [`docs/`](docs/) folder for comprehensive guides and technical references.

### Example Programs

**Hello World** (`examples/hello.jing`):
```jing
print("Hello, World!");
```

**Fibonacci** (`examples/fibonacci.jing`):
```jing
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

let result = fibonacci(10);
print("Fibonacci(10) = " + str(result));
```

**FizzBuzz** (`examples/fizzbuzz.jing`):
```jing
let i = 1;
while i <= 100 {
    if i % 15 == 0 {
        print("FizzBuzz");
    } else if i % 3 == 0 {
        print("Fizz");
    } else if i % 5 == 0 {
        print("Buzz");
    } else {
        print(str(i));
    }
    i = i + 1;
}
```

## 🔧 Extending Jing

Jing features a **modular, plugin-like architecture** that makes extending the language incredibly easy:

### Adding New Built-in Functions

The new modular system allows adding functions without touching core files:

1. **Implement the BuiltinFunction trait**:

```rust
use crate::features::BuiltinFunction;
use crate::value::Value;
use crate::error::{JingError, JingResult};

#[derive(Debug)]
pub struct MyFunction;

impl BuiltinFunction for MyFunction {
    fn name(&self) -> &str { "my_function" }
    fn arity(&self) -> usize { 1 }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        // Your implementation here
        Ok(Value::Number(42.0))
    }
    
    fn help(&self) -> &str {
        "my_function(arg) - Description of what it does"
    }
}
```

2. **Register it in the appropriate builtin module** (e.g., `src/builtins/math.rs`, `src/builtins/string.rs`):

```rust
// In src/builtins/mod.rs
register_builtin(Arc::new(math::MyFunction));
```

3. **Done!** Your function is now available in the language.

### Adding New Data Types

1. Extend the `Value` enum in `value.rs`
2. Update comparison and conversion functions
3. Add new bytecode instructions if needed

### Current Built-in Function Categories

- **Core**: `print()`, `type()`
- **Math**: `sqrt()`, `abs()`, `max()`, `min()`
- **String**: `len()`, `upper()`, `lower()`, `reverse()`
- **I/O**: `readline()`, `input()`, `read_file()`, `write_file()`, `file_exists()`
- **HTTP Server**: `start_http_server()`, `stop_http_server()`, `http_response()`, `list_http_servers()`, `http_register_handler()`

*The modular design makes adding new categories (like JSON, networking, databases) straightforward!*

## 📁 Project Structure

```text
src/
├── main.rs          # Entry point and REPL
├── lib.rs           # Public API and initialization
├── lexer.rs         # Tokenization
├── parser.rs        # AST construction
├── compiler.rs      # Bytecode generation
├── vm.rs           # Virtual machine
├── value.rs        # Value types and operations
├── error.rs        # Error handling
├── features/        # Trait definitions for extensibility
│   └── mod.rs      # BuiltinFunction trait
├── registry/        # Global function registry
│   └── mod.rs      # Thread-safe registration system
└── builtins/        # Built-in function implementations
    ├── mod.rs      # Initialization and registration
    ├── core.rs     # Core functions (print, type)
    ├── math.rs     # Math functions (sqrt, abs, etc.)
    ├── string.rs   # String functions (len, upper, etc.)
    └── io.rs       # I/O functions (readline, input)

docs/
├── README.md            # Documentation index
├── GETTING_STARTED.md   # Beginner's tutorial
├── LANGUAGE_REFERENCE.md # Complete language spec
├── HOW_IT_WORKS.md      # Implementation deep dive  
├── VISUAL_GUIDE.md      # Diagrams and visual guides
├── CONTRIBUTING.md      # How to contribute
└── DEVELOPMENT.md       # Development setup guide

examples/
├── hello.jing        # Hello world
├── fibonacci.jing    # Recursive fibonacci
├── fizzbuzz.jing     # FizzBuzz implementation
└── calculator.jing   # Simple calculator

tests/
├── lexer_tests.rs       # Lexer unit tests
├── parser_tests.rs      # Parser unit tests
├── compiler_vm_tests.rs # VM integration tests
├── value_tests.rs       # Value system tests
├── error_tests.rs       # Error handling tests
└── integration_tests.rs # End-to-end tests
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test lexer_tests

# Run tests with output
cargo test -- --nocapture
```

## 📚 Documentation

This project includes comprehensive documentation in the [`docs/`](docs/) folder:

- **[Getting Started Guide](docs/GETTING_STARTED.md)** - Your first steps with Jing
- **[Language Reference](docs/LANGUAGE_REFERENCE.md)** - Complete syntax and built-ins reference  
- **[How It Works](docs/HOW_IT_WORKS.md)** - Deep dive into the implementation
- **[Visual Guide](docs/VISUAL_GUIDE.md)** - Diagrams and visual explanations
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute to the project
- **[Development Setup](docs/DEVELOPMENT.md)** - Development environment and tools

## 📚 Learning Resources

- **Language Implementation**: Study `compiler.rs` to see how high-level constructs are translated to bytecode
- **VM Architecture**: Examine `vm.rs` to understand stack-based execution
- **Parser Design**: Look at `parser.rs` for recursive descent parsing techniques
- **Error Handling**: See `error.rs` for comprehensive error reporting
- **Beginner's Guide**: Read [`HOW_IT_WORKS.md`](docs/HOW_IT_WORKS.md) for a detailed explanation of how the language implementation works from scratch
- **Visual Guide**: Check out [`VISUAL_GUIDE.md`](docs/VISUAL_GUIDE.md) for diagrams and visual explanations of the compilation pipeline

## 🚧 Known Limitations & TODO

### Current Limitations

- **Standard Library**: Currently limited to basic built-in functions (print, type, math operations, string operations)

### Planned Features

- [ ] **Advanced Data Types**: Arrays, objects/structs, and other complex data structures  
- [ ] **Enhanced Standard Library**: More built-in functions for common operations
- [ ] **Import/Module System**: Support for organizing code across multiple files
- [ ] **Error Handling**: Try-catch mechanisms for robust error handling
- [ ] **Performance Optimizations**: Bytecode optimizations and JIT compilation
- [ ] **Enhanced REPL**: Auto-completion, syntax highlighting, and better error reporting

### Development TODOs

- [ ] Add benchmarking suite for performance testing
- [ ] Improve error messages with better source location tracking  
- [ ] Add more comprehensive integration tests for edge cases
- [ ] Implement arrays and object data structures

## 📚 Complete Documentation

### Complete Documentation
- **[docs/](docs/)** - Complete documentation collection
- **[docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)** - Step-by-step tutorial for beginners
- **[docs/LANGUAGE_REFERENCE.md](docs/LANGUAGE_REFERENCE.md)** - Complete language specification and builtin functions
- **[docs/HOW_IT_WORKS.md](docs/HOW_IT_WORKS.md)** - Technical deep dive into implementation
- **[docs/VISUAL_GUIDE.md](docs/VISUAL_GUIDE.md)** - Visual diagrams and flowcharts

### Development & Contributing
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute (REQUIRED READING)
- **[docs/DEVELOPMENT_GUIDELINES.md](docs/DEVELOPMENT_GUIDELINES.md)** - **MANDATORY** development standards
- **[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)** - Development environment setup
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and changes

## 🤝 Contributing

**Want to contribute?** 

1. **MUST READ**: [docs/DEVELOPMENT_GUIDELINES.md](docs/DEVELOPMENT_GUIDELINES.md) - Our mandatory development standards
2. **Follow**: [CONTRIBUTING.md](CONTRIBUTING.md) - Step-by-step contribution guide
3. **Setup**: [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) - Development environment

This is a learning project with professional development standards! We welcome contributions that:

- Add new language features
- Improve error messages
- Optimize the VM
- Add more built-in functions
- Enhance the REPL experience
- Write more example programs

## 📄 License

MIT License - feel free to use this for learning and experimentation!

---

Happy coding with Jing! 🚀
