// This is a comprehensive test of JiLang features
print("=== JiLang Demo ===");

// Variables and basic operations
let x = 42;
let y = 13;
let name = "JiLang";

print("Variables:");
print(x);
print(y);
print(name);

// Arithmetic operations
let sum = x + y;
let difference = x - y;
let product = x * y;
let quotient = x / y;
let remainder = x % y;

print("Arithmetic:");
print(sum);
print(difference);
print(product);
print(quotient);
print(remainder);

// String operations
let greeting = "Hello, " + name + "!";
print("String concatenation:");
print(greeting);

// Boolean operations
let is_positive = x > 0;
let is_equal = x == 42;
let both_true = is_positive && is_equal;

print("Boolean operations:");
print(is_positive);
print(is_equal);
print(both_true);

// Control flow - if statements
if x > y {
    print("x is greater than y");
} else {
    print("y is greater than or equal to x");
}

// Control flow - while loops
let counter = 1;
while counter <= 3 {
    print("Counter: ");
    print(counter);
    let counter = counter + 1;
}

print("=== Demo Complete ===");
