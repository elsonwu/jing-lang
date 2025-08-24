use jing::*;
use jing::vm::REPL;
use std::env;
use std::fs;
use std::process;

fn main() {
    // Initialize the modular language system
    jing::init();
    
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments - start REPL
            let mut repl = REPL::new();
            if let Err(err) = repl.run() {
                eprintln!("REPL error: {}", err);
                process::exit(1);
            }
        }
        2 => {
            // One argument - interpret file
            let filename = &args[1];
            if let Err(err) = run_file(filename) {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage: {} [script.jing]", args[0]);
            process::exit(1);
        }
    }
}

fn run_file(filename: &str) -> JingResult<()> {
    let source = fs::read_to_string(filename).map_err(|err| {
        JingError::io_error(format!("Could not read file '{}': {}", filename, err))
    })?;

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;

    let mut vm = VM::new();
    vm.interpret(chunk)?;

    Ok(())
}
