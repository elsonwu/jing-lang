# JiLang Language Reference

## Variables

Variables are declared using the `let` keyword:

```jilang
let x = 42;
let name = "Hello";
let flag = true;
```

## Data Types

JiLang supports the following data types:

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

```jilang
if condition {
    // statements
} else {
    // statements
}
```

### While Loops

```jilang
while condition {
    // statements
}
```

## Functions

Function declarations:

```jilang
fn function_name(param1, param2) {
    // statements
    return value; // optional
}
```

Function calls:

```jilang
let result = function_name(arg1, arg2);
```

## Built-in Functions

- `print(value)` - Print a value to the console

## Comments

Single-line comments start with `//`:

```jilang
// This is a comment
let x = 42; // This is also a comment
```

## Examples

### Hello World
```jilang
print("Hello, World!");
```

### Variables and Arithmetic
```jilang
let a = 10;
let b = 20;
let sum = a + b;
print(sum); // Outputs: 30
```

### String Operations
```jilang
let greeting = "Hello, " + "World!";
print(greeting); // Outputs: Hello, World!
```

### Control Flow
```jilang
let x = 15;
if x > 10 {
    print("x is greater than 10");
} else {
    print("x is 10 or less");
}
```

### Functions
```jilang
fn square(n) {
    return n * n;
}

let result = square(5);
print(result); // Outputs: 25
```

### Loops
```jilang
let i = 1;
while i <= 5 {
    print(i);
    let i = i + 1; // Re-declaration to update variable
}
```

## Running JiLang

### From File
```bash
cargo run script.jl
```

### Interactive REPL
```bash
cargo run
```

## Error Handling

JiLang provides clear error messages for:

- Lexical errors (invalid characters)
- Parse errors (syntax errors)
- Runtime errors (type mismatches, undefined variables)
- Compile errors (invalid operations)

Example error messages:
- `Lexical error at line 5: Unexpected character: '@'`
- `Parse error at line 3: Expected ';' after expression`
- `Runtime error: Undefined variable 'foo'`
- `Type error: Cannot add number and string`
