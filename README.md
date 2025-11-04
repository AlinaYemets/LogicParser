# Logic Parser and Evaluator

This project is a simple **logical expression parser and evaluator** written in Rust.  
It allows to parse and evaluate boolean expressions with variables and logical operators such as `NOT`, `AND`, `OR`, and `->`.  
The program can read an input file, automatically detect variables, ask for their truth values, and print the result.

## Project Structure
```text
`logic-parser/
├── src/
│ ├── lib.rs # Core logic: parsing, AST structure and evaluation
│ ├── main.rs # CLI entry point that interacts with the user
│ └── grammar.pest # Grammar definition for the parser
├── tests/
│ └── parser_tests.rs # Unit tests for the parser and evaluator
├── Cargo.toml # Dependencies and metadata
└── README.md # Project documentation```

## Grammar Overview

- **Variables** - uppercase/lowercase letters or numbers, e.g. `A`, `B`, `x1`
- **Operators:**
  - `NOT` - negation (highest precedence)
  - `AND` - conjunction
  - `OR` - disjunction
  - `->` - implication (lowest precedence)
- **Parentheses** - `(` and `)` for grouping expressions

The grammar defines logical hierarchy of operations: not -> and -> or -> implication

## Lib Overview

- **Grammar integration:** Uses the pest parsing library and the grammar defined in logic.pest to parse logical expressions.
- **Expr enum:** Represents the structure of logical expressions as an Abstract Syntax Tree (AST).
- **parse_expression():** Parses a string into an Expr based on the grammar and constructs the AST using a helper function build_ast().
- **evaluate():** Recursively evaluates the expression by applying logical rules to each node in the AST.

## Test Overview

Unit tests are placed in the tests/ directory and cover:

- Each logical operator (NOT, AND, OR, ->)
- Variable evaluation
- Complex nested expressions
- Ensuring parser correctly builds the AST

**Run tests with**: `cargo test -- --nocapture`

## Main Overview
```text
main.rs
├── parse_file()
│   ├── reads text from a file
│   ├── calls parse_expression() from lib.rs
│   ├── obtains the AST (Expr)
│   ├── collect_vars() extracts all variable names
│   ├── read_bool() prompts the user for values
│   └── evaluate() computes the final result
└── prints the summary to the console```

## How to Run

- **Run the CLI:** `cargo run`
- **Parse expressions from a file:** `cargo run -- parse input.txt`
- **Show help:** `cargo run -- help`
- **Show project credits:** `cargo run -- credits`
