// convert tree(Node) to wasm binary using wasm-encoder
/*
Node {
    rule: PROGRAM,
    text: "fx add(a i32, b i32) ~ i32 {\n    return a + b;\n}\n\nfx sub(a i32, b i32) ~ i32 {\n    return a - b;\n}\n\nfx mul(a i32, b i32) ~ i32 {\n    return a * b;\n}\n\nfx div(a i32, b i32) ~ i32 {\n    return a / b;\n}\n\nfx modo(a i32, b i32) ~ i32 {\n    return a % b;\n}",
    type_: None,
    children: [
        Node {
            rule: PROGRAM_BLOCK,
            text: "fx add(a i32, b i32) ~ i32 {\n    return a + b;\n}\n\nfx sub(a i32, b i32) ~ i32 {\n    return a - b;\n}\n\nfx mul(a i32, b i32) ~ i32 {\n    return a * b;\n}\n\nfx div(a i32, b i32) ~ i32 {\n    return a / b;\n}\n\nfx modo(a i32, b i32) ~ i32 {\n    return a % b;\n}",
            type_: None,
            children: [
                Node {
                    rule: FUNCTION_DECL,
                    text: "fx add(a i32, b i32) ~ i32 {\n    return a + b;\n}",
                    type_: Some(
                        Function(
                            [
                                I32,
                                I32,
                            ],
                            I32,
                        ),
                    ),
                    children: [
                        Node {
                            rule: IDENTIFIER,
                            text: "add",
                            type_: None,
                            children: [],
                        },
                        Node {
                            rule: PARAMETER_LIST,
                            text: "(a i32, b i32)",
                            type_: None,
                            children: [
                                Node {
                                    rule: PARAMETER,
                                    text: "a i32",
                                    type_: None,
                                    children: [
                                        Node {
                                            rule: IDENTIFIER,
                                            text: "a",
                                            type_: Some(
                                                I32,
                                            ),
                                            children: [],
                                        },
                                        Node {
                                            rule: ANY_TYPE,
                                            text: "i32",
                                            type_: None,
                                            children: [
                                                Node {
                                                    rule: TYPE,
                                                    text: "i32",
                                                    type_: None,
                                                    children: [
                                                        Node {
                                                            rule: I32,
                                                            text: "i32",
                                                            type_: None,
                                                            children: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Node {
                                    rule: PARAMETER,
                                    text: "b i32",
                                    type_: None,
                                    children: [
                                        Node {
                                            rule: IDENTIFIER,
                                            text: "b",
                                            type_: Some(
                                                I32,
                                            ),
                                            children: [],
                                        },
                                        Node {
                                            rule: ANY_TYPE,
                                            text: "i32",
                                            type_: None,
                                            children: [
                                                Node {
                                                    rule: TYPE,
                                                    text: "i32",
                                                    type_: None,
                                                    children: [
                                                        Node {
                                                            rule: I32,
                                                            text: "i32",
                                                            type_: None,
                                                            children: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                        Node {
                            rule: RETURN_TYPE,
                            text: "~ i32",
                            type_: Some(
                                I32,
                            ),
                            children: [
                                Node {
                                    rule: TYPE,
                                    text: "i32",
                                    type_: None,
                                    children: [
                                        Node {
                                            rule: I32,
                                            text: "i32",
                                            type_: None,
                                            children: [],
                                        },
                                    ],
                                },
                            ],
                        },
                        Node {
                            rule: FUNCTION_BODY,
                            text: "{\n    return a + b;\n}",
                            type_: None,
                            children: [
                                Node {
                                    rule: SCOPE_START,
                                    text: "{",
                                    type_: None,
                                    children: [],
                                },
                                Node {
                                    rule: RETURN_STMT,
                                    text: "return a + b;",
                                    type_: None,
                                    children: [
                                        Node {
                                            rule: EXPRESSION,
                                            text: "a + b",
                                            type_: Some(
                                                I32,
                                            ),
                                            children: [
                                                Node {
                                                    rule: FACTOR,
                                                    text: "a ",
                                                    type_: None,
                                                    children: [
                                                        Node {
                                                            rule: DATA_TYPES,
                                                            text: "a",
                                                            type_: None,
                                                            children: [
                                                                Node {
                                                                    rule: IDENTIFIER,
                                                                    text: "a",
                                                                    type_: None,
                                                                    children: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                                Node {
                                                    rule: OPERATOR,
                                                    text: "+",
                                                    type_: None,
                                                    children: [
                                                        Node {
                                                            rule: OPERATOR_LEVEL_2,
                                                            text: "+",
                                                            type_: None,
                                                            children: [
                                                                Node {
                                                                    rule: add,
                                                                    text: "+",
                                                                    type_: None,
                                                                    children: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                                Node {
                                                    rule: FACTOR,
                                                    text: "b",
                                                    type_: None,
                                                    children: [
                                                        Node {
                                                            rule: DATA_TYPES,
                                                            text: "b",
                                                            type_: None,
                                                            children: [
                                                                Node {
                                                                    rule: IDENTIFIER,
                                                                    text: "b",
                                                                    type_: None,
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
                                            type_: None,
                                            children: [],
                                        },
                                    ],
                                },
                                Node {
                                    rule: SCOPE_END,
                                    text: "}",
                                    type_: None,
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
            type_: None,
            children: [],
        },
    ],
}
*/

