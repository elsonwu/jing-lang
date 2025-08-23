# JiLang Compilation Pipeline Visual Guide

This document provides visual diagrams to help understand how JiLang transforms your code from text to execution.

## The Complete Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│             │    │             │    │             │    │             │    │             │
│ Source Code │───▶│   Lexer     │───▶│   Parser    │───▶│  Compiler   │───▶│     VM      │
│             │    │             │    │             │    │             │    │             │
│ "let x=42;" │    │ Tokenizer   │    │ AST Builder │    │ Code Gen    │    │ Executor    │
│             │    │             │    │             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼                   ▼
Text Characters        Token Stream        Abstract Tree        Bytecode         Final Output
```

## Example: `let x = 42 + 8;`

### Step 1: Lexical Analysis
```
Input:  "let x = 42 + 8;"
        
Lexer processes character by character:
'l' 'e' 't' ' ' 'x' ' ' '=' ' ' '4' '2' ' ' '+' ' ' '8' ';'
 └─────────┘     │       │       └───┘       │       └┘
     LET      IDENT("x") EQUAL   NUM(42)   PLUS   NUM(8) SEMICOLON

Output: [LET, IDENTIFIER("x"), EQUAL, NUMBER(42), PLUS, NUMBER(8), SEMICOLON]
```

### Step 2: Syntax Analysis  
```
Tokens: [LET, IDENTIFIER("x"), EQUAL, NUMBER(42), PLUS, NUMBER(8), SEMICOLON]

Parser builds tree structure:
                LetStmt
                   │
        ┌──────────┼──────────┐
        │                     │
    name: "x"            initializer
                              │
                         BinaryExpr
                              │
                  ┌───────────┼───────────┐
                  │           │           │
               left       operator      right  
                  │           │           │
            Literal(42)    PLUS     Literal(8)
```

### Step 3: Code Generation
```
AST: LetStmt { name: "x", initializer: BinaryExpr { left: 42, op: +, right: 8 }}

Compiler generates bytecode:
1. CONSTANT(0)   // Push 42 onto stack       Stack: [42]
2. CONSTANT(1)   // Push 8 onto stack        Stack: [42, 8]  
3. ADD           // Pop 8 and 42, push 50    Stack: [50]
4. STORE("x")    // Pop 50, store in var x   Stack: []

Constants table: [42, 8]
Variables: {"x": 50}
```

### Step 4: Virtual Machine Execution
```
Bytecode: [CONSTANT(0), CONSTANT(1), ADD, STORE("x")]

VM execution step by step:

Initial state:
├─ Stack: []
├─ Variables: {}
└─ IP (instruction pointer): 0

Step 1: CONSTANT(0)
├─ Action: Push constants[0] (42) onto stack  
├─ Stack: [42]
├─ Variables: {}
└─ IP: 1

Step 2: CONSTANT(1)  
├─ Action: Push constants[1] (8) onto stack
├─ Stack: [42, 8]
├─ Variables: {}
└─ IP: 2

Step 3: ADD
├─ Action: Pop 8 and 42, compute 42+8=50, push result
├─ Stack: [50]  
├─ Variables: {}
└─ IP: 3

Step 4: STORE("x")
├─ Action: Pop 50 from stack, store in variable "x"
├─ Stack: []
├─ Variables: {"x": 50}
└─ IP: 4

Final result: Variable x contains the value 50
```

## Stack-Based Computation Visual

The stack is like a pile of plates - you can only add to the top (push) or remove from the top (pop).

### Example: `(10 + 5) * 2`

```
Bytecode: [CONSTANT(10), CONSTANT(5), ADD, CONSTANT(2), MULTIPLY]

Step-by-step execution:

Step 1: CONSTANT(10)
Stack:
┌───────┐
│   10  │ ← Top
└───────┘

Step 2: CONSTANT(5) 
Stack:
┌───────┐
│   5   │ ← Top  
├───────┤
│   10  │
└───────┘

Step 3: ADD (pops 5 and 10, pushes 15)
Stack:
┌───────┐
│   15  │ ← Top
└───────┘

Step 4: CONSTANT(2)
Stack:  
┌───────┐
│   2   │ ← Top
├───────┤  
│   15  │
└───────┘

Step 5: MULTIPLY (pops 2 and 15, pushes 30)
Stack:
┌───────┐
│   30  │ ← Top (Final result)
└───────┘
```

## Control Flow: If Statement

### Example: `if x > 10 { print("big"); } else { print("small"); }`

```
Compilation strategy:
1. Evaluate condition (x > 10)
2. If false, jump to else branch  
3. Execute then branch, then jump past else
4. Execute else branch

Bytecode:
┌─────────────────────────────────────────────────────────────────────────┐
│ 0: LOAD("x")         // Load x onto stack                                │
│ 1: CONSTANT(10)      // Load 10 onto stack                               │  
│ 2: GREATER           // Compare: x > 10, result on stack                 │
│ 3: JUMP_IF_FALSE(7)  // If false, jump to instruction 7 (else branch)   │
│ 4: CONSTANT("big")   // Then branch: load "big"                          │
│ 5: PRINT             // Print "big"                                      │
│ 6: JUMP(9)           // Jump past else branch                            │
│ 7: CONSTANT("small") // Else branch: load "small"                        │
│ 8: PRINT             // Print "small"                                    │  
│ 9: (continue...)     // Program continues here                           │
└─────────────────────────────────────────────────────────────────────────┘

