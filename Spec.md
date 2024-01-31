# The Arc Programming Language Specification

##### Last updated: Jan 22, 2024

## Introduction

This is an initial specifications sheet for Arc. It may change as the language evolves.

Arc stands for **A**nother **R**ust based **C**ompiler. Arc is an _experimental_ programming language, tuned for compilation to [WASM](https://webassembly.org/). It is fast and light. It is a statically typed language. The remainder of this document will take you through the various specifications of the language.

- Productions are formed by combining terms and the operators listed below, with each operator having a higher precedence than the one that follows it. All productions mentioned in this document follow a variant of EBNF (Extended Backus-Naur Form).

```rust
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

## Source Code Representation

The compiler only supports files that have the `.arc` extension and are encoded in `UTF-8`.

#### Character Sets

```rust
letter        = <all unicode letters> | "_" .
decimal_digit = "0" … "9" .
binary_digit  = "0" | "1" .
octal_digit   = "0" … "7" .
hex_digit     = "0" … "9" | "A" … "F" | "a" … "f" .
```

## Lexical Elements

### Comments

Useful for documenting any program. Any of these forms can be used:

1. Line comments are initiated by the `//` sequence and extend until the end of the line.
2. General comments are initiated by the `/*` sequence and conclude with the first occurrence of the `*/` sequence.

A general comment that doesn't include line breaks behaves as a space. Any other type of comment behaves as a line break.

### Semicolons

`;` will be used as terminators for all statements and indicate the end of a line or instruction, allowing for multiple statements on a single line if desired.

## Identifiers

They serve as names for variables and types and must begin with a letter.

```rust
identifier = letter { letter | unicode_digit } .
```

```
x
_compilers
cs_327
GRADE_11
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
result    // Keyword for error handling
ok        // Keyword for error handling in case of success
err       // Keyword for error handling in case of error
```

## Operators and punctuations

Following are the sets of supported operators (unary and binary).

```go
Arithmetic Operators
+    -    *    /    %
++    --
```

```go
Logical Operators
&&    ||    !
```

```go
Comparison Operators
==    !=    <    >    >=    <=
```

```py
Assignment Operators
=     +=    -=    *=    /=
%=    &=    |=    ^=    <<=
>>=
```

```go
Bitwise Operators
!    &    |    ^    <<    >>
```

```go
Punctuations
(    )
[    ]
{    }
,    ;
.    :
`
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
'\n'
'2'
```

## String Literals

```go
string_lit = `"` { unicode_value } `"` .
raw_string = "`" { unicode_value } "`" .
```

```
"Hello, World!"
"Hello, \nWorld!"
"CS 327: Compilers"
```

## Variables

All variables are _immutable_ by default and mutability for a variable can be added using the `mut` keyword after the `let` keyword. A variable can be assigned a value at the time of declaration in which case the type is optional and can be inferred.  
Variables names must follow the following rules:
1) Must not be a keyword as defined in the *Keywords* section.  
2) Must start with a letter or an underscore and must not start with digit (For example, Valid--> `_myVar`, Invalid--> `1one`).
3) Must only contain alpha-numeric characters along with the underscore.
4) Are case-sensitive.  

```rust
let x = 23;
let mut z = 469;
let weather: string = "cold";
let m;      // illegal: a type must be defined for uninitialised variables
```

## Types

### Boolean types

Keyword for boolean type is `bool` whose truth values are predeclared constants `true` and `false`.

```
let x: bool = true;
let y: bool = false;
```

### Numeric types

An _integer_ type, and _floating-point_ type represent the set of integer and floating-point values respectively. These are collectively called as _numeric_ types.

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

A _string_ type is a set of string values (a sequence of characters). Length of string is otherwise the number of characters present in it which is non negative.

A _string slice_ type is a subsequence of characters derived from an existing string value.

```rust
let s: string = "Hello, World!";
let s_slice: string = s[3:5]; // "lo"
let inferred_s_type = "Hello, World!"; // inferred as string

let s_len = s.len(); // length of string, impl can be used to define len() for types [similar to methods]
let big_s = s + " How are you?"; // concatenation
```

### Array types

A fixed size collection of objects of same type. `[]` is used to declare array.
#### Syntax
```rust
// Declaring an array
let <array_name>: [<element_type>; <size>] = [element1, element2...];
```
#### Features
array.*len*()  
This return a non negative integer as the length of the array (number of elements).  

array *[M]*  
This returns the element in the array at the Mth index.  

array *[M:N]*  
This return the slice of the array from index M, to index N-1. The returned type is also an array.  

#### Examples  
```rust
let arr: [u32; 3] = [1, 2, 3];
let inferred_arr_type = [1, 2, 4, 6]; // inferred as [i32; 4]

let x = arr[0]; // accessing array elements
let a_slice = arr[0:3] // slicing array, elements [0, 3)
```

### Tuple types

A fixed size collection of literals of different types. Tuple contains elements of different types. `()` is used to declare tuple.
#### Syntax
```rust
// Declaring an tuple
let <tuple_name>: (<element1_type>, <element2_type>,...) = (element1, element2...);
// or
let <tuple_name>: (<element_type>, <size>) = (element1, element2...);
```

#### Features
tuple.*len*()  
This return a non negative integer as the length of the tuple (number of elements).  

tuple.*(M)*  
This returns the element in the tuple at the Mth index.  

tuple *[M:N]*  
This return the slice of the array from index M, to index N-1. The returned type is also an array.  

let <iterable_tuple> = tuple;  
The tuple type also supports unpacking (destructuring) as shown in the example.  

#### Examples 
```rust
let tup: (i32, u64, u32) = (-5, 67, 13);
let inferred_tup_type = (-5, 67, 13); // inferred as (i32, i32, i32)

let (x, y, z) = tup; // destructuring
let x = tup.0; // accessing tuple elements
```

### List types

A dynamic size array. List contains elements of same type. `<>` is used to declare list.
#### Syntax
```rust
// Declaring an list
let <list_name>: <<element_type>> = <element1, element2...>;
```

#### Features
list.*len*()  
This return a non negative integer as the length of the list (number of elements).  

list *[M]*  
This returns the element in the list at the Mth index.  

list *[M:N]*  
This return the slice of the array from index M, to index N-1. The returned type is also an array.  

list.*push(element)*  
This appends element to the end of the list, as the lists do not have a fixed sixe and can grow from both directions.   

#### Examples 
```rust
let a_list: <u32> = <1, 2, 3>;

let x = a_list[0]; // accessing list elements
let a_slice = a_list[0:3] // slicing list, elements [0, 3)
a_list.push(4); // push element to list, demonstrates the dynamic size
```

### Syntax for type declaration

```rust
type t := i32 | i64 | f32 | f64
type s := (u32, f32)
```

## Blocks & Scope

```go
Block = "{" StatementList "}" .
StatementList = { Statement ";" } .
```

A block can also return a final value using the `return` keyword.

```rust
{
    statement1;
    statement2;
    return exp1 + exp2;
}
```

Variables defined inside a block statment are scoped to itself and cannot be accessed outside. Variables defined in parent block can be accessed in inner/child blocks.

## Flow Control
#### Examples  
```rust
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

Special keywords such as `break`, `continue` and `return` are used arbitrarily jump out of loops to manipulate the execution flow.  
`break` jumps out of the iteration.  
`continue` skips the iteration.  
`return` exits functions and return a value to the caller.  

#### Examples  
```rust
for variable in range_expression {}
    if condition1 {}
        break; 
while condition {}
    if condition1 {}
        continue;

fx add(a i32, b i32) ~ i32 {
    let c = a + b;
    return c;
}

```
`if - else if - else` and `match` conditionals can have return value.

Examples:

```rust
for line in lines {
    print(line);
}

while x > 0 {
    print("x: {}", x);
    x--;
}

if isSunny {
    print("It's a sunny day!");
} else if isRaining {
    print("It's raining outside.");
} else {
    print("The weather is unknown.");
}

match grade {
    'A' => "Excellent!",
    'B' => "Good job!",
    'C' => "Satisfactory.",
    _   => "Needs improvement.",
}
```

## Functions

```rust
fx function_name(a type, b type) ~ type {
    statement1;
    statement2;

    return expr1 + expr2;
}
```

Use of `fx` for defining the function. Function name is identifier.

Examples:

```rust
fx add_u32s(a u32, b u32) ~ u32 {
    return a + b;
}

fx say_hi() {
    print("hi");
}
```

## Closures

`fx() {}` can be used to define closures. Closures are anonymous functions. Closures can be assigned to variables. Closures can be passed as arguments to other functions. Closures can capture variables from the enclosing scope. Last expression in the closure is returned without explicit mention of return keyword.

```rust
let add = fx(a, b) { a + b };
let multiply = fx(x, y) {
    let result = x * y;
    result
};
```

## Exception Handling

Exceptions can occur during the runtime of a program, causing the program to exhivit an undefined behaviour. The programmer can handle these exceptions by using the `Result<success_type, error>`. Furthermore, `ok` and `err` can be used to return the success and error values respectively. `if let` syntax is used to destructure the result.

#### Examples  
```rust
let res = fx() ~ result<u32, string> {
    if condition {
        return ok(1);
    } else {
        return err("Error message");
    }
}

if let ok(value) = res {
    print("Value: {}", value);
} else {
    print("Error: {}", res.err());
}
```

## Modules

Filename is automatically considered as module name. Filename can be used for imports. Child modules can be defined.

```rust
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

```rust
fx main() {
    statement1;
    statement2;
    statement3;
}
```

## Imports

`import` keyword is used for importing from another module. `*` can be used to import everything public from the module. `super` keyword is used to access parent module.

```python
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

```rust
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

`struct`, `enum`, `impl`.

```rust
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
