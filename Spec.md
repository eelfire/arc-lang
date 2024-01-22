# The Arc Programming Language Specification

##### Last updated: Jan 22, 2024

## Introduction

This is initial specifications sheet for Arc. The specification may change as the language evolves.
  
Arc stands for **A**nother **R**ust base **C**ompiler. Arc is an experimental programming language, tuned for compilation to [WASM](https://webassembly.org/). It is fast and light. Arc is statically typed language. The remainder of this spec-sheet will take you through the various specifications of the language.  

- All productions mentioned here are in modified EBNF (Extended Backus-Naur Form).

```go
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

## Source Code Representation

Only `UTF-8` encoding file with `.arc` extension is supported.

#### Character Sets

```go
letter        = <all unicode letters> | "_" .
decimal_digit = "0" … "9" .
binary_digit  = "0" | "1" .
octal_digit   = "0" … "7" .
hex_digit     = "0" … "9" | "A" … "F" | "a" … "f" .
```

## Lexical Elements

### Comments

Useful for documenting the program. Any of these forms can be used:

1. Line comments are initiated by the `//` sequence and extend until the end of the line.
2. General comments are initiated by the `/*` sequence and conclude with the first occurrence of the `*/` sequence.

A general comment that doesn't include line breaks behaves as a space. Any other type of comment behaves as a line break.

### Semicolons

`;`: These will be used as terminators for all statements and \_\_\_.

## Identifiers

Used for naming variables and types. First character must be a letter.

```go
identifier = letter { letter | unicode_digit } .
```

```
x
cs_327
GRADE_11
_compilers
```

## Keywords

```
let       // Used to declare variable
mut       // Used to declare mutability of variable
type      // Used to declare type
fx        // Used to declare function
main      // Used to define the start of execution (entry point) of a program
return    // Used to return a value from a function to the caller
while     // Keyword for 'while' loop
for       // Keyword for 'for' loop
continue  // Used to stop an iteration and continue with the next in a loop block
break     // Used to abruptly jump out of the loop block
in        // Keyword to represent membership in arrays, lists, tuplesa and iterators
import    // Import built-in and user defined packages
pub       // public
mod       // 
super     // 
struct    // Denote user defined structures
enum      // Denote user defined enumerations
impl      // Denote user defined implementations (similar to methods) for types and structs
self      // Denote current instance of a type or struct
true      // Boolean type true 
false     // Boolean type false
try       // Exception handling
catch     // "        "
throw     // "                 "
```

## Operators

Following are the set of supported operator (including assignment operators)

```
Arithmetic Operators
```

```go
+    &     +=    &=     &&    ==    !=    (    )
-    |     -=    |=     ||    <     <=    [    ]
*    ^     *=    ^=     <-    >     >=    {    }
/    <<    /=    <<=    ++    =     :=    ,    ;
%    >>    %=    >>=    --    !     ...   .    :
**   &^          &^=          ~
```

## Integer Literals

```go
decimal_digits = decimal_digit { [ "_" ] decimal_digit } .
binary_digits  = binary_digit { [ "_" ] binary_digit } .
octal_digits   = octal_digit { [ "_" ] octal_digit } .
hex_digits     = hex_digit { [ "_" ] hex_digit } .

int_lit        = decimal_lit | binary_lit | octal_lit | hex_lit .
decimal_lit    = "0" | ( "1" … "9" ) [ [ "_" ] decimal_digits ] .
binary_lit     = "0" ( "b" | "B" ) [ "_" ] binary_digits .
octal_lit      = "0" ( "o" | "O" ) [ "_" ] octal_digits .
hex_lit        = "0" ( "x" | "X" ) [ "_" ] hex_digits .
```

## Character Literals

```go
char_lit = "'" unicode_value "'" .
unicode_value = <all unicode characters> | escaped_char .
escaped_char  = `\` ( "n" | "r" | "t" | "v" | `\` | "'" | `"` ) .
```

```
'a'
'\\n'
'2'
```

## String Literals

```go
string_lit = `"` { unicode_value } `"` .
```

```
"Hello, World!"
"Hello, \nWorld!"
"CS 327: Compilers"
```

## Variables

All variables are *immutable* by default and mutability for a variable can be added using the `mut` keyword after the `let` keyword. A variable can be assigned a value at the time of declaration in which case the type is optional and can be inferred.

```rs
let x = 23;
let mut z = 469;
let m;      // illegal: a type must be defined for uninitialised variables
let weather: string = "cold";
```

## Types

### Boolean types

Keyword for boolean type is `bool` whose truth values are predeclared constants `true` and `false`.

```
let x: bool = true;
let y: bool = false;
```

### Numeric types

An *integer* type, and *floating-point* type represent the set of integer and floating-point values respectively. These are collectively called as *numeric* types.

```
u32   the set of all unsigned 32-bit integers (0 to 4294967295)
u64   the set of all unsigned 64-bit integers (0 to 18446744073709551615)

i32   the set of all signed 32-bit integers (-2147483648 to 2147483647)
i64   the set of all signed 64-bit integers (-9223372036854775808 to 9223372036854775807)

f32   the set of all IEEE-754 32-bit floating-point numbers
f64   the set of all IEEE-754 64-bit floating-point numbers

char  alias for u32
```

### String types

A *string* type is a set of string values (a sequence of characters). Length of string is otherwise the number of characters present in it which is non negative.

A *string slice* type is a subsequence of characters derived from an existing string value.

`string`

### Array types

A fixed size collection of objects of same type

`[]`

### Tuple types

A fixed size collection of literals of different types

`()`

### List types

A variable size array

`![]`

### Syntax for type declaration

```rs
type t := i32 | i64 | f32 | f64
```

## Blocks & Scope

```go
Block = "{" StatementList "}" .
StatementList = { Statement ";" } .
```

A block can also return a final value using the `return` keyword.

```
{
    statement1;
    statement2;
    return exp1 + exp2;
}
```

Variables defined inside a block statment are scoped to itself and cannot be accessed outside. Variables defined in parent block can be accessed in inner/child blocks.

## Flow Control

```
for variable in range_expression {}

while condition {}

if condition1 {} else if condition2 {} else {}

match variable {
    condition1 => {},
    condition2 => {},
    condition3 => {},
    _ => {},
}
```

`if-else if-else` and `match` conditionals can have return value.

Examples:

```
for line in lines {
    print(line);
}

while x > 0 {
    print("x: {}", x);
    x = x - 1;
}

if

match
```

## Functions

```
fx function_name(a type, b type) ~ type {
    statement1;
    statement2;

    return expr1 + expr2;
}
```

Use of `fx` for defining the function. Function name is identifier.

Examples:

```
fx add_u32s(a u32, b u32) ~ u32 {
    return a + b;
}

fx say_hi() {
    print("hi");
}
```

## Closures

...

## Error Handling (Exceptions)  

Errors or bugs in the program cause exceptions, that can be handled by using the ```try-catch-throw``` blocks.  

```
try this() {
    success_statement;
} catch {
    catch_statement;
}

fx this() { throw “something wrong”; }
```

## Modules

Filename is automatically considered as module name. Filename can be used for imports. Child modules can be defined.

```
mod module_name {
    fx a_function() {
        statement1;
    }

    fx b_function() ~ u32 {
        return 11;
    }

    mod child_module {
        import super::*;
        fx yay() {
        let sqrt_2 = that_module::sqrt(4);
        return sqrt_2;
        }
    }
}
```

## Entry Point

`main` is entry point for the arc lang.

```
fx main() {
    statement1;
    statement2;
    statement3;
}
```

## Imports

`import` keyword is used for importing from another module. `*` can be used to import everything public from the module. `super` keyword is used to access parent module.

```
import module::function_name;
import super::this_function;
import another_module::*;
import folder_name::module_name::that_function;
```

## Public and Private

By default everything is private. `pub` keyword can be used to make it public.

Following are exportable using `pub` keyword.

```
function
type
```

```
pub fx public(a i32) ~ i32 {
    statement;
    return expr;
}

fx private(b u64) ~ bool { // this function is private
    statement;
    return expr;
}
```

## Other

`struct`, `enum`, `impl` can be introduced to arc lang if time permits.

```
struct StructName {
    field1: u32,
    field2: u32,
    field3: i32,
    field4: SomeStruct,
    field5: SomeEnum,
}

enum EnumName {
    Option1,
    Option2,
    Option3,
}

impl SomeStruct {
    fx len(&self) ~ u32 {
        statement;
        return length;
    }
}

impl string {
    fx len(&self) ~ u32 {
        statement;
        return length;
    }
}
```
