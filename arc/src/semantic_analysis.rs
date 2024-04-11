use crate::parser::Rule;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::parser::print_nested_pairs;

// "i32" | "i64" | "f32" | "f64" | "char" | "bool" | "string"
const BUILTIN_DATA_TYPES: [&str; 7] = ["i32", "i64", "f32", "f64", "char", "bool", "string"];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Parameter,
    // Identifier,
    Function,
    Struct,
    Enum,
    Impl,
    Mut,
    Immut,
    // VariableReass,
    FnCall,
    ImplAccess,
    StructAccess,
    EnumAccess,
    // ModAccess,
    TupAccess,
    ArrAccess,
    ListAccess,
    Other,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    location: (usize, usize),
    // used: bool,
    // other fields...
}

#[derive(Debug, Clone)]
pub struct Scope {
    name: String,
    parent: String,
    symbols: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: HashMap<String, Scope>,
    current_scope: String,
}

pub trait SymbolTableTrait {
    fn insert_scope(&mut self, scope: Scope);
    fn insert_symbol(&mut self, symbol: Symbol);
    fn get_symbol(&self, symbol_name: &String, scope_name: &String) -> Option<&Symbol>;
    fn get_scope(&self, scope_name: &String) -> Option<&Scope>;
    fn get_current_scope(&self) -> &String;
    fn set_current_scope(&mut self, scope_name: String);
    fn get_symbol_recursive(&self, symbol_name: &String, scope_name: &String) -> Option<&Symbol>;
    // fn already_exists(&self, symbol_name: String, scope_name: String) -> Option<&Symbol>;
}

impl SymbolTableTrait for SymbolTable {
    fn insert_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.name.clone(), scope);
    }

    fn insert_symbol(&mut self, symbol: Symbol) {
        self.scopes
            .get_mut(&self.current_scope)
            .unwrap()
            .symbols
            .insert(symbol.name.clone(), symbol);
    }

    fn get_symbol(&self, symbol_name: &String, scope_name: &String) -> Option<&Symbol> {
        self.scopes
            .get(scope_name)
            .unwrap()
            .symbols
            .get(symbol_name)
    }

    fn get_scope(&self, scope_name: &String) -> Option<&Scope> {
        self.scopes.get(scope_name)
    }

    fn get_current_scope(&self) -> &String {
        &self.current_scope
    }

    fn set_current_scope(&mut self, scope_name: String) {
        self.current_scope = scope_name;
    }

    fn get_symbol_recursive(&self, symbol_name: &String, scope_name: &String) -> Option<&Symbol> {
        let mut current_scope = scope_name;
        loop {
            let scope = self.get_scope(current_scope).unwrap();
            if let Some(symbol) = scope.symbols.get(symbol_name) {
                return Some(symbol);
            }
            if scope.parent == "" {
                return None;
            }
            current_scope = &scope.parent;
        }
    }

    // fn already_exists(&self, symbol_name: &String, scope_name: &String) -> Option<&Symbol> {
    //     self.scopes
    //         .get(scope_name)
    //         .unwrap()
    //         .symbols
    //         .get(symbol_name)
    // }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum FlagType {
    Expression,
    CallStmt,
    // FlowControl,
    For,
}

fn analyze_pair(pair: Pair<Rule>, symbol_table: &mut SymbolTable, file_path: &str) {
    let pairs = pair.into_inner();
    for a_pair in pairs {
        let node = a_pair.into_inner().next().unwrap();
        let node_name = node.as_str().to_string();

        if let Some(symbol) = symbol_table.get_symbol(&node_name, symbol_table.get_current_scope())
        {
            println!(
                "SEM_ERR: `{}` already exists in scope at `{}:{}:{}`",
                node_name, file_path, symbol.location.0, symbol.location.1
            );
            return;
        }

        let symbol = Symbol {
            name: node_name.clone(),
            symbol_type: SymbolType::Other,
            location: node.line_col(),
        };

        println!("{:?}", symbol_table.get_current_scope());

        symbol_table.insert_symbol(symbol);
    }
}

