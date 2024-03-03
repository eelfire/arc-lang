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
    As,
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
            "as" => Token::As,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "impl" => Token::Impl,

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

    pub fn new_literal_num(token: String) -> Option<Self> {
        let num = if token.len() > 1 {
            match &token[0..2] {
                "0b" | "0B" => i64::from_str_radix(&token[2..], 2),
                "0o" | "0O" => i64::from_str_radix(&token[2..], 8),
                "0x" | "0X" => i64::from_str_radix(&token[2..], 16),
                _ => token.parse::<i64>(),
            }
        } else {
            token.parse::<i64>()
        };
        if let Ok(n) = num {
            Some(Token::Num(n))
        } else {
            eprintln!("Error: Could not parse to literal to num: {}", token);
            None
        }
    }

    // pub fn new_literal_num(token: String) -> Option<Self> {
    //     // 0b1010, 0o123, 0x1A
    //     // 0B1010, 0O123, 0X1A
    // }

    pub fn new_literal_char(token: String) -> Option<Self> {
        let char = token.chars().nth(0);
        if let Some(c) = char {
            Some(Token::Char(c))
        } else {
            eprintln!("Error: Could not parse to literal to char: {}", token);
            None
        }
    }

    pub fn new_literal_string(token: String) -> Self {
        Token::String(token)
    }
}
