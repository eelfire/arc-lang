use crate::parser::ArcParser;
use crate::parser::Rule;
use pest::iterators::Pairs;
use std::any::Any;
use std::collections::HashMap;

use crate::parser::print_nested_pairs;

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    symbol_type: String,
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

pub trait SymbolTableTrait {}

impl SymbolTableTrait for SymbolTable {}

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

    while let Some(pair) = flatten_pairs.next() {
        // println!("{:?}", pair.as_rule());
        match pair.as_rule() {
            Rule::EOI => {
                // return;
            }

            // Rule::WHITESPACE => todo!(),
            // Rule::COMMENT => todo!(),
            // Rule::SingleLineComment => todo!(),
            // Rule::MultiLineComment => todo!(),

            // Rule::IDENT_CHARS => todo!(),
            // Rule::IDENTIFIER => {
            //     // let mut IDENTIFER = EXPRESSION;
            //     // let IDENTIFIER type = EXPRESSION;
            //     // let IDENTIFIER = EXPRESSION;
            //     // let mut IDENTIFIER type = EXPRESSION;
            // }
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

            // Rule::BREAK_STMT => todo!(),
            // Rule::CONTINUE_STMT => todo!(),

            // Rule::IF_STATEMENT => todo!(),
            // Rule::ELSE_IF_STATEMENT => todo!(),
            // Rule::ELSE_BLOCK => todo!(),
            // Rule::EXP_BLOCK => todo!(),
            // Rule::BLOCK => todo!(),

            // Rule::MATCH_STATEMENT => todo!(),
            // Rule::MATCH_CASE => todo!(),
            // Rule::MATCH_DEFAULT => todo!(),

            // Rule::FOR_LOOP => todo!(),
            // Rule::RANGE => todo!(),

            // Rule::WHILE_LOOP => todo!(),

            // Rule::EXPRESSION => todo!(),
            // Rule::FACTOR => todo!(),

            // Rule::TUPLE_ACCESS => todo!(),
            // Rule::ARRAY_ACCESS => todo!(),
            // Rule::LIST_ACCESS => todo!(),
            // Rule::IMPL_ACCESS => todo!(),
            Rule::STRUCT_ENUM_ACCESS => todo!(),

            Rule::FUNCTION_DECL => {
                println!("{:?}", pair.as_rule());
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
                    symbol_type: String::from("function"),
                    location: pair.line_col(),
                };

                symbol_table
                    .scopes
                    .get_mut(&symbol_table.current_scope)
                    .unwrap()
                    .symbols
                    .insert(function_name.clone(), function_symbol);

                symbol_table
                    .scopes
                    .insert(function_name.clone(), function_scope);

                symbol_table.current_scope = function_name.clone();

                loop {
                    let next_pair = flatten_pairs.next().unwrap();
                    if next_pair.as_rule() == Rule::FUNCTION_BODY {
                        break;
                    }
                    // println!("\t{:?}", next_pair.as_rule());
                }

                // let function_body = flatten_pairs.next().unwrap();
                // function_body.into_inner().

                let function_pairs = pair.into_inner();
                // print_nested_pairs(&function_pairs, 0);

                for function_pair in function_pairs {
                    // parameter list if any
                    // return type if any
                    // function body

                    match function_pair.as_rule() {
                        Rule::PARAMETER_LIST => {
                            let parameter_list_pairs = function_pair.into_inner();
                            for parameter_pair in parameter_list_pairs {
                                let parameter = parameter_pair.into_inner().next().unwrap();
                                let parameter_name = parameter.as_str().to_string();

                                // check if parameter is already in symbol table of current scope
                                let already_exists = symbol_table
                                    .scopes
                                    .get(&function_name)
                                    .unwrap()
                                    .symbols
                                    .get(&parameter_name);

                                if let Some(symbol) = already_exists {
                                    println!(
                                        "SEM_ERR: Parameter `{}` already exists in scope at `{}:{}:{}`",
                                        parameter_name,
                                        file_path,
                                        symbol.location.0,
                                        symbol.location.1
                                    );
                                    return;
                                }

                                let parameter_symbol = Symbol {
                                    name: parameter_name.clone(),
                                    symbol_type: String::from("parameter"),
                                    location: parameter.line_col(),
                                };

                                symbol_table
                                    .scopes
                                    .get_mut(&function_name)
                                    .unwrap()
                                    .symbols
                                    .insert(parameter_name.clone(), parameter_symbol);
                            }
                        }
                        Rule::RETURN_TYPE => {
                            let return_type_pairs = function_pair.into_inner();
                            for return_type_pair in return_type_pairs {
                                println!("TODO: remember to check return type")
                            }
                        }
                        Rule::FUNCTION_BODY => {
                            let function_body_pairs = function_pair.into_inner();
                            for function_body_pair in function_body_pairs {
                                match function_body_pair.as_rule() {
                                    Rule::STATEMENT => {
                                        let statement_pairs = function_body_pair.into_inner();
                                        for statement_pair in statement_pairs {
                                            println!(
                                                "\t\tstmt: {:?} - {:?}",
                                                statement_pair.as_rule(),
                                                statement_pair.as_str()
                                            );
                                            // statement grammar rule:
                                            // STATEMENT = {
                                            //     DECL_STMT
                                            //   | CALL_STMT
                                            //   | BREAK_STMT
                                            //   | CONTINUE_STMT
                                            //   | IF_STATEMENT
                                            //   | MATCH_STATEMENT
                                            //   | FOR_LOOP
                                            //   | WHILE_LOOP
                                            // }
                                            let statement_inner_pairs = statement_pair.into_inner();
                                            for statement_inner_pair in statement_inner_pairs {
                                                match statement_inner_pair.as_rule() {
                                                    Rule::MUT => {
                                                        let mut_pairs =
                                                            statement_inner_pair.into_inner();
                                                        for mut_pair in mut_pairs {
                                                            match mut_pair.as_rule() {
                                                                _ => {
                                                                    println!(
                                                                        "\t\t\tmut: {:?} - {:?}",
                                                                        mut_pair.as_rule(),
                                                                        mut_pair.as_str()
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Rule::IMMUT => {
                                                        let immut_pairs =
                                                            statement_inner_pair.into_inner();
                                                        for immut_pair in immut_pairs {
                                                            match immut_pair.as_rule() {
                                                                _ => {
                                                                    println!(
                                                                        "\t\t\timmut: {:?} - {:?}",
                                                                        immut_pair.as_rule(),
                                                                        immut_pair.as_str()
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Rule::VARIABLE_REASS => {
                                                        let variable_reass_pairs =
                                                            statement_inner_pair.into_inner();
                                                        for variable_reass_pair in
                                                            variable_reass_pairs
                                                        {
                                                            match variable_reass_pair.as_rule() {
                                                                _ => {
                                                                    println!(
                                                                        "\t\t\tvar_reass: {:?} - {:?}",
                                                                        variable_reass_pair.as_rule(),
                                                                        variable_reass_pair.as_str()
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Rule::FUNCTION_CALL => {
                                                        let function_call_pairs =
                                                            statement_inner_pair.into_inner();
                                                        for function_call_pair in
                                                            function_call_pairs
                                                        {
                                                            match function_call_pair.as_rule() {
                                                                _ => {
                                                                    println!(
                                                                        "\t\t\tfn_call: {:?} - {:?}",
                                                                        function_call_pair.as_rule(),
                                                                        function_call_pair.as_str()
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Rule::IMPL_ACCESS => todo!(),
                                                    Rule::MODULE_ACCESS => todo!(),
                                                    _ => {
                                                        // println!(
                                                        //     "\t\t\tstmt_inner: {:?} - {:?}",
                                                        //     statement_inner_pair.as_rule(),
                                                        //     statement_inner_pair.as_str()
                                                        // );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Rule::RETURN_STMT => {
                                        let expression_pairs = function_body_pair.into_inner();
                                        for expression_pair in expression_pairs {
                                            println!(
                                                "\t\tret_stmt: {:?}",
                                                expression_pair.as_str()
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {
                            println!("\t\t - {:?}", function_pair.as_rule());
                        }
                    }
                }

                // go back to parent scope
                symbol_table.current_scope = symbol_table
                    .scopes
                    .get(&function_name)
                    .unwrap()
                    .parent
                    .clone();
            }
            // Rule::PARAMETER_LIST => todo!(),
            // Rule::PARAMETER => todo!(),
            // Rule::RETURN_TYPE => todo!(),
            // Rule::RETURN_STMT => todo!(),

            // Rule::FUNCTION_BODY => todo!(),
            // Rule::FUNCTION_CALL => todo!(),
            // Rule::ARGUMENTS_LIST => todo!(),
            // Rule::ARGUMENT => todo!(),

            // Rule::ERROR_CHECK => todo!(),

            // Rule::CLOSURE => todo!(),

            // Rule::MODULE_DECL => todo!(),
            // Rule::MODULE_BLOCK => todo!(),
            // Rule::MODULE_ACCESS => todo!(),

            // Rule::IMPORT_DECL => todo!(),
            // Rule::IMPORT_IDENTIFIER => todo!(),

            // Rule::STRUCT_DECL => todo!(),
            // Rule::STRUCT_FIELD => todo!(),

            // Rule::ENUM_DECL => todo!(),
            // Rule::ENUM_VARIANT => todo!(),

            // Rule::IMPL_DECL => todo!(),

            // Rule::PROGRAM_BLOCK => todo!(),
            // Rule::PROGRAM => todo!(),
            _ => {
                println!("hmm aa baaki chhe: {:?}", pair.as_rule());
            }
        }
    }

    println!("{:#?}", symbol_table);
}