fn analyze_pair_identifier(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    flags: &HashMap<FlagType, bool>,
    symbol_type: SymbolType,
    file_path: &str,
) {
    let node_name = pair.as_str().to_string();

    // if let Some(symbol) = symbol_table.get_symbol(&node_name, symbol_table.get_current_scope()) {
    //     if flags.values().any(|x| x == &true) {
    //         return;
    //     } else {
    //         println!(
    //             "SEM_ERR: `{}` already exists in scope at `{}:{}:{}`",
    //             node_name, file_path, symbol.location.0, symbol.location.1
    //         );
    //         return;
    //     }
    // }

    // println!("flags: {:?}", &flags);
    // println!("symbol_type: {:?}", symbol_type);
    // println!("node_name: {:?}\n", node_name);

    let is_one_of_flags = flags.values().any(|x| x == &true);
    let in_current_scope = symbol_table.get_symbol(&node_name, symbol_table.get_current_scope());
    let in_parent_scope = symbol_table.get_symbol_recursive(
        &node_name,
        &symbol_table
            .scopes
            .get(&symbol_table.current_scope)
            .unwrap()
            .parent,
    );

    // println!("in_current_scope: {:?}", in_current_scope);
    // println!("in_parent_scope: {:?}\n", in_parent_scope);

    if is_one_of_flags {
        if in_current_scope.is_none() && in_parent_scope.is_none() {
            println!(
                "SEM_ERR: `{}` does not exist in any scope at `{}:{}:{}`",
                node_name,
                file_path,
                &pair.line_col().0,
                &pair.line_col().1
            );
            return;
        }
    } else {
        if in_current_scope.is_some() {
            println!(
                "SEM_ERR: `{}` already exists in scope at `{}:{}:{}`",
                node_name,
                file_path,
                in_current_scope.unwrap().location.0,
                in_current_scope.unwrap().location.1
            );
            return;
        } else {
            // println!("symbol_type: {:?}", symbol_type);
            let symbol = Symbol {
                name: node_name.clone(),
                symbol_type: symbol_type,
                location: pair.line_col(),
            };
            // println!("{:?}", symbol_table.get_current_scope());
            symbol_table.insert_symbol(symbol);
        }
    }
}

pub fn analyze(program: Pairs<Rule>, file_path: &str) {
    loop_analyze(program, file_path);
}

