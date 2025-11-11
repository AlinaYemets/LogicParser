# Logic Parser

This project is a simple **logical expression parser and evaluator** written in Rust. It supports **boolean and predicate logic** with variables, predicates, quantifiers like `∀`, `∃`, and logical operators like `NOT`, `AND`, `XAND, `XOR`, `OR`, `->`, `<->`. The program can read an input file, automatically detect variables, ask for their truth values, and print the result.

## Project Structure
```text
logic-parser/
├── src/
│ ├── lib.rs # Core logic: parsing, AST structure and evaluation
│ ├── main.rs # CLI entry point that interacts with the user
│ └── grammar.pest # Grammar definition for the parser
├── tests/
│ └── parser_tests.rs # Unit tests for the parser and evaluator
├── Cargo.toml # Dependencies and metadata
└── README.md # Project documentation
```
## Grammar Overview

- **Variables** - uppercase/lowercase letters or numbers, e.g. `A`, `B`, `x1`
- **Predicates** - functions with optional arguments, e.g. P(x), Q(x,y)
- **Operators:**
  - `NOT` - negation
  - `AND` - conjunction
  - `XAND` - exclusive AND
  - `OR` - disjunction
  - `XOR` – exclusive OR
  - `->` - implication
  - `<->` - equivalence
- **Quantifiers:**
  - `∀x` - for all x
  - `∃x` - exists x
- **Parentheses** - `(` and `)` for grouping expressions

The grammar defines logical hierarchy of operations: not -> and -> or -> implication -> equivalence

## Lib Overview

- **Grammar integration:** Uses the pest parsing library and the grammar defined in logic.pest to parse logical expressions.
- **Expr enum:** Represents the structure of logical expressions as an Abstract Syntax Tree (AST).
- **parse_expression():** Parses a string into an Expr based on the grammar and constructs the AST using a helper function build_ast().
- **evaluate():** Recursively evaluates the expression by applying logical rules to each node in the AST.

## Test Overview

Unit tests are placed in the `tests/` directory and cover:

- Variable and predicate evaluation
- Each logical operator (`NOT`, `AND`, `XAND`, `XOR`, `OR`, `->`, `<->`)
- Quantifiers (`∀`, `∃`) and their effect on free variables
- Complex nested expressions
- Ensuring the parser correctly builds the AST

**Run tests with**: `cargo test -- --nocapture`

## Main Overview
```text
main.rs
├── collect_ent()
│   ├── recursively collects all variables and free predicates from an expression
│   └── ignores predicates under quantifiers
│
├── read_bool()
│   ├── prompts the user for boolean values of variables or predicates
│
├── parse_file(filename)
│   ├── reads lines from a file
│   ├── parses each line into an AST using parse_expression()
│   ├── collects free variables and predicates with collect_ent()
│   ├── prompts the user for values using read_bool()
│   └── evaluates the expression and prints the result
│
├── show_help()
│   └── prints CLI usage instructions
│
├── show_credits()
│   └── prints author and project information
│
└── main()
    ├── reads CLI arguments
    ├── calls parse_file(), show_help(), or show_credits() based on command
    └── prints error if command is unknown or missing
```

## How to Run

- **Run the CLI:** `cargo run`
- **Parse expressions from a file:** `cargo run parse input.txt`
- **Show help:** `cargo run help`
- **Show project credits:** `cargo run credits`
