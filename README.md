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
let x = 42;
let name = "World";
let flag = true;
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
```jing
print("Hello, World!");           // Output to console
let length = len("Hello");        // String length
let text = str(42);              // Convert to string
```

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

**New to Jing?** Check out the [`GETTING_STARTED.md`](GETTING_STARTED.md) guide for a step-by-step tutorial!

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

The modular design makes it easy to extend Jing:

### Adding New Data Types
1. Extend the `Value` enum in `value.rs`
2. Update comparison and conversion functions
3. Add new bytecode instructions if needed

### Adding New Operators
1. Add tokens to the lexer
2. Update the parser with precedence rules
3. Add compilation logic
4. Implement VM instruction

### Adding New Built-in Functions
1. Add function name to the compiler's built-ins
2. Implement the function in the VM
3. Update documentation

### Adding New Control Structures
1. Add keywords to the lexer
2. Update parser grammar
3. Add compilation logic with jump instructions
4. Test with examples

## 📁 Project Structure

```
src/
├── main.rs          # Entry point and REPL
├── lexer.rs         # Tokenization
├── parser.rs        # AST construction
├── compiler.rs      # Bytecode generation
├── vm.rs           # Virtual machine
├── value.rs        # Value types and operations
├── error.rs        # Error handling
└── lib.rs          # Library exports

examples/
├── hello.jing        # Hello world
├── fibonacci.jing    # Recursive fibonacci
├── fizzbuzz.jing     # FizzBuzz implementation
└── calculator.jing   # Simple calculator

tests/
├── lexer_tests.rs  # Lexer unit tests
├── parser_tests.rs # Parser unit tests
├── vm_tests.rs     # VM integration tests
└── examples_tests.rs # Example program tests
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

## 📚 Learning Resources

- **Language Implementation**: Study `compiler.rs` to see how high-level constructs are translated to bytecode
- **VM Architecture**: Examine `vm.rs` to understand stack-based execution
- **Parser Design**: Look at `parser.rs` for recursive descent parsing techniques
- **Error Handling**: See `error.rs` for comprehensive error reporting
- **Beginner's Guide**: Read [`HOW_IT_WORKS.md`](HOW_IT_WORKS.md) for a detailed explanation of how the language implementation works from scratch
- **Visual Guide**: Check out [`VISUAL_GUIDE.md`](VISUAL_GUIDE.md) for diagrams and visual explanations of the compilation pipeline

## 🤝 Contributing

This is a learning project! Feel free to:
- Add new language features
- Improve error messages
- Optimize the VM
- Add more built-in functions
- Enhance the REPL experience
- Write more example programs

## 📄 License

MIT License - feel free to use this for learning and experimentation!

---

*Happy coding with Jing! 🚀*
