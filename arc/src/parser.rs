use crate::lexer;
use crate::token;

pub fn run(file_path: &str) {
    println!("Parsing Stage Initiated...");
    let tokens = lexer::run(file_path);
    println!("{:?}", tokens);
    println!("Parsing Stage Completed...");
}
