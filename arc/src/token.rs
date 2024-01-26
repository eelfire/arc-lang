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
    DoubleStar,

    // logical operators
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,

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
    // and more...

    // punctuations
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Dot,
    Colon,

    // Literals
    NumType,
    Char(char),
    String(String),

    // Identifiers
    Ident(String),

    // Misc
    SingleLineComment(String),
    MultiLineComment(String),
    EOF,
}

struct Int {
    int32: i32,
    int64: i64,
}

struct Float {
    float32: f32,
    float64: f64,
}

struct NumType {
    int: Int,
    float: Float,
}
