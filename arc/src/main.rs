pub mod lexer;
pub mod token;

fn main() {
    println!("Hello from simple arc-lang compiler!");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: arc <input-file>");
        std::process::exit(1);
    }
    lexer::run(args[1].as_str());
}
