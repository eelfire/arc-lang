use crate::pair_to_tree::Node;
use crate::pair_to_tree::Type;
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
    Expression,
    Other,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub location: (usize, usize),
    pub type_: Option<Type>,
    pub index: Option<usize>,
    // used: bool,
    // other fields...
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub parent: String,
    pub symbols: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: HashMap<String, Scope>,
    pub current_scope: String,
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

fn analyze_pair_identifier(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    flags: &HashMap<FlagType, bool>,
    symbol_type: SymbolType,
    file_path: &str,
    index: &mut usize,
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
            let s_index = match symbol_type {
                SymbolType::Parameter | SymbolType::Immut | SymbolType::Mut => {
                    *index += 1;
                    Some(*index - 1)
                }
                _ => None,
            };
            let symbol = Symbol {
                name: node_name.clone(),
                symbol_type: symbol_type,
                location: pair.line_col(),
                type_: None,
                index: s_index,
            };
            // println!("{:?}", symbol_table.get_current_scope());
            symbol_table.insert_symbol(symbol);
        }
    }
}

fn evaluate_type(pair: Pair<Rule>) -> Option<Type> {
    let pairs = pair.into_inner().flatten();

    let type_ = Some(Type::Any);
    for pair in pairs {
        match pair.as_rule() {
            Rule::I32 => return Some(Type::I32),
            Rule::I64 => return Some(Type::I64),
            Rule::F32 => return Some(Type::F32),
            Rule::F64 => return Some(Type::F64),
            Rule::BOOL_TYPE => return Some(Type::Bool),
            Rule::CHAR_TYPE => return Some(Type::Char),
            Rule::STRING_TYPE => return Some(Type::String),
            Rule::ARRAY_TYPE => {
                // ARRAY_TYPE: "[i32;2]"
                //     TYPE: "i32"
                //         I32: "i32"
                //     INTEGER: "2"
                // Some(Type::Array(Box::new(Type::I32), 2))

                let mut array_type_pairs = pair.into_inner().flatten();
                let array_type_ = evaluate_type(array_type_pairs.next().unwrap()).unwrap();
                let mut array_size = 0;
                for pair in array_type_pairs {
                    match pair.as_rule() {
                        Rule::INTEGER => {
                            array_size = pair.as_str().parse::<usize>().unwrap();
                        }
                        _ => {}
                    };
                }
                return Some(Type::Array(Box::new(array_type_), array_size));
            }
            Rule::TUPLE_TYPE => {
                // TUPLE_TYPE: "(i32, bool)"
                //     TYPE: "i32"
                //         I32: "i32"
                //     TYPE: "bool"
                //         BOOL_TYPE: "bool"
                // Some(Type::Tuple(vec![Type::I32, Type::Bool]))

                let mut tuple_type_pairs = pair.into_inner().flatten();
                let mut tuple_types = vec![];
                for tuple_type_pair in tuple_type_pairs {
                    match tuple_type_pair.as_rule() {
                        Rule::TYPE => tuple_types.push(evaluate_type(tuple_type_pair).unwrap()),
                        _ => {}
                    }
                }
                return Some(Type::Tuple(tuple_types));
            }
            Rule::LIST_TYPE => {
                // LIST_TYPE: "<i64>"
                //     TYPE: "i64"
                //         I64: "i64"
                // Some(Type::List(Box::new(Type::I64)))

                let mut list_type_pairs = pair.into_inner().flatten();
                let list_type_ = evaluate_type(list_type_pairs.next().unwrap()).unwrap();
                return Some(Type::List(Box::new(list_type_)));
            }
            // Rule::RESULT_TYPE => {}
            Rule::RETURN_TYPE => {
                // RETURN_TYPE: "~ i32"
                //     TYPE: "i32"
                //         I32: "i32"
                // Some(Type::I32)

                let mut return_type_pairs = pair.into_inner().flatten();
                let return_type_ = evaluate_type(return_type_pairs.next().unwrap()).unwrap();
                return Some(return_type_);
            }
            _ => {}
        };
    }
    type_
}

fn get_type_from_symbol_table(symbol_table: &SymbolTable, symbol_name: &String) -> Option<Type> {
    let symbol = symbol_table.get_symbol(symbol_name, &symbol_table.get_current_scope());
    if let Some(symbol) = symbol {
        return symbol.type_.clone();
    }
    None
}

