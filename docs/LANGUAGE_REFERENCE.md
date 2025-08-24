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
