// pest pair to tree

use std::ops::{Deref, DerefMut};

use crate::parser::Rule;
use crate::semantic_analysis::SymbolType;
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
pub fn unflatten(tree: &Node, flatten_tree: Vec<Node>) -> Node {
    let mut tree = tree.clone();
    let mut flatten_tree = flatten_tree.into_iter();
    flatten_tree.next();
    let mut new_children = vec![];
    for child in &tree.children {
        if child.children.is_empty() {
            let next = flatten_tree.next();
            if let Some(next) = next {
                let mut new_child = child.clone();
                new_child.text = next.text.clone();
                new_child.type_ = next.type_.clone();
                new_children.push(new_child);
            }
        } else {
            new_children.push(unflatten(child, flatten_tree.by_ref().collect()));
        }
    }
    tree.children = new_children;
    tree
}