fn evaluate_expression_type(pair: Pair<Rule>, symbol_table: &SymbolTable, file_path: &str) -> Type {
    // 4
    // 4 + 5
    // 4 / 5
    // 4 * 5
    // 4 % 5
    // 4 - 5
    // true
    // false
    // 'a'
    // "hello"
    // (4)
    // (4 + 5)
    // (4 / 5)
    // a + (b * 2)
    // add(4, 5)
    // add(4, 5) + 2
    // dbg!(&pair);
    let line_col = pair.line_col();
    let mut pairs = pair.into_inner().flatten();
    let mut all_the_types_in_expression = vec![];
    let mut type_ = Type::Any;
    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::INTEGER => all_the_types_in_expression.push(Type::I32),
            Rule::FLOAT => all_the_types_in_expression.push(Type::F32),
            Rule::BOOL => all_the_types_in_expression.push(Type::Bool),
            Rule::CHAR => all_the_types_in_expression.push(Type::Char),
            Rule::STRING => all_the_types_in_expression.push(Type::String),
            Rule::IDENTIFIER => {
                // check if identifier exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the identifier
                let identifier = pair.as_str().to_string();
                let symbol_type = get_type_from_symbol_table(&symbol_table, &identifier);
                if let Some(symbol_type) = symbol_type {
                    all_the_types_in_expression.push(symbol_type);
                } else {
                    all_the_types_in_expression.push(Type::Any);
                }
            }
            Rule::ARRAY => {
                // let arr = [1, 2, 3];
                let mut array_pairs = pair.clone().into_inner().flatten();
                let array_pairs_count = array_pairs.clone().count();
                let mut array_length = 0;
                while let Some(inner_array_pair) = array_pairs.next() {
                    match inner_array_pair.as_rule() {
                        Rule::INNER_EXPRESSION => array_length += 1,
                        _ => {}
                    }
                }
                // let array_pairs_count = array_pairs.clone().count();
                let array_elements_types = evaluate_expression_type(pair, symbol_table, file_path);
                all_the_types_in_expression
                    .push(Type::Array(Box::new(array_elements_types), array_length));

                // skip flatten_pairs for the number of pairs in array
                for _ in 0..array_pairs_count {
                    pairs.next();
                }
            }
            Rule::LIST => {
                // let list = <1, 2, 3>;
                let mut list_pairs = pair.clone().into_inner().flatten();
                let list_pairs_count = list_pairs.clone().count();
                let mut list_length = 0;
                while let Some(inner_list_pair) = list_pairs.next() {
                    match inner_list_pair.as_rule() {
                        Rule::INNER_EXPRESSION => list_length += 1,
                        _ => {}
                    }
                }
                // let list_pairs_count = list_pairs.clone().count();
                let list_elements_types = evaluate_expression_type(pair, symbol_table, file_path);
                all_the_types_in_expression.push(Type::List(Box::new(list_elements_types)));

                // skip flatten_pairs for the number of pairs in list
                for _ in 0..list_pairs_count {
                    pairs.next();
                }
            }
            Rule::RETURN_TYPE => {
                // check if return type exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the return type
                let return_type = pair.as_str().to_string();
                let return_type_symbol =
                    symbol_table.get_symbol(&return_type, &symbol_table.get_current_scope());
                if let Some(return_type_symbol) = return_type_symbol {
                    all_the_types_in_expression.push(return_type_symbol.type_.clone().unwrap());
                }
            }
            Rule::FUNCTION_CALL => {
                // check if function exists in symbol table
                // if not, return Type::Any
                // if exists, return the return type of the function
                let function_name = pair.as_str().to_string();
                let function_symbol =
                    symbol_table.get_symbol(&function_name, &symbol_table.get_current_scope());
                if let Some(function_symbol) = function_symbol {
                    if function_symbol.symbol_type == SymbolType::Function {
                        all_the_types_in_expression.push(function_symbol.type_.clone().unwrap());
                    }
                }
            }
            Rule::STRUCT_ACCESS => {
                // check if struct exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the struct
                let struct_name = pair.as_str().to_string();
                let struct_symbol =
                    symbol_table.get_symbol(&struct_name, &symbol_table.get_current_scope());
                if let Some(struct_symbol) = struct_symbol {
                    if struct_symbol.symbol_type == SymbolType::Struct {
                        all_the_types_in_expression.push(struct_symbol.type_.clone().unwrap());
                    }
                }
            }
            Rule::ENUM_ACCESS => {
                // check if enum exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the enum
                let enum_name = pair.as_str().to_string();
                let enum_symbol =
                    symbol_table.get_symbol(&enum_name, &symbol_table.get_current_scope());
                if let Some(enum_symbol) = enum_symbol {
                    if enum_symbol.symbol_type == SymbolType::Enum {
                        all_the_types_in_expression.push(enum_symbol.type_.clone().unwrap());
                    }
                }
            }
            Rule::TUPLE_ACCESS => {
                // check if tuple exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the tuple
                let tuple_name = pair.as_str().to_string();
                let tuple_symbol =
                    symbol_table.get_symbol(&tuple_name, &symbol_table.get_current_scope());
                if let Some(tuple_symbol) = tuple_symbol {
                    if tuple_symbol.symbol_type == SymbolType::TupAccess {
                        all_the_types_in_expression.push(tuple_symbol.type_.clone().unwrap());
                    }
                }
            }
            Rule::ARRAY_ACCESS => {
                // check if array exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the array
                let array_access_pair = pair.clone().into_inner().flatten();
                let array_access_pairs_count = array_access_pair.clone().count();
                let array_name = pair.as_str().to_string();
                let array_symbol =
                    symbol_table.get_symbol(&array_name, &symbol_table.get_current_scope());
                if let Some(array_symbol) = array_symbol {
                    if array_symbol.symbol_type == SymbolType::ArrAccess {
                        all_the_types_in_expression.push(array_symbol.type_.clone().unwrap());
                    }
                }

                // skip flatten_pairs for the number of pairs in array
                // for _ in 0..array_access_pairs_count {
                //     pairs.next();
                // }
            }
            Rule::LIST_ACCESS => {
                // check if list exists in symbol table
                // if not, return Type::Any
                // if exists, return the type of the list
                let list_access_pair = pair.clone().into_inner().flatten();
                let list_access_pairs_count = list_access_pair.clone().count();
                let list_name = pair.as_str().to_string();
                let list_symbol =
                    symbol_table.get_symbol(&list_name, &symbol_table.get_current_scope());
                if let Some(list_symbol) = list_symbol {
                    if list_symbol.symbol_type == SymbolType::ListAccess {
                        all_the_types_in_expression.push(list_symbol.type_.clone().unwrap());
                    }
                }
            }
            _ => {}
        }
    }
    // dbg!(&all_the_types_in_expression);
    if all_the_types_in_expression.len() == 1 {
        type_ = all_the_types_in_expression[0].clone();
    } else {
        // check if all the types in the expression are the same
        let mut same_types = true;
        for i in 1..all_the_types_in_expression.len() {
            if all_the_types_in_expression[i] != all_the_types_in_expression[0] {
                same_types = false;
                break;
            }
        }
        if same_types {
            type_ = all_the_types_in_expression[0].clone();
        } else {
            // mismatch type error reporting
            println!(
                "SEM_ERR: Mismatched types in expression at `{}:{}:{}`",
                file_path, line_col.0, line_col.1
            );
        }
    }
    type_
}

