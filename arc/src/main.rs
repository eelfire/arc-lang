pub mod lexer;
pub mod parser;
pub mod token;

fn main() {
    println!("Hello from simple arc-lang compiler!");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: arc <input-file>");
        std::process::exit(1);
    }
    let file_path = args[1].as_str();
    parser::run(file_path);
}
