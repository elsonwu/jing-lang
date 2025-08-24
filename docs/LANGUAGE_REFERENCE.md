# Jing Language Reference

## Variables

Variables are declared using the `let` keyword:

```jing
let x = 42;
let name = "Hello";
let flag = true;
```

### Variable Assignment

After declaration, variables can be reassigned using the assignment operator:

```jing
let x = 10;
x = 20;        // Reassign x to 20
x = x + 5;     // x is now 25
```

Assignment is an expression that returns the assigned value:

```jing
let a = 5;
let b = (a = 10); // b is 10, a is 10
```

## Data Types

Jing supports the following data types:

- **Numbers**: `42`, `3.14`, `-7.5`
- **Strings**: `"Hello, World!"`, `""`
- **Booleans**: `true`, `false`
- **Nil**: `nil` (represents no value)

## Operators

### Arithmetic Operators
- `+` Addition (also string concatenation)
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo
- `-x` Negation (unary minus)

### Comparison Operators
- `==` Equal
- `!=` Not equal
- `<` Less than
- `<=` Less than or equal
- `>` Greater than
- `>=` Greater than or equal

### Logical Operators
- `&&` Logical AND
- `||` Logical OR
- `!` or `not` Logical NOT

## Control Flow

### If Statements

```jing
if condition {
    // statements
} else {
    // statements
}
```

### While Loops

```jing
while condition {
    // statements
}
```

## Functions

Function declarations:

```jing
fn function_name(param1, param2) {
    // statements
    return value; // optional
}
```

Function calls:

```jing
let result = function_name(arg1, arg2);
```

### Recursive Functions

Recursive function calls are fully supported:

```jing
fn factorial(n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

print(factorial(5)); // Output: 120
```

## Built-in Functions

### Core Functions

#### `print(value)`
Print a value to the console.

```jing
print("Hello, World!");
print(42);
print(true);
```

### I/O Functions

#### `read_file(path)`
Read entire file contents as a string.

```jing
let content = read_file("data.txt");
print(content);
```

#### `write_file(path, content)`
Write string content to a file.

```jing
write_file("output.txt", "Hello from Jing!");
```

#### `file_exists(path)`
Check if a file or directory exists.

```jing
if (file_exists("config.txt")) {
    let config = read_file("config.txt");
    print("Config loaded");
} else {
    print("Config file not found");
}
```

#### `input(prompt)`
Display a prompt and read user input.

```jing
let name = input("What's your name? ");
print("Hello, " + name + "!");
```

#### `readline()`
Read a line from standard input without a prompt.

```jing
print("Enter text:");
let text = readline();
print("You entered: " + text);
```

### HTTP Server Functions

#### `start_http_server(port)`
Start an HTTP server on the specified port (8000-9999). Returns a server handle string.

```jing
let server_handle = start_http_server(8080);
print(server_handle); // "server_8080"
```

The server automatically provides these endpoints:
- `GET /` - Welcome page with endpoint information
- `GET /status` - JSON status information
- `POST /echo` - Echo back request body as JSON

#### `stop_http_server(server_handle)`
Stop the HTTP server using its server handle.

```jing
let server = start_http_server(8080);
let stop_result = stop_http_server(server);
print(stop_result); // "HTTP server server_8080 stopped"
```

#### `list_http_servers()`
List all currently running HTTP servers.

```jing
let servers = list_http_servers();
print(servers);
// Output: "Running HTTP servers:\n  Port 8080: running\n  Port 8081: running"
```

#### `http_response(status, content_type, body)`
Create an HTTP response with the specified status code, content type, and body.

```jing
// JSON response
let json_resp = http_response(200, "application/json", '{"message": "Hello!"}');

// HTML response  
let html_resp = http_response(200, "text/html", "<h1>Welcome</h1>");

// Error response
let error_resp = http_response(404, "text/plain", "Page not found");
```

Note: Currently this returns a formatted response string. Future versions may integrate with custom request handlers.

#### `http_register_handler(server_handle, method, path, handler_function_name)`
Register a custom Jing function to handle HTTP routes on a specific server.

```jing
let server = start_http_server(8080);

// Register handlers using server handle
http_register_handler(server, "GET", "/users", "get_users");
http_register_handler(server, "POST", "/users", "create_user");
http_register_handler(server, "DELETE", "/users/123", "delete_user");
```

- `server_handle`: Server handle returned by `start_http_server()`
- `method`: HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- `path`: URL path to handle (e.g., "/users", "/api/data")
- `handler_function_name`: Name of Jing function to call (as string)

Note: Handler function calling is not yet implemented. Currently returns placeholder responses.

## Running Jing Programs
```jing
let x = 15;
if x > 10 {
    print("x is greater than 10");
} else {
    print("x is 10 or less");
}
```

### Functions
```jing
fn square(n) {
    return n * n;
}

let result = square(5);
print(result); // Outputs: 25
```

### Loops
```jing
let i = 1;
while i <= 5 {
    print(i);
    i = i + 1; // Assignment expression to update variable
}
```

## Running Jing

### From File
```bash
cargo run script.jing
```

### Interactive REPL
```bash
cargo run
```

## Error Handling

Jing provides clear error messages for:

- Lexical errors (invalid characters)
- Parse errors (syntax errors)
- Runtime errors (type mismatches, undefined variables)
- Compile errors (invalid operations)

Example error messages:
- `Lexical error at line 5: Unexpected character: '@'`
- `Parse error at line 3: Expected ';' after expression`
- `Runtime error: Undefined variable 'foo'`
- `Type error: Cannot add number and string`
