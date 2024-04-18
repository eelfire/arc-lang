use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArcParser;

pub fn run(unparsed_file: &String) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    let program = ArcParser::parse(Rule::PROGRAM, unparsed_file);
    return program;
}

pub fn print_nested_pairs(pairs: &Pairs<Rule>, depth: usize) {
    for pair in pairs.clone() {
        println!(
            "{:indent$}{:?}: {:?}",
            "",
            pair.as_rule(),
            pair.as_str(),
            indent = depth * 2
        );
        print_nested_pairs(&pair.into_inner(), depth + 1);
    }
}
