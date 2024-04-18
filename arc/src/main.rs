pub mod lexer;
pub mod pair_to_tree;
pub mod parser;
pub mod semantic_analysis;
pub mod token;

use std::fs;

use crate::pair_to_tree::pair_to_nodes;
use crate::parser::print_nested_pairs;
use crate::semantic_analysis::analyze;

fn main() {
    println!("Hello from simple arc-lang compiler!");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: arc <input-file>");
        std::process::exit(1);
    }
    let file_path = args[1].as_str();
    let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");

    let program = parser::run(&unparsed_file);
    let program_copy = program.clone();
    match program {
        Ok(pairs) => {
            print_nested_pairs(&pairs, 0);
            println!("\n\n>>> Analyzing...");
            analyze(pairs, file_path);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    let program_block = program_copy.unwrap().next().unwrap();
    let tree = pair_to_nodes(program_block);
    // println!("{:#?}", tree);
}