use wasm_encoder::Encode;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, MemArg,
    MemorySection, MemoryType, Module, TableSection, TableType, TypeSection, ValType,
};
use wasmparser::types;

use crate::pair_to_tree::Node;
use crate::pair_to_tree::Type;
use crate::parser::Rule;
use crate::semantic_analysis::SymbolTable;
use crate::semantic_analysis::SymbolType;

struct Context {
    functions: Vec<Function>,
    types: Vec<Function>,
    memory: Option<MemoryType>,
    // ...
}

fn arc_type_to_wasm_type(t: &Type) -> ValType {
    match t {
        Type::I32 => ValType::I32,
        Type::I64 => ValType::I64,
        Type::F32 => ValType::F32,
        Type::F64 => ValType::F64,
        Type::Bool => ValType::I32,
        Type::Char => ValType::I32,
        Type::String => ValType::I32,
        Type::Array(_, _) => ValType::I32,
        Type::Tuple(_) => ValType::I32,
        Type::List(_) => ValType::I32,
        Type::Function(_, _) => ValType::I32,
        Type::Struct(_) => ValType::I32,
        Type::Enum(_) => ValType::I32,
        Type::Any => ValType::I32,
    }
}

fn hash_identifier(node: &Node) -> u32 {
    let mut hash = 0;
    for c in node.text.chars() {
        hash = (hash << 5) - hash + c as u32 - 97;
    }
    hash
}

fn visit_node(
    node: &Node,
    module: &mut Module,
    type_section: &mut TypeSection,
    function_section: &mut FunctionSection,
    code_section: &mut CodeSection,
    export_section: &mut ExportSection,
    function_index: &mut u32,
    last_symbol_type: &mut SymbolType,
    types: &mut Vec<(Vec<ValType>, Vec<ValType>)>,
    functions: &mut Vec<Function>,
    exports: &mut Vec<(String, ExportKind)>,
    f: &mut Function,
) {
    // println!("{:?}", node.rule);
    match node.rule {
        Rule::PARAMETER => {
            *last_symbol_type = SymbolType::Parameter;
        }
        Rule::FUNCTION_DECL => {
            *last_symbol_type = SymbolType::Function;
            let mut params = vec![];
            let mut results = vec![];

            // destructuring node.type_ to get function type
            if let Some(Type::Function(p, r)) = &node.type_ {
                for param in p {
                    params.push(arc_type_to_wasm_type(param));
                }
                results.push(arc_type_to_wasm_type(r));
            }
            types.push((params.clone(), results.clone()));
        }
        Rule::STRUCT_DECL => {
            *last_symbol_type = SymbolType::Struct;
        }
        Rule::ENUM_DECL => {
            *last_symbol_type = SymbolType::Enum;
        }
        Rule::IMPL_DECL => {
            *last_symbol_type = SymbolType::Impl;
        }
        Rule::MUT => {
            *last_symbol_type = SymbolType::Mut;
        }
        Rule::IMMUT => {
            *last_symbol_type = SymbolType::Immut;
        }
        Rule::FUNCTION_CALL => {
            *last_symbol_type = SymbolType::FnCall;
        }
        Rule::IMPL_ACCESS => {
            *last_symbol_type = SymbolType::ImplAccess;
        }
        Rule::STRUCT_ACCESS => {
            *last_symbol_type = SymbolType::StructAccess;
        }
        Rule::ENUM_ACCESS => {
            *last_symbol_type = SymbolType::EnumAccess;
        }
        Rule::LIST_ACCESS => {
            *last_symbol_type = SymbolType::ListAccess;
        }
        Rule::EXPRESSION => {
            *last_symbol_type = SymbolType::Expression;
        }

        Rule::IDENTIFIER => {
            // if function name then add to export
            // if function call then add to function call
            // if struct name then add to struct
            // if enum name then add to enum
            // if impl name then add to impl
            // if variable (mut, immut, variable reassign) then add to variable
            // if function parameter then add to function parameter
            // if function return type then add to function return type

            let symbol = node.text.clone();

            match last_symbol_type {
                // SymbolType::Parameter => todo!(),
                SymbolType::Function => {
                    // add function to export
                    exports.push((node.text.clone(), ExportKind::Func));
                }
                // SymbolType::Struct => todo!(),
                // SymbolType::Enum => todo!(),
                // SymbolType::Impl => todo!(),
                SymbolType::Mut => {
                    f.instruction(&Instruction::LocalSet(hash_identifier(node)));
                }
                SymbolType::Immut => {
                    f.instruction(&Instruction::LocalSet(hash_identifier(node)));
                }
                // SymbolType::FnCall => todo!(),
                // SymbolType::ImplAccess => todo!(),
                // SymbolType::StructAccess => todo!(),
                // SymbolType::EnumAccess => todo!(),
                // SymbolType::TupAccess => todo!(),
                // SymbolType::ArrAccess => todo!(),
                // SymbolType::ListAccess => todo!(),
                SymbolType::Expression => {
                    f.instruction(&Instruction::LocalGet(hash_identifier(node)));
                }
                // SymbolType::Other => todo!(),
                _ => {}
            }
        }

        Rule::multiply => {
            f.instruction(&Instruction::I32Mul);
        }
        Rule::divide => {
            f.instruction(&Instruction::I32DivS);
        }
        Rule::remainder => {
            f.instruction(&Instruction::I32RemS);
        }
        Rule::add => {
            f.instruction(&Instruction::I32Add);
        }
        Rule::subtract => {
            f.instruction(&Instruction::I32Sub);
        }

        Rule::SCOPE_END => {
            f.instruction(&Instruction::End);

            functions.push(f.clone());

            // clear f
            *f = Function::new(vec![]);
        }

        Rule::EOI => {
            return;
        }

        _ => {
            // println!(">>> {:?}", node.rule);
        }
    }
    for child in &node.children {
        visit_node(
            child,
            module,
            type_section,
            function_section,
            code_section,
            export_section,
            function_index,
            last_symbol_type,
            types,
            functions,
            exports,
            f,
        );
    }
}

