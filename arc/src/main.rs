pub mod lexer;
pub mod pair_to_tree;
pub mod parser;
pub mod semantic_analysis;
pub mod token;
// pub mod tree_to_wasm;
pub mod tree_to_wat;
pub mod type_system;

use std::fs;

use crate::pair_to_tree::{pair_to_nodes, unflatten};
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
    let program_block = program_copy.unwrap().next().unwrap();
    let mut tree = pair_to_nodes(program_block);
    let mut flatten_tree = tree.flatten();
    // println!("{:#?}", flatten_tree);

    match program {
        Ok(pairs) => {
            print_nested_pairs(&pairs, 0);
            println!("\n\n>>> Analyzing...");
            analyze(pairs, &mut flatten_tree, file_path);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    type_system::infer_types(&mut flatten_tree);
    type_system::check_types(&flatten_tree);
    // println!("{:#?}", flatten_tree);

    let tree = unflatten(&tree, flatten_tree);
    println!("\nType annotated tree:\n{:#?}", tree);

    let wat = tree_to_wat::convert_to_wat(&tree);
    // println!("{}", wat);

    // let wasm = tree_to_wasm::convert_to_wasm(&flatten_tree);
    // let wasm_file_path = format!("{}.wasm", file_path);
}
