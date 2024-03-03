use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArcParser;

pub fn run(file_path: &str) {
    let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");

    let program = ArcParser::parse(Rule::PROGRAM, &unparsed_file);

    match program {
        Ok(pairs) => {
            print_nested_pairs(pairs, 0);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn print_nested_pairs(pairs: Pairs<Rule>, depth: usize) {
    for pair in pairs {
        println!(
            "{:indent$}{:?}: {:?}",
            "",
            pair.as_rule(),
            pair.as_str(),
            indent = depth * 2
        );
        print_nested_pairs(pair.into_inner(), depth + 1);
    }
}
