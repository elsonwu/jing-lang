mod token;
mod lexer;
mod ast;
mod parser;
mod bytecode;
mod compiler;
mod vm;

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;
use vm::VM;
use std::io::{self, Write};

fn main() {
    println!("JiLang - A Simple Toy Language with VM");
    println!("Type 'exit' to quit, or enter code to execute:");
    
    let mut vm = VM::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                if input == "exit" || input == "quit" {
                    println!("Goodbye!");
                    break;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                match run_code(input, &mut vm) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn run_code(source: &str, vm: &mut VM) -> Result<(), Box<dyn std::error::Error>> {
    // Lex
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize();
    
    // Parse
    let mut parser = Parser::new(tokens);
    let statement = parser.parse_repl_expression()?;
    let statements = vec![statement];
    
    // Compile
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    
    // Execute
    vm.interpret(chunk).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // For REPL, print the result if there's something on the stack
    if !vm.stack.is_empty() {
        let result = vm.stack.last().unwrap();
        println!("{}", vm.value_to_string(result));
    }
    
    Ok(())
}
