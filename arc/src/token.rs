// keywords
// identifiers
// operators
// punctuation
// literals
// misc

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // keyword,
    /// this is let
    Let,
    Mut,
    Fx,
    Tilde,
    Import,
    Pub,
    Mod,
    Super,
    Itself, // self keyword

    If,
    ElseIf,
    Else,
    While,
    For,
    In,
    Continue,
    Break,
    Match,
    FatArrow,

    Return,
    Result,
    Okay, // ok keyword
    Err,

    Type,
    Struct,
    Enum,
    Impl,

    // arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    UnaryPlus,
    UnaryMinus,

    // logical operators
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // comparison operators
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,

    // assignment operators
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    BitwiseLeftShiftAssign,
    BitwiseRightShiftAssign,

    // bitwise operators
    BitwiseNot, // unary
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,

    // punctuations
    LParen,
    RParen,
    LAngle,
    RAngle,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Dot,
    Colon,
    BackTick,
    Quotation,
    Apostrophe,

    // Literals
    Num(i64),
    Char(char),
    String(String),
    Bool(bool),

    // Identifiers
    Ident(String),

    // Misc
    SingleLineComment,
    MultiLineComment,

    EOF,
}

impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str() {
            "let" => Token::Let,
            "mut" => Token::Mut,
            "fx" => Token::Fx,
            "~" => Token::Tilde,
            "import" => Token::Import,
            "pub" => Token::Pub,
            "mod" => Token::Mod,
            "super" => Token::Super,
            "self" => Token::Itself,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "continue" => Token::Continue,
            "break" => Token::Break,
            "match" => Token::Match,
            "=>" => Token::FatArrow,
            "return" => Token::Return,
            "result" => Token::Result,
            "ok" => Token::Okay,
            "err" => Token::Err,
            "type" => Token::Type,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "impl " => Token::Impl,

            // arithmetic operators
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Star,
            "/" => Token::Slash,
            "%" => Token::Percent,

            // logical operators
            "&&" => Token::LogicalAnd,
            "||" => Token::LogicalOr,
            "!" => Token::LogicalNot,
            "&" => Token::BitwiseAnd,
            "|" => Token::BitwiseOr,
            "`" => Token::BitwiseNot,
            "^" => Token::BitwiseXor,
            "<<" => Token::BitwiseLeftShift,
            ">>" => Token::BitwiseRightShift,

            // comparison operators
            "==" => Token::Eq,
            "!=" => Token::Neq,
            // ">" => Token::Gt,
            // "<" => Token::Lt,
            ">=" => Token::Gte,

            // assignment operators
            "=" => Token::Assign,
            "+=" => Token::PlusAssign,
            "-=" => Token::MinusAssign,
            "*=" => Token::StarAssign,
            "/=" => Token::SlashAssign,
            "%=" => Token::PercentAssign,

            // literals
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            // Comments
            _ => Token::Ident(token),
        }
    }

    pub fn new_literal_num(token: String) -> Self {
        match token.parse::<i64>() {
            Ok(num) => Token::Num(num),
            Err(_) => {
                eprintln!("Error: Could not parse to literal to num: {}", token);
            }
        }
    }

    pub fn new_literal_char(token: String) -> Self {
        match token.chars().nth(0) {
            Some(ch) => Token::Char(ch),
            None => {
                eprintln!("Error: Could not parse to literal to char: {}", token);
            }
        }
    }

    pub fn new_literal_string(token: String) -> Self {
        Token::String(token)
    }
}
