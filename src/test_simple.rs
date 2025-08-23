use std::io::{self, Write};

fn main() {
    println!("JiLang Test - Simple Input Test");
    
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            println!("You entered: '{}'", input);
            
            if input == "exit" {
                println!("Goodbye!");
                return;
            }
        }
        Err(e) => {
            println!("Error reading input: {}", e);
        }
    }
}
