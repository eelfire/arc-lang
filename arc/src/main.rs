pub mod lexer;
pub mod token;

fn main() {
    println!("Hello from simple arc-lang compiler!");
    lexer::run();
}