pub fn convert_to_wasm(tree: &Node) -> Vec<u8> {
    let mut module = Module::new();
    let mut type_section = TypeSection::new();
    let mut function_section = FunctionSection::new();
    let mut code_section = CodeSection::new();
    let mut export_section = ExportSection::new();
    let mut function_index = 0;

    // generation context

    let mut last_symbol_type = SymbolType::Other;
    let mut types = vec![];
    let mut functions = vec![];
    let mut exports = vec![];

    // visit nodes
    visit_node(
        tree,
        &mut module,
        &mut type_section,
        &mut function_section,
        &mut code_section,
        &mut export_section,
        &mut function_index,
        &mut last_symbol_type,
        &mut types,
        &mut functions,
        &mut exports,
        &mut Function::new(vec![]),
    );

    // add types
    for t in types {
        type_section.function(t.0.into_iter(), t.1.into_iter());
    }
    module.section(&type_section);

    // add functions
    for _ in 0..functions.len() {
        function_section.function(function_index);
        function_index += 1;
    }
    module.section(&function_section);

    // add memory
    let memory = MemoryType {
        minimum: 256,
        maximum: Some(256),
        memory64: false,
        shared: false,
        page_size_log2: None,
    };
    let mut memory_section = MemorySection::new();
    memory_section.memory(memory);
    module.section(&memory_section);

    // add export
    export_section.export("memory", ExportKind::Memory, 0);
    function_index = 0;
    for e in exports {
        export_section.export(&e.0, e.1, function_index);
        function_index += 1;
    }
    module.section(&export_section);

    println!("functions: {:?}", functions);
    for f in functions {
        code_section.function(&f);
    }
    module.section(&code_section);

    let wasm_bytes = module.finish();
    // assert!(wasmparser::validate(&wasm_bytes).is_ok());

    let wasm_print = wasmprinter::print_bytes(&wasm_bytes).unwrap();
    println!("{}", wasm_print);

    wasm_bytes
}

pub fn demo() {
    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();
    let params = vec![ValType::I32, ValType::I32];
    let results = vec![ValType::I32];
    types.function(params, results);
    module.section(&types);

    // Encode the function section.
    let mut functions = FunctionSection::new();
    let type_index = 0;
    functions.function(type_index);
    module.section(&functions);

    // Encode the export section.
    let mut exports = ExportSection::new();
    exports.export("f", ExportKind::Func, 0);
    module.section(&exports);

    // Encode the code section.
    let mut codes = CodeSection::new();
    let locals = vec![];
    let mut f = Function::new(locals);
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(1));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    let wasm_bytes = module.finish();

    // We generated a valid Wasm module!
    assert!(wasmparser::validate(&wasm_bytes).is_ok());

    // Print WAT
    let wasm_print = wasmprinter::print_bytes(&wasm_bytes).unwrap();
    println!("{}", wasm_print);
}
