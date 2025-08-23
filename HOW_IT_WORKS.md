# How Jing Works: A Beginner's Guide to Language Implementation

This guide explains how Jing works internally, step by step, for people who have never implemented a programming language before.

## The Big Picture: From Text to Execution

When you write code in any programming language and run it, the computer doesn't understand your text directly. It needs to be transformed through several stages:

```
Your Code Text → [Magic Happens] → Computer Executes It
```

Jing implements this "magic" through a pipeline of well-defined steps:

```
Source Code → Lexer → Parser → Compiler → Virtual Machine → Output
   "let x = 42;"    Tokens     AST      Bytecode    Execution    "42"
```

Let's walk through each step with a simple example: `let x = 42;`

## Step 1: Lexical Analysis (Lexer) - Breaking Text into Tokens

**What it does**: The lexer takes your source code text and breaks it into "tokens" - meaningful chunks that the computer can understand.

**Think of it like**: Reading a sentence and identifying each word and punctuation mark.

### Example:
```rust
Input:  "let x = 42;"
Output: [LET, IDENTIFIER("x"), EQUAL, NUMBER(42), SEMICOLON]
```

### How it works:
The lexer reads your code character by character:

1. **Sees 'l'**: Starts reading letters
2. **Reads "let"**: Recognizes this as a keyword → creates `LET` token
3. **Sees space**: Skips whitespace
4. **Sees 'x'**: Recognizes as identifier → creates `IDENTIFIER("x")` token
5. **Sees '='**: Recognizes as operator → creates `EQUAL` token
6. **Sees "42"**: Recognizes as number → creates `NUMBER(42)` token
7. **Sees ';'**: Recognizes as punctuation → creates `SEMICOLON` token

### Key Code Location: `src/lexer.rs`
```rust
pub enum TokenType {
    // Keywords
    Let, If, While, Fn,
    // Literals  
    Number(f64), String(String), Identifier(String),
    // Operators
    Plus, Minus, Equal, EqualEqual,
    // ... and more
}
```

## Step 2: Syntax Analysis (Parser) - Building the Structure

**What it does**: The parser takes the tokens and builds a tree structure (AST - Abstract Syntax Tree) that represents the meaning and structure of your code.

**Think of it like**: Understanding that "The cat sat on the mat" has a subject (cat), verb (sat), and prepositional phrase (on the mat).

### Example:
```rust
Tokens: [LET, IDENTIFIER("x"), EQUAL, NUMBER(42), SEMICOLON]
AST:    LetStmt {
          name: "x",
          initializer: Literal(Number(42))
        }
```

### How it works:
The parser follows grammar rules to understand code structure:

1. **Sees LET token**: "This must be a variable declaration"
2. **Expects identifier**: "What's the variable name?" → gets "x"
3. **Expects EQUAL**: "There should be an assignment" → finds it
4. **Parses expression**: "What value?" → finds number 42
5. **Expects SEMICOLON**: "Statement should end" → finds it
6. **Creates AST node**: Builds a `LetStmt` structure

### Key Code Location: `src/parser.rs`
```rust
pub enum Stmt {
    Let(LetStmt),      // let x = 42;
    If(IfStmt),        // if condition { ... }
    While(WhileStmt),  // while condition { ... }
    // ... and more
}

pub enum Expr {
    Literal(LiteralExpr),  // 42, "hello", true
    Variable(VariableExpr), // x, myVar  
    Binary(BinaryExpr),    // x + y, a == b
    Assign(AssignExpr),    // x = 10, y = x + 1
    // ... and more
}
```

## Step 3: Code Generation (Compiler) - Creating Instructions

**What it does**: The compiler takes the AST and converts it into bytecode - simple instructions that a virtual machine can execute.

**Think of it like**: Taking a recipe (AST) and breaking it down into step-by-step cooking instructions (bytecode).

### Example:
```rust
AST:      LetStmt { name: "x", initializer: Literal(Number(42)) }
Bytecode: [CONSTANT(0), STORE("x")]  // where constants[0] = 42
```

