// pest pair to tree

use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Node {
    pub rule: Rule,
    pub text: String,
    pub children: Vec<Node>,
}

pub fn pair_to_nodes(pair: Pair<Rule>) -> Node {
    let rule = pair.as_rule();
    // let span = pair.as_span();
    let text = pair.as_str().to_string();
    let children: Vec<Node> = pair.into_inner().map(pair_to_nodes).collect();
    Node {
        rule,
        text,
        children,
    }
}