pub fn analyze(program: Pairs<Rule>, tree: &mut Vec<Node>, file_path: &str) -> SymbolTable {
    let symbol_table = loop_analyze(program, tree, file_path);
    symbol_table
}

fn loop_analyze(program: Pairs<Rule>, tree: &mut Vec<Node>, file_path: &str) -> SymbolTable {
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
    let mut last_symbol_name = String::new();
    let mut iter = 0;
    let mut parameters_list = vec![];
    let mut return_type = Some(Type::Any);
    let mut index = 0;

    while let Some(pair) = flatten_pairs.next() {
        // while let Some((i, pair)) = flatten_pairs.enumerate().next() {
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

                index = 0;
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

                index = 0;
            }

            // Rule::WHITESPACE => todo!(),
            // Rule::COMMENT => todo!(),
            // Rule::SingleLineComment => todo!(),
            // Rule::MultiLineComment => todo!(),

            // Rule::IDENT_CHARS => todo!(),
            Rule::IDENTIFIER => {
                // println!("IDENTIFIER: {:?}", pair);
                // println!("IDENTIFIER: {:?}", pair.as_str());
                last_symbol_name = pair.clone().as_str().to_string();
                analyze_pair_identifier(
                    pair,
                    &mut symbol_table,
                    &flags,
                    last_symbol_type.clone(),
                    file_path,
                    &mut index,
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
            Rule::ANY_TYPE => {
                // modify the type_ of last inserted symbol in symbol table for the current scope
                let current_scope = symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap();
                let last_symbol = current_scope.symbols.get_mut(&last_symbol_name).unwrap();
                last_symbol.type_ = evaluate_type(pair);
                tree[iter - 1].type_ = last_symbol.type_.clone();

                if last_symbol_type == SymbolType::Parameter {
                    parameters_list.push(last_symbol.type_.clone().unwrap());
                }
            }
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
                iter += 1;
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
                iter += 1;
                // insert for_identifier in flow control scope
                let for_symbol = Symbol {
                    name: for_identifier.clone(),
                    symbol_type: SymbolType::Mut,
                    location: pair.line_col(),
                    type_: None,
                    index: Some(index),
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
                let expression_type = evaluate_expression_type(pair, &symbol_table, file_path);
                // println!("<><><><> expression_type: {:?}", expression_type);

                // store the type in the tree
                // dbg!(&tree[iter]);
                tree[iter].type_ = Some(expression_type);
                // dbg!(&tree[i]);
            }
            Rule::FACTOR => {}

            Rule::RETURN_TYPE => {
                return_type = evaluate_type(pair);
                // println!("return_type: {:?}", return_type);
                // store the return type in the tree
                tree[iter].type_ = return_type.clone();
            }
            Rule::RETURN_STMT => {
                // check if return type matches the return type of the function
                // if not, return type mismatch error
                // if matches, continue
            }

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
                    iter += 1;
                }
            }

            Rule::FUNCTION_DECL => {
                // println!("{:?}", pair.as_rule());
                let function_name = flatten_pairs.next().unwrap().as_str().to_string();
                iter += 1;

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
                    // return;
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
                    type_: Some(Type::Function(vec![], Box::new(Type::Any))), // Type::Function(vec![]
                    index: None,
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
            // Rule::PARAMETER_LIST => {}
            Rule::PARAMETER => {
                last_symbol_type = SymbolType::Parameter;
            }
            // Rule::RETURN_TYPE => todo!(),
            // Rule::RETURN_STMT => todo!(),
            Rule::FUNCTION_BODY => {
                // update the type of the function in the symbol table with Function(parameters_list, return_type) in parent scope
                let parent_scope = symbol_table
                    .scopes
                    .get(&symbol_table.current_scope)
                    .unwrap()
                    .parent
                    .clone();

                let function_symbol = symbol_table
                    .scopes
                    .get_mut(&parent_scope)
                    .unwrap()
                    .symbols
                    .get_mut(&symbol_table.current_scope)
                    .unwrap();

                function_symbol.type_ = Some(Type::Function(
                    parameters_list.clone(),
                    Box::new(return_type.clone().unwrap()),
                ));
                parameters_list.clear();
                return_type = Some(Type::Any);

                // also update the type of the function in the tree
                let current_scope_name = symbol_table.current_scope.clone();
                let mut i = 1;
                while i < tree.len() - 1 {
                    if tree[i].text == current_scope_name && tree[i - 1].rule == Rule::FUNCTION_DECL
                    {
                        tree[i - 1].type_ = function_symbol.type_.clone();
                        break;
                    }
                    i += 1;
                }
            }
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
                iter += 1;
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
                    type_: Some(Type::Struct(vec![])),
                    index: None,
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
                iter += 1;
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
                    type_: Some(Type::Enum(vec![])),
                    index: None,
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
                iter += 1;
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
                    type_: None,
                    index: None,
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
        iter += 1;
    }

    println!("\n\n>>> Symbol Table:");
    println!("{:#?}", symbol_table);

    symbol_table
}