### How it works:
The compiler walks through the AST and generates bytecode:

1. **Sees LetStmt**: "I need to store a value in a variable"
2. **Sees Literal(42)**: "First, put 42 on the stack" → `CONSTANT(0)`
3. **Sees name "x"**: "Now store it in variable x" → `STORE("x")`

### Key Code Location: `src/compiler.rs`
```rust
pub enum OpCode {
    Constant(usize),    // Push constant to stack
    Load(String),       // Load variable to stack  
    Store(String),      // Store stack top to variable
    Add, Subtract,      // Arithmetic operations
    Jump(usize),        // Control flow
    Call(usize),        // Function calls
    // ... and more
}
```

## Step 4: Execution (Virtual Machine) - Running the Code

**What it does**: The VM executes the bytecode instructions one by one, using a stack to keep track of values.

**Think of it like**: A very simple computer that only understands these specific instructions and has a stack of papers to write numbers on.

### Example:
```rust
Bytecode: [CONSTANT(0), STORE("x")]
Stack:    [] → [42] → []  (42 gets stored in variable "x")
Variables: {} → {"x": 42}
```

### How it works:
The VM has a simple execution loop:

1. **Read instruction**: `CONSTANT(0)`
2. **Execute**: Look up constants[0] (which is 42), push onto stack
3. **Stack now**: [42]
4. **Read instruction**: `STORE("x")`  
5. **Execute**: Pop 42 from stack, store in variable "x"
6. **Stack now**: []
7. **Variables now**: {"x": 42}

### Key Code Location: `src/vm.rs`
```rust
pub struct VM {
    chunk: Chunk,           // The bytecode to execute
    ip: usize,             // Instruction pointer (which instruction we're on)
    stack: Vec<Value>,     // Stack of values
    globals: Environment,  // Variable storage
}
```

## The Stack: Why It's Important

The **stack** is like a stack of plates - you can only add to the top (push) or remove from the top (pop). It's crucial for:

1. **Storing temporary values** during calculations
2. **Passing arguments** to functions
3. **Keeping track** of where we are in nested operations

### Example: `(10 + 5) * 2`
```
Step 1: CONSTANT(10)     Stack: [10]
Step 2: CONSTANT(5)      Stack: [10, 5]  
Step 3: ADD              Stack: [15]        (pops 10 and 5, pushes 15)
Step 4: CONSTANT(2)      Stack: [15, 2]
Step 5: MULTIPLY         Stack: [30]        (pops 15 and 2, pushes 30)
```

## Value System: How Jing Handles Different Types

Jing uses dynamic typing - variables can hold different types of values:

```rust
pub enum Value {
    Nil,                    // nil
    Bool(bool),            // true, false  
    Number(f64),           // 42, 3.14
    String(String),        // "hello"
    Function { ... },      // user-defined functions
}
```

### Type Operations:
- **Numbers**: Can do math (`+`, `-`, `*`, `/`)
- **Strings**: Can concatenate with `+` 
- **Booleans**: Used in `if` statements and logic
- **Mixed**: `"Score: " + 42` becomes `"Score: 42"`

## Error Handling: When Things Go Wrong

Jing provides helpful errors at each stage:

### Lexer Errors:
```
Input: "let x = 42@"
Error: "Lexical error at line 1: Unexpected character: '@'"
```

### Parser Errors:  
```
Input: "let = 42;"
Error: "Parse error at line 1: Expected variable name"
```

### Runtime Errors:
```
Input: "print(unknownVar);"
Error: "Runtime error: Undefined variable 'unknownVar'"
```

### Type Errors:
```
Input: "let x = 42 / false;"  
Error: "Type error: Cannot divide number and bool"
```

## Memory Management: How Variables Are Stored

Jing uses an **Environment** system for variables:

```rust
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,  // Stack of variable scopes
}
```

### How Scopes Work:
1. **Global scope**: Variables accessible everywhere
2. **Function scope**: Variables only in that function
3. **Block scope**: Variables only in `{ }` blocks

