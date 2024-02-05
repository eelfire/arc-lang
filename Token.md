# Token Classes for ARC Programming Language

This document outlines the various token classes used in the programming language, providing explanations for each. Tokens are the smallest units in the language, classified into different types based on their role and syntax.

## Keywords

Keywords are reserved words that have special meaning in the language. They cannot be used as identifiers.

- `Let`, `Mut`, `As`: Used for variable declarations and type assertions, allowing for mutable and immutable bindings.
- `Fx`, `Tilde`: Function declaration keyword (_fx_) and specify return type with tilde (~).
- `If`, `ElseIf`, `Else`, `While`, `For`, `In`, `Match`, `Continue`, `Break`, `Return`: Control flow keywords for execution of the program, allowing for conditional statements, loops, and early exits.
- `Result`, `Okay`, `Err`: Used for error handling, where _Result_ represents a type that can return either a success (ok) or an error (err).
- `Type`: Defines new types, allowing for the creation of aliases or new data types.
- `Struct`: Structured data type, which is a collection of fields to represent an entity with multiple attributes.
- `Enum`: Defines an enumeration, which is a type that can have one of a few predefined values.
- `Impl`: Define implementations of functions and methods associated with a specific type.
- `Import`, `Pub`, `Mod`, `Super`: Module and visibility management, where _Import_ is for importing modules, _Pub_ for making entities public, _Mod_ for defining modules, and _Super_ for accessing parent module contents.
- `Itself`: Refers to the current instance of a class or struct.

## Operators

Operators are symbols that perform operations on operands.

### Arithmetic Operators

- `Plus`, `Minus`, `Star`, `Slash`, `Percent`: Standard arithmetic operations.
- `UnaryPlus`, `UnaryMinus`: Unary operations for positive and negative values.

### Logical Operators

- `LogicalAnd`, `LogicalOr`, `LogicalNot`: Standard logical operations, used in conditionals and loops.

### Comparison Operators

- `Eq`, `Neq`, `Gt`, `Lt`, `Gte`, `Lte`: Equality, inequality, greater than, less than comparisons.

### Assignment Operators

- `Assign`, `PlusAssign`, `MinusAssign`, etc.: Used to assign and update values of variables.

### Bitwise Operators

- `BitwiseAnd`, `BitwiseOr`, `BitwiseXor`, `BitwiseNot`: Perform bitwise operations.
- Shift operators: `BitwiseLeftShift`, `BitwiseRightShift`.

## Punctuation

Punctuation tokens are used to structure the code.

- Parentheses, brackets, braces: `LParen`, `RParen`, `LBracket`, `RBracket`, `LBrace`, `RBrace`.
- `Comma`, `Semicolon`, `Dot`, `Colon`: Used in expressions, function calls, and type annotations.

## Literals

Literals represent fixed values in the code.

- `Num(i64)`: Integer literals.
- `Char(char)`: Character literals.
- `String(String)`: String literals.
- `Bool(bool)`: Boolean literals (`true`, `false`).

## Identifiers

Identifiers are user-defined names for variables, functions, and types.

- `Ident(String)`: Matches any user-defined name that doesn't fall into other token classes.

## Miscellaneous

Other tokens that don't fit into the above categories.

- Comments: `SingleLineComment`, `MultiLineComment`.
- `EOF`: Represents the end of the file.

## Implementation Notes

Each token class is defined within an enum, facilitating the parsing and analysis process in the compiler. The `Token` enum includes methods for creating new instances of tokens, particularly useful for literals where the value needs to be parsed from the source code.
