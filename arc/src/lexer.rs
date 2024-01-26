use std::path::Path;

use crate::token::Token;

pub fn run() {
    println!("hi from lexer");

    // read chars from utf-8 encoded file
    let path = Path::new("testcases/add.arc");
    let contents = std::fs::read_to_string(path).expect("failed to read file");
    for c in contents.chars() {
        if c == '(' {
            let tok = Token::LParen;
            println!("{:?}", tok);
        }
    }
}
