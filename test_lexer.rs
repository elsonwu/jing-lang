use jilang::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("1 + 2".to_string());
    let tokens = lexer.tokenize();
    
    println!("Tokens: {:#?}", tokens);
}