Execution flow if x = 15:
0 → 1 → 2 → 3 → 4 → 5 → 6 → 9 (skips else branch)

Execution flow if x = 5:  
0 → 1 → 2 → 3 → 7 → 8 → 9 (jumps to else branch)
```

## Function Call Mechanics

### Example: `fn add(a, b) { return a + b; }` and `let result = add(10, 20);`

```
Function Definition Compilation:
┌─────────────────────────────────────────────────────────────────────────┐
│ Function "add" compiled to:                                              │
│ 0: LOAD("a")         // Load parameter a                                │
│ 1: LOAD("b")         // Load parameter b                                │  
│ 2: ADD               // Add them                                        │
│ 3: RETURN            // Return result                                   │
│                                                                         │
│ Function stored as: Function { name: "add", arity: 2, start: 0 }        │
└─────────────────────────────────────────────────────────────────────────┘

Function Call Compilation:
┌─────────────────────────────────────────────────────────────────────────┐
│ Call site "add(10, 20)" compiled to:                                    │
│ 10: CONSTANT(10)     // Push first argument                             │
│ 11: CONSTANT(20)     // Push second argument                            │
│ 12: LOAD("add")      // Load function                                   │
│ 13: CALL(2)          // Call with 2 arguments                          │
│ 14: STORE("result")  // Store return value                              │
└─────────────────────────────────────────────────────────────────────────┘

Call Stack during execution:
┌─────────────────────────────────────────────────────────────────────────┐
│ Before CALL(2):                                                         │
│ Stack: [10, 20, Function("add")]                                        │
│ Call Stack: []                                                          │
│                                                                         │
│ During function execution:                                              │
│ Stack: [10, 20] (function removed, args remain)                        │  
│ Call Stack: [CallFrame { return_addr: 14, stack_base: 0 }]             │
│ IP jumps to function start (instruction 0)                             │
│                                                                         │
│ Function executes:                                                      │
│ - Loads a (10) and b (20)                                              │
│ - Adds them (30)                                                       │
│ - Returns 30                                                           │
│                                                                         │
│ After RETURN:                                                           │
│ Stack: [30] (return value)                                             │
│ Call Stack: [] (frame popped)                                          │
│ IP returns to 14                                                       │
└─────────────────────────────────────────────────────────────────────────┘
```

## Error Handling Flow

```
┌─────────────┐    Error?    ┌─────────────┐
│   Lexer     │─────No──────▶│   Parser    │
└─────────────┘              └─────────────┘
       │                            │
    Yes│                         Yes│
       ▼                            ▼
┌─────────────┐              ┌─────────────┐
│ Lex Error   │              │Parse Error  │  
│"Unexpected  │              │"Expected ;" │
│ character"  │              │             │
└─────────────┘              └─────────────┘

┌─────────────┐    Error?    ┌─────────────┐
│  Compiler   │─────No──────▶│     VM      │
└─────────────┘              └─────────────┘  
       │                            │
    Yes│                         Yes│
       ▼                            ▼
┌─────────────┐              ┌─────────────┐
│Compile Error│              │Runtime Error│
│"Invalid     │              │"Undefined   │
│ operation"  │              │ variable"   │  
└─────────────┘              └─────────────┘
```

## Memory Layout

```
JiLang VM Memory Layout:

┌─────────────────────────────────────────────────────────────────┐
│                        VM Memory                                │
├─────────────────────────────────────────────────────────────────┤
│ Constants Pool                                                  │
│ ┌─────┬─────┬─────┬─────┐                                      │
│ │ 42  │"hi" │true │ ... │                                      │  
│ └─────┴─────┴─────┴─────┘                                      │
├─────────────────────────────────────────────────────────────────┤
│ Bytecode Instructions                                           │
│ ┌─────────────┬─────────────┬─────────────┐                    │
│ │ CONSTANT(0) │  STORE("x") │    HALT     │                    │
│ └─────────────┴─────────────┴─────────────┘                    │
├─────────────────────────────────────────────────────────────────┤
│ Value Stack (grows up)                                         │
│ ┌─────────────┬─────────────┬─────────────┐                    │
│ │    empty    │    empty    │    empty    │                    │
│ └─────────────┴─────────────┴─────────────┘                    │
│                      ▲                                         │
│                 Stack Pointer                                  │
├─────────────────────────────────────────────────────────────────┤
│ Global Variables                                               │
│ ┌─────────────────────┬──────────────────────────────────────┐   │
│ │ "x" → Number(42)    │ "name" → String("JiLang")          │   │
│ │ "flag" → Bool(true) │ "add" → Function(...)              │   │
│ └─────────────────────┴──────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│ Call Stack (for function calls)                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ CallFrame { return_addr: 15, stack_base: 2 }               │ │
│ │ CallFrame { return_addr: 8, stack_base: 0 }                │ │
│ └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

This visual guide should help anyone understand how JiLang transforms source code into executable instructions, step by step!
