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

> **⚠️ Current Limitation**: Recursive function calls are not yet fully supported. While the syntax is accepted, the VM's call stack management needs enhancement for proper recursion. This is planned for a future release.

## Built-in Functions

- `print(value)` - Print a value to the console

## Comments

Single-line comments start with `//`:

```jing
// This is a comment
let x = 42; // This is also a comment
```

## Examples

### Hello World
```jing
print("Hello, World!");
```

### Variables and Arithmetic
```jing
let a = 10;
let b = 20;
let sum = a + b;
print(sum); // Outputs: 30
```

### String Operations
```jing
let greeting = "Hello, " + "World!";
print(greeting); // Outputs: Hello, World!
```

### Control Flow
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
