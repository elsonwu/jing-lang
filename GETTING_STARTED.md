# Getting Started with Jing

Welcome to Jing! This guide will help you get up and running quickly.

## Quick Setup

1. **Clone or have the Jing project**
2. **Build the project:**
   ```bash
   cargo build --release
   ```
3. **You're ready to go!**

## Your First Jing Program

Create a file called `hello.jing`:

```jing
print("Hello, Jing!");
```

Run it:
```bash
cargo run hello.jing
```

Output:
```
Hello, Jing!
```

## Interactive Mode (REPL)

Start the interactive interpreter:
```bash
cargo run
```

Try these commands:
```
> let x = 42;
> let y = 8;
> let result = x + y;
> print(result);
50
> let greeting = "Hello, " + "World!";
> print(greeting);
Hello, World!
> exit
```

## Basic Concepts

### Variables
```jing
let number = 42;
let text = "Hello";
let flag = true;
let nothing = nil;
```

### Math
```jing
let a = 10;
let b = 3;
print(a + b);  // 13
print(a - b);  // 7
print(a * b);  // 30
print(a / b);  // 3.333...
print(a % b);  // 1 (remainder)
```

### Strings
```jing
let first = "Hello";
let second = "World";
let combined = first + ", " + second + "!";
print(combined);  // Hello, World!
```

### Comparisons
```jing
let x = 10;
print(x > 5);    // true
print(x == 10);  // true
print(x != 20);  // true
```

### If Statements
```jing
let age = 25;
if age >= 18 {
    print("You are an adult");
} else {
    print("You are a minor");
}
```

### While Loops
```jing
let count = 1;
while count <= 5 {
    print(count);
    count = count + 1;  // Assignment expression to update
}
```

### Functions (Basic)
```jing
fn greet() {
    print("Hello from a function!");
}

greet();  // Call the function
```

### Functions with Return Values
```jing
fn square() {
    return 5 * 5;
}

let result = square();
print(result);  // 25
```

## Example Programs

### FizzBuzz
```jing
let i = 1;
while i <= 15 {
    if i % 15 == 0 {
        print("FizzBuzz");
    } else if i % 3 == 0 {
        print("Fizz");
    } else if i % 5 == 0 {
        print("Buzz");
    } else {
        print(i);
    }
    i = i + 1;
}
```

### Simple Calculator
```jing
let a = 20;
let b = 4;

print("Addition: ");
print(a + b);

print("Subtraction: ");
print(a - b);

print("Multiplication: ");
print(a * b);

print("Division: ");
print(a / b);
```

## Common Mistakes

### 1. Forgetting Semicolons
```jing
// Wrong:
let x = 42
print(x)

// Right:
let x = 42;
print(x);
```

### 2. Variable Updates
Jing currently doesn't have assignment (only declaration), so to "update" a variable, you need to redeclare it:

```jing
// Variable assignment now works!
let x = 1;
x = x + 1;  // Assignment expression - x is now 2

// Assignment is an expression that returns the assigned value
let y = (x = 10);  // x is 10, y is 10
```

### 3. Function Parameters
Function parameters now work correctly:

```jing
// Functions with parameters work:
fn add(a, b) {
    return a + b;
}

let result = add(5, 3);
print(result);  // Outputs: 8

// Functions without parameters also work:
fn sayHello() {
    print("Hello!");
}

sayHello();  // Outputs: Hello!
```

## Tips for Learning

1. **Start simple**: Begin with variables and basic operations
2. **Use the REPL**: Test small code snippets interactively
3. **Check examples**: Look at the `examples/` folder for inspiration
4. **Read error messages**: Jing provides helpful error messages
5. **Experiment**: Try different combinations and see what happens!

## What's Next?

Once you're comfortable with the basics:

1. **Read the full language reference**: [`LANGUAGE_REFERENCE.md`](LANGUAGE_REFERENCE.md)
2. **Understand how it works**: [`HOW_IT_WORKS.md`](HOW_IT_WORKS.md)
3. **See the visual guide**: [`VISUAL_GUIDE.md`](VISUAL_GUIDE.md)
4. **Try extending the language**: Add new features!

## Getting Help

If something doesn't work as expected:

1. Check the error message - they're designed to be helpful
2. Try the same code in the REPL to isolate the issue
3. Look at the example programs for reference
4. Remember that Jing is a learning project - some features are still evolving!

Happy coding! 🎉
