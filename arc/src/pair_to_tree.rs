// pest pair to tree

use core::slice::Iter;

use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
    String,
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    List(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Struct(Vec<(String, Type)>),
    Enum(Vec<(String, Option<Type>)>),
    Any,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub rule: Rule,
    pub text: String,
    pub type_: Option<Type>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn flatten(&self) -> Vec<Node> {
        let mut new_node = self.clone();
        // remove children
        new_node.children.clear();
        let mut nodes = vec![new_node];
        for child in &self.children {
            nodes.extend(child.flatten());
        }
        nodes
    }
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
        type_: None,
    }
}

// unflatten a flatten tree: take tree (Node) as template, and flatten_tree (Vec<Node>) as input
// create new Node with template of tree but values from flatten_tree
pub fn unflatten(node: &Node, flatten_tree_iter: &mut Iter<Node>) -> Node {
    let mut new_node = node.clone();

    if let Some(flatten_node) = flatten_tree_iter.next() {
        new_node.rule = flatten_node.rule.clone();
        new_node.text = flatten_node.text.clone();
        new_node.type_ = flatten_node.type_.clone();
    }

    for child in &mut new_node.children {
        *child = unflatten(&child, flatten_tree_iter);
        // flatten_tree_iter.next();
    }

    new_node
}