fn loop_analyze(program: Pairs<Rule>, file_path: &str) {
    let mut symbol_table = SymbolTable {
        scopes: HashMap::new(),
        current_scope: String::from("global"),
    };

    // insert global scope
    let global_scope = Scope {
        name: String::from("global"),
        parent: String::from(""),
        symbols: HashMap::new(),
    };

    symbol_table
        .scopes
        .insert(String::from("global"), global_scope);

    let mut flatten_pairs = program.flatten();

    let mut flags = HashMap::new();
    flags.insert(FlagType::Expression, false);
    flags.insert(FlagType::CallStmt, false);
    // flags.insert(FlagType::FlowControl, false);
    flags.insert(FlagType::For, false);

    let mut flow_control_count = 0;
    let mut last_symbol_type = SymbolType::Other;

    while let Some(pair) = flatten_pairs.next() {
        // println!("{:?}\t{:?}", pair.as_rule(), pair.as_str());
        // println!("current_scope: {:?}", symbol_table.current_scope);
        // println!("flags: {:?}", &flags);
        // println!("symbol_type: {:?}", last_symbol_type);

        match pair.as_rule() {
            Rule::EOI => {
                // return;
            }

            Rule::SCOPE_START => {
                // println!("scope_start: {:?}", pair.as_str());
                // if *flags.get(&FlagType::FlowControl).unwrap_or(&false) {
                //     let flow_control_scope_name = format!(
                //         "{}_fc_{}",
                //         symbol_table.get_current_scope(),
                //         flow_control_count
                //     );
                //     flow_control_count += 1;

                //     let flow_control_scope = Scope {
                //         name: flow_control_scope_name.clone(),
                //         parent: symbol_table.current_scope.clone(),
                //         symbols: HashMap::new(),
                //     };

                //     symbol_table.insert_scope(flow_control_scope);
                //     symbol_table.current_scope = flow_control_scope_name;
                // } else {
                //     // let scope_name = flatten_pairs.next().unwrap().as_str().to_string();
                //     // let scope = Scope {
                //     //     name: scope_name.clone(),
                //     //     parent: symbol_table.current_scope.clone(),
                //     //     symbols: HashMap::new(),
                //     // };

                //     // symbol_table.insert_scope(scope);
                //     // symbol_table.current_scope = scope_name.clone();
                // }
            }
            Rule::SCOPE_END => {
                symbol_table.current_scope = symbol_table
                    .scopes
                    .get(&symbol_table.current_scope)
                    .unwrap()
                    .parent
                    .clone();
            }
            Rule::STATEMENT_END => {
                flags.insert(FlagType::Expression, false);
                flags.insert(FlagType::CallStmt, false);
            }
            Rule::FLOW_CONTROL_START => {
                // flags.insert(FlagType::FlowControl, true);
                flags.insert(FlagType::Expression, false);
                flags.insert(FlagType::CallStmt, false);

                if *flags.get(&FlagType::For).unwrap_or(&false) {
                    flags.insert(FlagType::For, false);
                    continue;
                }

                // create scope for flow control
                // insert flow control scope in symbol table

                let flow_control_scope_name = format!(
                    "{}_fc_{}",
                    symbol_table.get_current_scope(),
                    flow_control_count
                );
                flow_control_count += 1;

                let flow_control_scope = Scope {
                    name: flow_control_scope_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                symbol_table.insert_scope(flow_control_scope);
                symbol_table.current_scope = flow_control_scope_name;
            }
            Rule::FLOW_CONTROL_END => {
                // flags.insert(FlagType::FlowControl, false);

                // go back to parent scope
                symbol_table.current_scope = symbol_table
                    .scopes
                    .get(&symbol_table.current_scope)
                    .unwrap()
                    .parent
                    .clone();
            }

            // Rule::WHITESPACE => todo!(),
            // Rule::COMMENT => todo!(),
            // Rule::SingleLineComment => todo!(),
            // Rule::MultiLineComment => todo!(),

            // Rule::IDENT_CHARS => todo!(),
            Rule::IDENTIFIER => {
                // println!("IDENTIFIER: {:?}", pair);
                // println!("IDENTIFIER: {:?}", pair.as_str());
                analyze_pair_identifier(
                    pair,
                    &mut symbol_table,
                    &flags,
                    last_symbol_type.clone(),
                    file_path,
                );
            }
            Rule::STRUCT_ENUM_IDENTIFIER => {
                // println!("STRUCT_ENUM_IDENTIFIER: {:?}", pair);
            }
            // Rule::KEYWORD => todo!(),

            // Rule::a_fx => todo!(),
            // Rule::a_tilde => todo!(),
            // Rule::a_let => {
            //     // let IDENTIFIER = EXPRESSION;
            //     // let mut IDENTIFIER = EXPRESSION;
            //     // let IDENTIFIER type = EXPRESSION;
            //     // let mut IDENTIFIER type = EXPRESSION;
            //     // let IDENTIFIER = EXPRESSION;
            //     // let mut IDENTIFIER = EXPRESSION;
            //     // let IDENTIFIER type = EXPRESSION;
            //     // let mut IDENTIFIER type = EXPRESSION;
            //     // We will identify the use of the
            //     // mut_or_ident = pair.clone().into_inner().next().unwrap().as_str();
            //     // if ()
            // }
            // Rule::a_mut => todo!(),
            // Rule::a_import => todo!(),
            // Rule::a_pub => todo!(),
            // Rule::a_mod => todo!(),
            // Rule::a_super => todo!(),
            // Rule::a_self => todo!(),
            // Rule::a_if => todo!(),
            // Rule::a_else => todo!(),
            // Rule::a_while => todo!(),
            // Rule::a_for => todo!(),
            // Rule::a_in => todo!(),
            // Rule::a_continue => todo!(),
            // Rule::a_break => todo!(),
            // Rule::a_match => todo!(),
            // Rule::a_fat_arrow => todo!(),
            // Rule::a_return => todo!(),
            // Rule::a_result => todo!(),
            // Rule::a_ok => todo!(),
            // Rule::a_err => todo!(),
            // Rule::a_type => todo!(),
            // Rule::a_as => todo!(),
            // Rule::a_struct => todo!(),
            // Rule::a_enum => todo!(),
            // Rule::a_impl => todo!(),

            // Rule::INTEGER => todo!(),
            // Rule::FLOAT => todo!(),
            // Rule::BOOL => todo!(),
            // Rule::STRING => todo!(),
            // Rule::CHAR => todo!(),
            // Rule::TUPLE => todo!(),
            // Rule::ARRAY => todo!(),
            // Rule::LIST => todo!(),

            // Rule::DATA_TYPES => todo!(),
            // Rule::TYPE => todo!(),
            // Rule::TUPLE_TYPE => todo!(),
            // Rule::ARRAY_TYPE => todo!(),
            // Rule::LIST_TYPE => todo!(),
            // Rule::RESULT_TYPE => todo!(),

            // Rule::OPERATOR => todo!(),
            // Rule::OPERATOR_LEVEL_1 => todo!(),
            // Rule::OPERATOR_LEVEL_2 => todo!(),
            // Rule::OPERATOR_LEVEL_3 => todo!(),
            // Rule::OPERATOR_LEVEL_4 => todo!(),
            // Rule::OPERATOR_LEVEL_5 => todo!(),
            // Rule::OPERATOR_LEVEL_6 => todo!(),
            // Rule::OPERATOR_LEVEL_7 => todo!(),
            // Rule::OPERATOR_LEVEL_8 => todo!(),
            // Rule::OPERATOR_LEVEL_9 => todo!(),
            // Rule::OPERATOR_LEVEL_10 => todo!(),

            // Rule::multiply => todo!(),
            // Rule::divide => todo!(),
            // Rule::remainder => todo!(),
            // Rule::add => todo!(),
            // Rule::subtract => todo!(),
            // Rule::unary_plus => todo!(),
            // Rule::unary_minus => todo!(),
            // Rule::logical_and => todo!(),
            // Rule::logical_or => todo!(),
            // Rule::logical_not => todo!(),
            // Rule::comparison => todo!(),
            // Rule::equal => todo!(),
            // Rule::not_equal => todo!(),
            // Rule::greater_than_equal => todo!(),
            // Rule::less_than_equal => todo!(),
            // Rule::greater_than => todo!(),
            // Rule::less_than => todo!(),
            // Rule::ASSIGNMENT => todo!(),
            // Rule::assign => todo!(),
            // Rule::plus_assign => todo!(),
            // Rule::minus_assign => todo!(),
            // Rule::multiply_assign => todo!(),
            // Rule::divide_assign => todo!(),
            // Rule::remainder_assign => todo!(),
            // Rule::bitwise_and_assign => todo!(),
            // Rule::bitwise_or_assign => todo!(),
            // Rule::bitwise_xor_assign => todo!(),
            // Rule::bitwise_left_shift_assign => todo!(),
            // Rule::bitwise_right_shift_assign => todo!(),
            // Rule::bitwise_shift => todo!(),
            // Rule::bitwise_not => todo!(),
            // Rule::bitwise_and => todo!(),
            // Rule::bitwise_xor => todo!(),
            // Rule::bitwise_or => todo!(),
            // Rule::left_shift => todo!(),
            // Rule::right_shift => todo!(),

            // Rule::STATEMENT => {
            //     // DECL_STMT = { MUT | IMMUT | VARIABLE_REASS };
            // }
            Rule::MUT => {
                last_symbol_type = SymbolType::Mut;
            }
            Rule::IMMUT => {
                last_symbol_type = SymbolType::Immut;
            }
            Rule::VARIABLE_REASS => {
                // last_symbol_type = SymbolType::Other;
                // check if variable is already in symbol table of current scope
                let variable_name = flatten_pairs.next().unwrap().as_str().to_string();
                let in_current_scope =
                    symbol_table.get_symbol(&variable_name, symbol_table.get_current_scope());
                let in_parent_scope = symbol_table.get_symbol_recursive(
                    &variable_name,
                    &symbol_table
                        .scopes
                        .get(&symbol_table.current_scope)
                        .unwrap()
                        .parent,
                );

                let line_col = pair.line_col();
                if in_current_scope.is_none() && in_parent_scope.is_none() {
                    println!(
                        "SEM_ERR: Variable `{}` does not exist in any scope at `{}:{}:{}`",
                        variable_name, file_path, line_col.0, line_col.1
                    );
                    // return;
                }
            }

            Rule::CALL_STMT => {
                flags.insert(FlagType::CallStmt, true);
            }

            Rule::BREAK_STMT => {
                // check if break is inside a loop
                let mut current_scope = symbol_table.get_current_scope();
                loop {
                    let scope = symbol_table.get_scope(current_scope).unwrap();
                    if scope.name == "global" {
                        println!(
                            "SEM_ERR: `break` statement not inside a loop at `{}:{}`",
                            file_path,
                            pair.line_col().0
                        );
                        break;
                    }
                    current_scope = &scope.parent;
                }
            }
            // Rule::CONTINUE_STMT => todo!(),
            Rule::FLOW_CONTROL => {
                // flags.insert(FlagType::FlowControl, true);

                // FLOW_CONTROL = { IF_STATEMENT | MATCH_STATEMENT | FOR_LOOP | WHILE_LOOP }
                // flow_control_scope_name = parent_scope_name + "_flow_control_" + flow_control_count
                // flow_control_count += 1

                // let flow_control_scope_name = format!(
                //     "{}_flow_control_{}",
                //     symbol_table.get_current_scope(),
                //     flow_control_count
                // );
                // flow_control_count += 1;

                // let flow_control_scope = Scope {
                //     name: flow_control_scope_name.clone(),
                //     parent: symbol_table.current_scope.clone(),
                //     symbols: HashMap::new(),
                // };

                // symbol_table.insert_scope(flow_control_scope);
                // symbol_table.current_scope = flow_control_scope_name;
            }
            // Rule::IF_STATEMENT => todo!(),
            // Rule::ELSE_IF_STATEMENT => todo!(),
            // Rule::ELSE_BLOCK => todo!(),
            // Rule::EXP_BLOCK => todo!(),
            // Rule::BLOCK => todo!(),
            Rule::MATCH_STATEMENT => {
                // let flow_control_scope_name = format!(
                //     "{}_fc_{}",
                //     symbol_table.get_current_scope(),
                //     flow_control_count
                // );
                // flow_control_count += 1;

                // let flow_control_scope = Scope {
                //     name: flow_control_scope_name.clone(),
                //     parent: symbol_table.current_scope.clone(),
                //     symbols: HashMap::new(),
                // };

                // symbol_table.insert_scope(flow_control_scope);
                // symbol_table.current_scope = flow_control_scope_name;
            }
            // Rule::MATCH_CASE => todo!(),
            // Rule::MATCH_DEFAULT => todo!(),
            Rule::FOR_LOOP => {
                flags.insert(FlagType::For, true);

                let flow_control_scope_name = format!(
                    "{}_fc_{}",
                    symbol_table.get_current_scope(),
                    flow_control_count
                );
                flow_control_count += 1;

                let flow_control_scope = Scope {
                    name: flow_control_scope_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                symbol_table.insert_scope(flow_control_scope);
                symbol_table.current_scope = flow_control_scope_name;

                let for_identifier = flatten_pairs.next().unwrap().as_str().to_string();
                // insert for_identifier in flow control scope
                let for_symbol = Symbol {
                    name: for_identifier.clone(),
                    symbol_type: SymbolType::Mut,
                    location: pair.line_col(),
                };

                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(for_identifier.clone(), for_symbol);
            }
            // Rule::RANGE => todo!(),

            // Rule::WHILE_LOOP => todo!(),
            Rule::EXPRESSION => {
                flags.insert(FlagType::Expression, true);
            }
            Rule::FACTOR => {}

            Rule::TUPLE_ACCESS => {
                last_symbol_type = SymbolType::TupAccess;
            }
            Rule::ARRAY_ACCESS => {
                last_symbol_type = SymbolType::ArrAccess;
            }
            Rule::LIST_ACCESS => {
                last_symbol_type = SymbolType::ListAccess;
            }
            Rule::IMPL_ACCESS => {
                last_symbol_type = SymbolType::ImplAccess;
            }
            Rule::STRUCT_ACCESS => {
                last_symbol_type = SymbolType::StructAccess;
            }
            Rule::ENUM_ACCESS => {
                last_symbol_type = SymbolType::EnumAccess;
                // store_identifier = true;
                // println!("STRUCT_ENUM_ACCESS: {:?}", pair);
                let str = pair.as_str().split("->").collect::<Vec<&str>>();
                let struct_enum_name = str[0].to_string();
                let struct_enum_field = str[1].to_string();

                let get_symbol =
                    symbol_table.get_symbol(&struct_enum_name, &String::from("global"));

                if let Some(symbol) = get_symbol {
                    let get_field = symbol_table.get_symbol(&struct_enum_field, &struct_enum_name);
                    if get_field.is_none() {
                        println!(
                            "SEM_ERR: Field `{}` does not exist in `{}` at `{}:{}`",
                            struct_enum_field,
                            struct_enum_name,
                            file_path,
                            pair.line_col().0
                        );
                    }
                } else {
                    if struct_enum_name == "self" {
                        // println!("self");
                    } else {
                        println!(
                            "SEM_ERR: Struct/Enum `{}` does not exist in any scope at `{}:{}`",
                            struct_enum_name,
                            file_path,
                            pair.line_col().0
                        );
                    }
                }

                let struct_enum_access_pair_count = pair.clone().into_inner().flatten().count();

                // skip flatten_pairs for the number of pairs in struct_enum_access
                for _ in 0..struct_enum_access_pair_count {
                    flatten_pairs.next();
                }
            }

            Rule::FUNCTION_DECL => {
                // println!("{:?}", pair.as_rule());
                let function_name = flatten_pairs.next().unwrap().as_str().to_string();

                // check if function is already in symbol table of current scope
                let already_exists = symbol_table
                    .scopes
                    .get(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .get(&function_name);

                if let Some(symbol) = already_exists {
                    println!(
                        "SEM_ERR: Function `{}` already exists in scope at `{}:{}:{}`",
                        function_name, file_path, symbol.location.0, symbol.location.1
                    );
                    return;
                }

                let function_scope = Scope {
                    name: function_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                let function_symbol = Symbol {
                    name: function_name.clone(),
                    symbol_type: SymbolType::Function,
                    location: pair.line_col(),
                };

                let mut key = function_name.clone();
                if last_symbol_type == SymbolType::Impl {
                    key = format!("{}::{}", &symbol_table.get_current_scope(), function_name);
                }

                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(key.clone(), function_symbol);

                symbol_table.scopes.insert(key.clone(), function_scope);

                symbol_table.current_scope = key;

                last_symbol_type = SymbolType::Function;

                // // go back to parent scope
                // symbol_table.current_scope = symbol_table
                //     .scopes
                //     .get(&function_name)
                //     .unwrap()
                //     .parent
                //     .clone();
            }
            // Rule::PARAMETER_LIST => todo!(),
            Rule::PARAMETER => {
                last_symbol_type = SymbolType::Parameter;
            }
            // Rule::RETURN_TYPE => todo!(),
            // Rule::RETURN_STMT => todo!(),

            // Rule::FUNCTION_BODY => todo!(),
            Rule::FUNCTION_CALL => {
                last_symbol_type = SymbolType::FnCall;
            }
            // Rule::ARGUMENTS_LIST => todo!(),
            // Rule::ARGUMENT => todo!(),

            // Rule::ERROR_CHECK => todo!(),

            // Rule::CLOSURE => todo!(),

            // Rule::MODULE_DECL => todo!(),
            // Rule::MODULE_BLOCK => todo!(),
            Rule::MODULE_ACCESS => {
                // last_symbol_type = SymbolType::ModAccess;
            }

            // Rule::IMPORT_DECL => todo!(),
            // Rule::IMPORT_IDENTIFIER => todo!(),
            Rule::STRUCT_DECL => {
                last_symbol_type = SymbolType::Struct;

                let struct_name = flatten_pairs.next().unwrap().as_str().to_string();
                let get_symbol =
                    symbol_table.get_symbol(&struct_name, symbol_table.get_current_scope());

                if let Some(symbol) = get_symbol {
                    println!(
                        "SEM_ERR: Struct `{}` already exists in scope at `{}:{}:{}`",
                        struct_name, file_path, symbol.location.0, symbol.location.1
                    );
                    // return;
                }

                let struct_scope = Scope {
                    name: struct_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                let struct_symbol = Symbol {
                    name: struct_name.clone(),
                    symbol_type: SymbolType::Struct,
                    location: pair.line_col(),
                };

                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(struct_name.clone(), struct_symbol);

                symbol_table
                    .scopes
                    .insert(struct_name.clone(), struct_scope);

                symbol_table.current_scope = struct_name.clone();
            }
            // Rule::STRUCT_FIELD => todo!(),
            Rule::ENUM_DECL => {
                last_symbol_type = SymbolType::Enum;

                let enum_name = flatten_pairs.next().unwrap().as_str().to_string();
                let get_symbol =
                    symbol_table.get_symbol(&enum_name, symbol_table.get_current_scope());

                if let Some(symbol) = get_symbol {
                    println!(
                        "SEM_ERR: Enum `{}` already exists in scope at `{}:{}:{}`",
                        enum_name, file_path, symbol.location.0, symbol.location.1
                    );
                    // return;
                }

                let enum_scope = Scope {
                    name: enum_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                let enum_symbol = Symbol {
                    name: enum_name.clone(),
                    symbol_type: SymbolType::Enum,
                    location: pair.line_col(),
                };

                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(enum_name.clone(), enum_symbol);

                symbol_table.scopes.insert(enum_name.clone(), enum_scope);

                symbol_table.current_scope = enum_name.clone();
            }
            // Rule::ENUM_VARIANT => todo!(),
            Rule::IMPL_DECL => {
                last_symbol_type = SymbolType::Impl;

                let impl_name = flatten_pairs.next().unwrap().as_str().to_string();
                let get_symbol =
                    symbol_table.get_symbol(&impl_name, symbol_table.get_current_scope());

                let is_builtin_data_type =
                    BUILTIN_DATA_TYPES.iter().any(|&x| x == impl_name.as_str());

                let mut is_impl_able = false;
                if is_builtin_data_type {
                    is_impl_able = true;
                }
                if let Some(symbol) = get_symbol {
                    is_impl_able = true;
                }

                if !is_impl_able {
                    println!(
                        "SEM_ERR: Impl `{}` does not exist in any scope at `{}:{}:{}`",
                        impl_name,
                        file_path,
                        pair.line_col().0,
                        pair.line_col().1
                    )
                }

                let impl_scope = Scope {
                    name: impl_name.clone(),
                    parent: symbol_table.current_scope.clone(),
                    symbols: HashMap::new(),
                };

                let impl_symbol = Symbol {
                    name: impl_name.clone(),
                    symbol_type: SymbolType::Impl,
                    location: pair.line_col(),
                };

                let key = format!("impl::{}", impl_name);
                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(key.clone(), impl_symbol);

                symbol_table.scopes.insert(key.clone(), impl_scope);

                symbol_table.current_scope = key.clone();
            }

            // Rule::PROGRAM_BLOCK => todo!(),
            // Rule::PROGRAM => todo!(),
            _ => {
                // println!("hmm aa baaki chhe: {:?}", pair.as_rule());
            }
        }
    }

    println!("\n\n>>> Symbol Table:");
    println!("{:#?}", symbol_table);
}
