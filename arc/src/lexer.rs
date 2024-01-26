use std::path::Path;

use crate::token::Token;

pub fn run() {
    println!("hi from lexer");

    // read chars from utf-8 encoded file
    let path = Path::new("testcases/add.arc");
    let contents = std::fs::read_to_string(path).expect("failed to read file");

    let mut tokens = Vec::new();
    let mut token = String::new();
    // let mut is_operator = false;
    // let mut is_number = false;
    // let mut is_string = false;
    // let mut is_char = false;
    let mut is_last = [false, false, false, false, false]; // [is_operator, is_number, is_string, is_char, is_identifier]

    for ch in contents.chars() {
        if is_last[2] {
            if ch == '"' {
                handle_token_literal(
                    &mut token,
                    &mut tokens,
                    Token::String("discard".to_string()),
                );
                is_last[2] = false;
                tokens.push(Token::Quotation);
            } else {
                token.push(ch);
            }
            continue;
        } else if is_last[3] {
            if ch == '\'' {
                handle_token_literal(&mut token, &mut tokens, Token::Char('d'));
                is_last[3] = false;
                tokens.push(Token::Apostrophe);
            } else {
                token.push(ch);
            }
            continue;
        } else if is_last[1] && !is_last[4] {
            if ch.is_ascii_digit() {
                token.push(ch);
                continue;
            } else {
                handle_token_literal(&mut token, &mut tokens, Token::Num(0));
                is_last[1] = false;
            }
        } else if is_last[0] {
            is_last[0] = false;
        }

        match ch {
            _ if ch.is_whitespace() => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
            }
            '(' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::LParen);
            }
            ')' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::RParen);
            }
            '[' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::LBracket);
            }
            ']' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::RBracket);
            }
            '{' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::LBrace);
            }
            '}' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::RBrace);
            }
            ',' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::Comma);
            }
            ';' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::Semicolon);
            }
            '.' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::Dot);
            }
            ':' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::Colon);
            }
            '"' => {
                // if is_string {
                //     handle_token_literal(&mut token, &mut tokens, Token::String);
                // }
                // is_string = !is_string;
                // is_last[2] = !is_last[2];
                tokens.push(Token::Quotation);
                is_last[2] = true;
            }
            '\'' => {
                // if is_char {
                //     handle_token_literal(&mut token, &mut tokens, Token::Char);
                // }
                // is_char = !is_char;
                // is_last[3] = !is_last[3];
                tokens.push(Token::Apostrophe);
                is_last[3] = true;
            }
            '0'..='9' => {
                // is_number = true;
                if !is_last[4] {
                    is_last[1] = true;
                }
                token.push(ch);
            }
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '!' | '=' | '<' | '>' | '`' | '^' => {
                // is_operator = true;
                is_last[0] = true;
                token.push(ch);
            }
            _ => {
                if is_last[0] {
                    handle_token(&mut token, &mut tokens, &mut is_last[4]);
                    // is_operator = false;
                    is_last[0] = false;
                }
                is_last[4] = true;
                token.push(ch);
            }
        }
    }

    println!("{:?}", tokens);

    // for c in contents.chars() {
    //     if c == '(' {
    //         let tok = Token::LParen;
    //         println!("{:?}", tok);
    //     }
    // }
}

fn handle_token(token: &mut String, tokens: &mut Vec<Token>, is_identifier: &mut bool) {
    if !token.is_empty() {
        tokens.push(Token::new(token.clone()));
        token.clear();
        *is_identifier = false;
    }
}

fn handle_token_literal(token: &mut String, tokens: &mut Vec<Token>, token_type: Token) {
    match token_type {
        Token::Num(_) => {
            tokens.push(Token::new_literal_num(token.clone()));
        }
        Token::Char(_) => {
            tokens.push(Token::new_literal_char(token.clone()));
        }
        Token::String(_) => {
            tokens.push(Token::new_literal_string(token.clone()));
        }
        // Token::Bool(_) => {
        //     tokens.push(Token::new_literal_bool(token.clone()));
        // }
        _ => {
            tokens.push(Token::new(token.clone()));
        }
    }
    token.clear();
}
