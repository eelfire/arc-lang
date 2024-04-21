// convert tree(Node) to wasm binary using wasm-encoder

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, MemArg,
    MemorySection, MemoryType, Module, TableSection, TableType, TypeSection, ValType,
};

use crate::pair_to_tree::Node;
use crate::pair_to_tree::Type;

pub fn convert_to_wasm(tree: &Node) -> Vec<u8> {
    let mut module = Module::new();
    let mut type_section = TypeSection::new();
    let mut function_section = FunctionSection::new();
    let mut code_section = CodeSection::new();
    let mut export_section = ExportSection::new();

    // let mut function_types = vec![];
    // let mut function_bodies = vec![];
    let mut function_index = 0;

    // states

    // traverse tree

    let wasm_bytes = module.finish();
    assert!(wasmparser::validate(&wasm_bytes).is_ok());
    wasm_bytes
}
