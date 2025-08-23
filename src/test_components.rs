use jilang::{lexer::Lexer, parser::Parser, compiler::Compiler, vm::VM};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing JiLang components step by step...");
    
    // Test 1: Lexer
    println!("\n--- Testing Lexer ---");
    let input = "1 + 2";
    println!("Input: '{}'", input);
    
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    println!("Tokens: {:#?}", tokens);
    
    // Test 2: Parser
    println!("\n--- Testing Parser ---");
    let mut parser = Parser::new(tokens);
    let statement = parser.parse_repl_expression()?;
    println!("AST: {:#?}", statement);
    let statements = vec![statement];
    
    // Test 3: Compiler
    println!("\n--- Testing Compiler ---");
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;
    println!("Bytecode: {:#?}", chunk);
    
    // Test 4: VM
    println!("\n--- Testing VM ---");
    let mut vm = VM::new();
    vm.interpret(chunk).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    println!("\nAll tests completed successfully!");
    Ok(())
}
