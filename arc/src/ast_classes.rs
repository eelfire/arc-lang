// <<<<<<<<<<<<<<<<<<<< Expression >>>>>>>>>>>>>>>>>>>>>>>>>
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    And,
    Or,
}

pub enum UnaryOp {
    Not,
    Neg,
    Deref,
}

pub enum Assignment {
    Equal,
    AddEqual,
    MinusEqual,
    MulEqual,
}

pub struct FunctionCall {
    name: String,
    args: Vec<Expression>,
}

pub struct MethodCall {
    object: Box<Expression>,
    method: String,
    args: Vec<Expression>,
}

pub struct FieldAccess {
    object: Box<Expression>,
    field: String,
}

pub struct ArrayAccess {
    array: Box<Expression>,
    index: Box<Expression>,
}

pub struct Cast {
    expr: Box<Expression>,
    target_type: Type,
}

pub struct Block {
    expressions: Vec<Expression>,
}

pub enum Expression {
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    Assignment(Assignment, Box<Expression>, Box<Expression>),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    FieldAccess(FieldAccess),
    ArrayAccess(ArrayAccess),
    Cast(Cast),
    Block(Block),
}

// <<<<<<<<<<<<<<<<<<<< Statement >>>>>>>>>>>>>>>>>>>>>>>>>
pub struct VariableDecl {
    name: String,
    var_type: Type,
    initializer: Option<Box<Expression>>, // This will hold the value the varible is initialized with, None if not initialized
}

pub struct ExpressionStmt {
    expression: Box<Expression>, // Bunch of expressions
}

pub struct IfStmt {
    condition: Box<Expression>,
    then_branch: Box<Statement>,
    elseif_branch: Option<Box<Statement>>, // Can be none, or one.
    else_branch: Option<Box<Statement>>, // Can be none, or one.
}

pub struct WhileStmt {
    condition: Box<Expression>,
    body: Box<Statement>,
}

pub struct ForStmt {
    variable: String,
    start: Box<Expression>,
    end: Box<Expression>,
    step: Option<Box<Expression>>,
    body: Box<Statement>,
}

pub struct ReturnStmt {
    value: Option<Box<Expression>>,
}

pub struct BreakStmt {}

pub struct ContinueStmt {}

pub enum Statement {
    VariableDecl(VariableDecl),
    ExpressionStmt(ExpressionStmt),
    IfStmt(IfStmt),
    ElseStmt(ElseStmt),
    ElseIfStmt(ElseIfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    ReturnStmt(ReturnStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
}

// <<<<<<<<<<<<<<<<<<<< Core Types >>>>>>>>>>>>>>>>>>>>>>>
pub enum PrimitiveType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
}

pub struct ArrayType {
    element_type: Box<Type>,
    length: usize,
}

pub struct TupleType {
    element_types: Vec<Type>,
}

pub struct ListType {
    element_type: Box<Type>,
}
    
pub struct StructType {
    name: String,
    fields: Vec<StructField>,
}

pub struct StructField {
    name: String,
    field_type: Type,
}

pub struct EnumType {
    name: String,
    variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    name: String,
    variant_type: Option<Box<Type>>,
}

pub struct FunctionType {
    input_types: Vec<Type>,
    output_type: Box<Type>,
}

pub enum Type {
    Primitive(PrimitiveType),
    Array(ArrayType),
    Tuple(TupleType),
    Struct(StructType),
    Enum(EnumType),
    Pointer(PointerType),
    Reference(ReferenceType),
    Function(FunctionType), // Represents a function signature
}

// // <<<<<<<<<<<<<<<<<<<< PatternMatching >>>>>>>>>>>>>>>>>>>>>>>
// pub struct MatchPattern {
//     pattern: Box<Pattern>,
//     expression: Box<Expression>,
// }

// pub struct MatchArm {
//     pattern: Box<Pattern>,
//     expression: Box<Expression>,
// }

// pub struct WildcardPattern {}

// pub struct DestructuringPattern {
//     name: String,
//     subpatterns: Vec<Pattern>,
// }

// pub enum Pattern {
//     MatchPattern(MatchPattern),
//     WildcardPattern(WildcardPattern),
//     DestructuringPattern(DestructuringPattern),
// }

