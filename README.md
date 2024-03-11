<p align="center">
    <img src="image.png" alt="arc-logo" width="200"/>
</p>
<h1 align="center">The ARC Programming Language</h1>

## This is the main source code repository for Arc. This repository is built as a submisson for the course CS 327 at IITGN  

### Contributors

- Mithil Pechimuthu -> <pechimuthumithil@iitgn.ac.in>
- Naman Dharmani -> <dharmaninaman@iitgn.ac.in>
- Tirth Patel -> <pateltirth@iitgn.ac.in>  

### Assignment 1

- The `Spec.md` file describes the syntax and grammar for our ([Team - arc-lang](https://github.com/IITGN-CS327-2024/our-own-compiler-arc-lang)) simple, imperative programming language, `Arc`.

### Assignment 2  

- We build a lexer (`./arc/src/lexer.rs`) and test it with our custom test cases present in the folder named `./arc/testcases`.

### Assignment 3

- Show lexed tokens as the output.

### Assignment 4

- We build a PEG parser for the language. The grammar can be found in the file named `grammar.pest`. It is written using `pest`.
- To run the compiler until this stage, do the following:
- #### Build
  ```rs
  cargo build
  ```
  #### Run
  ```rs
  cargo run <path to the .arc source code file>
  ```
  #### Example
  ![parsed_op](https://github.com/IITGN-CS327-2024/our-own-compiler-arc-lang/assets/119656326/07e2a7d6-6ed8-4b2e-a2e8-6d13d7cc7a6f)

- To run all the testcases use the following command:
  ```shell
  ./run_tests.sh
  ```

### Assignment 5

- We build an AST (Abstract Syntax Tree) for the language. Before that we first finalze the design by drawing the classes of the AST. One can find it in `AST_Classes.jpg` or `AST_Classes.svg`.  
