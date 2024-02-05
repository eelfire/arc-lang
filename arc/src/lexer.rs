use std::path::Path;

use crate::token::Token;

pub fn run(input_file: &str) {
    println!("Lexing Stage Initiated...");

    // read chars from utf-8 encoded file
    let path = Path::new(input_file);
    let contents = std::fs::read_to_string(path).expect("failed to read file");

    let mut tokens = Vec::new();
    let mut token = String::new();

    let mut is_last = [false, false, false, false, false, false]; // [is_operator, is_number, is_string, is_char, is_identifier, is_comment]
    let mut prev_ch: char = ' ';

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
        } else if is_last[5] {
            if let Some(last_token) = tokens.last() {
                match last_token {
                    Token::SingleLineComment => {
                        if ch == '\n' {
                            is_last[5] = false;
                            continue;
                        } else {
                            continue;
                        }
                    }
                    Token::MultiLineComment => {
                        if prev_ch == '*' && ch == '/' {
                            is_last[5] = false;
                            continue;
                        } else {
                            prev_ch = ch;
                            continue;
                        }
                    }
                    _ => {}
                }
            }
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
            '<' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::LAngle);
            }
            '>' => {
                handle_token(&mut token, &mut tokens, &mut is_last[4]);
                tokens.push(Token::RAngle);
            }
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '!' | '=' | '`' | '^' => {
                // is_operator = true;
                if ch == '*' && prev_ch == '/' {
                    tokens.push(Token::MultiLineComment);
                    is_last[5] = true;
                    token.clear();
                } else if ch == '/' && prev_ch == '/' {
                    if let Some(popped) = tokens.pop() {
                        if popped != Token::Slash {
                            tokens.push(popped);
                        }
                    }
                    tokens.push(Token::SingleLineComment);
                    is_last[5] = true;
                    token.clear();
                } else {
                    is_last[0] = true;
                    token.push(ch);
                }
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
        prev_ch = ch;
    }
    println!("Lexing Stage Complete...");
    println!("{:?}", tokens);
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
            let tok = Token::new_literal_num(token.clone());
            if let Some(t) = tok {
                tokens.push(t);
            }
            // tokens.push();
        }
        Token::Char(_) => {
            let tok = Token::new_literal_char(token.clone());
            if let Some(t) = tok {
                tokens.push(t);
            }
            // tokens.push();
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