### Example:
```jing
let global = "I'm global";

if true {
    let local = "I'm local";  // Only exists in this block
    print(global);            // Can access global
}
// local is gone here, but global still exists
```

## Control Flow: How If/While Work

### If Statements:
```jing
if x > 10 {
    print("big");
} else {
    print("small");  
}
```

**Compiles to:**
```
1. LOAD("x")           // Put x on stack
2. CONSTANT(10)        // Put 10 on stack  
3. GREATER            // Compare, put result on stack
4. JUMP_IF_FALSE(8)   // If false, jump to instruction 8
5. CONSTANT("big")     // Push "big"
6. PRINT              // Print it
7. JUMP(10)           // Jump past else
8. CONSTANT("small")   // Push "small"  
9. PRINT              // Print it
10. (continue...)
```

### While Loops:
```jing
while x > 0 {
    x = x - 1;
}
```

**Creates a loop** by jumping back to the condition check.

## Function Calls: How They Work

Functions are more complex because they need to:
1. **Save where we were** (return address)
2. **Set up new variable scope** for parameters  
3. **Execute function body**
4. **Restore previous state** when done

This is why we have a **call stack** - it remembers where to return to.

## Why This Architecture?

This design is **educational** and **extensible**:

### Benefits:
1. **Separation of concerns**: Each part has one job
2. **Easy to debug**: You can see exactly what happens at each step
3. **Easy to extend**: Want to add a new operator? Just update lexer, parser, compiler, and VM
4. **Testable**: Each component can be tested independently

### Trade-offs:
- **Performance**: Multiple passes are slower than direct compilation
- **Memory**: AST and bytecode take extra memory
- **Complexity**: More moving parts than a simple interpreter

## How to Extend Jing

Want to add a new feature? Here's the process:

### Example: Adding a `for` loop

1. **Lexer**: Add `For` token type
2. **Parser**: Add `ForStmt` AST node and parsing logic
3. **Compiler**: Generate bytecode for loop (similar to while)
4. **VM**: No changes needed (uses existing jump instructions)
5. **Tests**: Add test cases

### Example: Adding a `%` operator (already done!)

1. **Lexer**: Add `Percent` token when seeing '%'
2. **Parser**: Add to operator precedence rules  
3. **Compiler**: Generate `MODULO` instruction
4. **VM**: Implement modulo operation in `Value::modulo()`

## Debugging Tips

### See What's Happening:
1. **Print tokens** after lexing
2. **Print AST** after parsing  
3. **Print bytecode** after compiling
4. **Print stack state** during execution

### Common Issues:
- **Stack underflow**: Popping from empty stack
- **Undefined variables**: Accessing variables that don't exist
- **Type mismatches**: Wrong operation for value type
- **Infinite loops**: Missing loop increment

## Real-World Applications

This architecture is used in many real languages:

- **Python**: Uses bytecode compilation (similar to Jing)
- **Java**: Compiles to JVM bytecode
- **JavaScript V8**: Multi-stage compilation pipeline
- **Lua**: Register-based VM (different from stack-based)

## Further Learning

### Books:
- "Crafting Interpreters" by Robert Nystrom
- "Engineering a Compiler" by Cooper & Torczon

### Concepts to Explore:
- **Optimization**: Making code run faster
- **Garbage Collection**: Automatic memory management  
- **JIT Compilation**: Compiling during execution
- **Static Analysis**: Finding bugs before running

## Conclusion

Jing demonstrates that building a programming language isn't magic - it's a series of well-defined transformations:

1. **Text** becomes **tokens** (lexing)
2. **Tokens** become **structure** (parsing)  
3. **Structure** becomes **instructions** (compiling)
4. **Instructions** become **execution** (virtual machine)

Each step is simple and testable. Combined, they create something that feels like magic: turning human-readable code into computer execution!

The beauty of this approach is that once you understand these concepts, you can implement any programming language feature by following the same pattern through each layer of the system.
