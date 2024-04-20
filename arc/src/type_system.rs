// type system, type checking, type inference

// goal: output a type-checked, type-populated AST
// steps:
// 1. type inference
// 2. type checking

use crate::pair_to_tree::{Node, Type};
use crate::parser::Rule;

// type inference
pub fn infer_types(tree: &mut Vec<Node>) {
    // traverse the tree and infer types of nodes if nodes are
    // LHS: immutable, mutable, variable reassignment
    // RHS: expressions
    // functions
    // structs
    // enums
    // arrays
    // tuples
    // lists

    for i in 0..tree.len() {
        match tree[i].rule {
            Rule::IMMUT => {
                // infer type of RHS
                let mut j = 0;
                while tree[i + j].rule != Rule::EXPRESSION {
                    j += 1;
                }
                // infer type of expression
                tree[i].type_ = tree[i + j].type_.clone();
            }
            Rule::MUT => {
                // infer type of RHS
                let mut j = 0;
                while tree[i + j].rule != Rule::EXPRESSION {
                    j += 1;
                }
                // infer type of expression
                tree[i].type_ = tree[i + j].type_.clone();
            }
            Rule::VARIABLE_REASS => {
                // infer type of RHS
                let mut j = 0;
                while tree[i + j].rule != Rule::EXPRESSION {
                    j += 1;
                }
                // infer type of expression
                tree[i].type_ = tree[i + j].type_.clone();
            }
            _ => {}
        }
    }
}

// type checking
pub fn check_types(tree: &Vec<Node>) {
    // traverse the tree and check types of nodes
    // LHS: immutable, mutable, variable reassignment
    // RHS: expressions
    // functions
    // structs
    // enums
    // arrays
    // tuples
    // lists
}
