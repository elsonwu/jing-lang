# Jing Language Examples

This directory contains example programs demonstrating various features of the Jing language.

## Basic Examples

### [`hello.jing`](hello.jing)
The classic "Hello, World!" program.

### [`calculator.jing`](calculator.jing)
Simple arithmetic operations and variable usage.

### [`demo.jing`](demo.jing)
Comprehensive demonstration of core language features.

## Advanced Features

### [`recursive.jing`](recursive.jing)
Demonstrates recursive function calls with factorial and fibonacci examples.

### [`file_io.jing`](file_io.jing)

Complete file I/O operations demonstration including:

- Interactive user input with `input()` and `readline()`
- File operations: `read_file()`, `write_file()`, `file_exists()`
- Error handling for file operations
- Real-world usage patterns

### [`http_server.jing`](http_server.jing)

Basic HTTP server demonstration including:

- Starting HTTP servers with `start_http_server()`
- Creating HTTP responses with `http_response()`
- Built-in endpoints: `/`, `/status`, `/echo`

### [`http_server_with_handlers.jing`](http_server_with_handlers.jing)

Advanced HTTP server with custom route handlers:

- Multiple server management using server handles
- Custom route registration with `http_register_handler()`
- Server-specific handler configuration
- Handler function definition patterns

### [`server_lifecycle_demo.jing`](server_lifecycle_demo.jing)

Complete server lifecycle management:

- Starting multiple services with different purposes
- Server handle management and configuration
- Stopping servers using handles
- Service monitoring with `list_http_servers()`

### [`fizzbuzz.jing`](fizzbuzz.jing)

Classic FizzBuzz implementation showing loops and conditionals.

### [`fibonacci.jing`](fibonacci.jing)

Fibonacci sequence calculation.

## Function Examples

### [`simple_function.jing`](simple_function.jing)
Basic function definition and calling.

### [`debug_function.jing`](debug_function.jing)
Function debugging and development examples.

### [`advanced.jing`](advanced.jing)
Advanced function usage patterns.

## Running Examples

To run any example, use:

```bash
cargo run examples/filename.jing
```

For example:
```bash
cargo run examples/file_io.jing
cargo run examples/recursive.jing
cargo run examples/hello.jing
```

## Interactive Mode

You can also run Jing in interactive mode to experiment with the language:

```bash
cargo run
```

This starts a REPL (Read-Eval-Print Loop) where you can type Jing expressions and see immediate results.

## Example Output

### File I/O Example
The `file_io.jing` example creates interactive sessions like:

```
What's your name? Alice
What's your age? 25
Nice to meet you, Alice!

=== File Operations ===
✅ Profile saved to user_profile.txt
✅ File exists check passed

📖 Profile content:
Name: Alice
Age: 25
Generated on: 2025-08-24
```

### Recursive Functions Example
The `recursive.jing` example demonstrates:

```
=== Recursive Functions Demo ===
factorial(5) = 120
fibonacci(8) = 21
✅ All recursive function tests passed
```

## Contributing Examples

When adding new language features, please:

1. Create example programs demonstrating the feature
2. Update this README to list the new examples
3. Ensure all examples run successfully
4. Include both simple and advanced usage patterns

Examples serve as both documentation and integration tests for the language features.
