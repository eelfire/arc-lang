// ast to wat (web assembly text format)

use crate::pair_to_tree::{Node, Type};
use crate::parser::Rule;

// example add.arc
/*
Node {
    rule: PROGRAM,
    text: "pub fx add(a i32, b i32) ~ i32 {\n    let c = a + b;\n    return c;\n}\n/*\nfx main() {\n    let mut a = 4;\n    a = 3;\n    let b = 5;\n    let c i32 = add(a, b);\n    print(\"{}\", c);\n    let ch char = ' ';\n}\n*/
",
    children: [
        Node {
            rule: PROGRAM_BLOCK,
            text: "pub fx add(a i32, b i32) ~ i32 {\n    let c = a + b;\n    return c;\n}",
            children: [
                Node {
                    rule: FUNCTION_DECL,
                    text: "pub fx add(a i32, b i32) ~ i32 {\n    let c = a + b;\n    return c;\n}",
                    children: [
                        Node {
                            rule: a_pub,
                            text: "pub",
                            children: [],
                        },
                        Node {
                            rule: IDENTIFIER,
                            text: "add",
                            children: [],
                        },
                        Node {
                            rule: PARAMETER_LIST,
                            text: "(a i32, b i32)",
                            children: [
                                Node {
                                    rule: PARAMETER,
                                    text: "a i32",
                                    children: [
                                        Node {
                                            rule: IDENTIFIER,
                                            text: "a",
                                            children: [],
                                        },
                                        Node {
                                            rule: ANY_TYPE,
                                            text: "i32",
                                            children: [],
                                        },
                                    ],
                                },
                                Node {
                                    rule: PARAMETER,
                                    text: "b i32",
                                    children: [
                                        Node {
                                            rule: IDENTIFIER,
                                            text: "b",
                                            children: [],
                                        },
                                        Node {
                                            rule: ANY_TYPE,
                                            text: "i32",
                                            children: [],
                                        },
                                    ],
                                },
                            ],
                        },
                        Node {
                            rule: RETURN_TYPE,
                            text: "~ i32",
                            children: [
                                Node {
                                    rule: TYPE,
                                    text: "i32",
                                    children: [],
                                },
                            ],
                        },
                        Node {
                            rule: FUNCTION_BODY,
                            text: "{\n    let c = a + b;\n    return c;\n}",
                            children: [
                                Node {
                                    rule: SCOPE_START,
                                    text: "{",
                                    children: [],
                                },
                                Node {
                                    rule: STATEMENT,
                                    text: "let c = a + b;",
                                    children: [
                                        Node {
                                            rule: DECL_STMT,
                                            text: "let c = a + b;",
                                            children: [
                                                Node {
                                                    rule: IMMUT,
                                                    text: "let c = a + b;",
                                                    children: [
                                                        Node {
                                                            rule: IDENTIFIER,
                                                            text: "c",
                                                            children: [],
                                                        },
                                                        Node {
                                                            rule: ASSIGNMENT,
                                                            text: "=",
                                                            children: [
                                                                Node {
                                                                    rule: assign,
                                                                    text: "=",
                                                                    children: [],
                                                                },
                                                            ],
                                                        },
                                                        Node {
                                                            rule: EXPRESSION,
                                                            text: "a + b",
                                                            children: [
                                                                Node {
                                                                    rule: EXPRESSION_TYPE,
                                                                    text: "",
                                                                    children: [],
                                                                },
                                                                Node {
                                                                    rule: FACTOR,
                                                                    text: "a ",
                                                                    children: [
                                                                        Node {
                                                                            rule: DATA_TYPES,
                                                                            text: "a",
                                                                            children: [
                                                                                Node {
                                                                                    rule: IDENTIFIER,
                                                                                    text: "a",
                                                                                    children: [],
                                                                                },
                                                                            ],
                                                                        },
                                                                    ],
                                                                },
                                                                Node {
                                                                    rule: OPERATOR,
                                                                    text: "+",
                                                                    children: [
                                                                        Node {
                                                                            rule: OPERATOR_LEVEL_2,
                                                                            text: "+",
                                                                            children: [
                                                                                Node {
                                                                                    rule: add,
                                                                                    text: "+",
                                                                                    children: [],
                                                                                },
                                                                            ],
                                                                        },
                                                                    ],
                                                                },
                                                                Node {
                                                                    rule: FACTOR,
                                                                    text: "b",
                                                                    children: [
                                                                        Node {
                                                                            rule: DATA_TYPES,
                                                                            text: "b",
                                                                            children: [
                                                                                Node {
                                                                                    rule: IDENTIFIER,
                                                                                    text: "b",
                                                                                    children: [],
                                                                                },
                                                                            ],
                                                                        },
                                                                    ],
                                                                },
                                                            ],
                                                        },
                                                        Node {
                                                            rule: STATEMENT_END,
                                                            text: ";",
                                                            children: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Node {
                                    rule: RETURN_STMT,
                                    text: "return c;",
                                    children: [
                                        Node {
                                            rule: EXPRESSION,
                                            text: "c",
                                            children: [
                                                Node {
                                                    rule: EXPRESSION_TYPE,
                                                    text: "",
                                                    children: [],
                                                },
                                                Node {
                                                    rule: FACTOR,
                                                    text: "c",
                                                    children: [
                                                        Node {
                                                            rule: DATA_TYPES,
                                                            text: "c",
                                                            children: [
                                                                Node {
                                                                    rule: IDENTIFIER,
                                                                    text: "c",
                                                                    children: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        Node {
                                            rule: STATEMENT_END,
                                            text: ";",
                                            children: [],
                                        },
                                    ],
                                },
                                Node {
                                    rule: SCOPE_END,
                                    text: "}",
                                    children: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Node {
            rule: EOI,
            text: "",
            children: [],
        },
    ],
}
 */

pub fn convert_to_wat(node: &Node) -> String {
    let mut wat = String::new();
    match node.rule {
        Rule::PROGRAM => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::PROGRAM_BLOCK => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::FUNCTION_DECL => {
            let mut function_name = String::new();
            let mut parameters = String::new();
            let mut return_type = String::new();
            let mut function_body = String::new();
            for child in &node.children {
                match child.rule {
                    Rule::IDENTIFIER => function_name = child.text.clone(),
                    Rule::PARAMETER_LIST => parameters = convert_to_wat(child),
                    Rule::RETURN_TYPE => return_type = convert_to_wat(child),
                    Rule::FUNCTION_BODY => function_body = convert_to_wat(child),
                    _ => {}
                }
            }
            wat.push_str(&format!(
                "(func ${}{}{}",
                function_name, parameters, return_type
            ));
            wat.push_str(&function_body);
            wat.push_str(")\n");
        }
        Rule::PARAMETER_LIST => {
            let mut parameters = String::new();
            for child in &node.children {
                parameters.push_str(&convert_to_wat(child));
            }
            wat.push_str(&parameters);
        }
        Rule::PARAMETER => {
            let mut parameter_name = String::new();
            let mut parameter_type = String::new();
            for child in &node.children {
                match child.rule {
                    Rule::IDENTIFIER => parameter_name = child.text.clone(),
                    Rule::ANY_TYPE => parameter_type = child.text.clone(),
                    _ => {}
                }
            }
            wat.push_str(&format!(" (param ${} {})", parameter_name, parameter_type));
        }
        Rule::RETURN_TYPE => {
            let mut return_type = String::new();
            for child in &node.children {
                return_type.push_str(&convert_to_wat(child));
            }
            wat.push_str(&format!(" (result {})", return_type));
        }
        Rule::FUNCTION_BODY => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::SCOPE_START => {
            wat.push_str(" (local ");
        }
        Rule::SCOPE_END => {
            wat.push_str(")\n");
        }
        Rule::STATEMENT => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::DECL_STMT => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::IMMUT => {
            wat.push_str(" (local.set ");
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
            wat.push_str(")");
        }
        Rule::IDENTIFIER => {
            wat.push_str(&format!("${}", node.text));
        }
        Rule::ASSIGNMENT => {
            wat.push_str(" (local ");
        }
        Rule::EXPRESSION => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::FACTOR => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::OPERATOR => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::STATEMENT_END => {
            wat.push_str(")");
        }
        Rule::RETURN_STMT => {
            for child in &node.children {
                wat.push_str(&convert_to_wat(child));
            }
        }
        Rule::EOI => {}
        _ => {}
    }
    wat
}
