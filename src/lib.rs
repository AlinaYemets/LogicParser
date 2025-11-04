use pest:: Parser;
use pest_derive:: Parser;
use std:: collections:: HashMap;
use thiserror:: Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LogicParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Var(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Implies(Box<Expr>, Box<Expr>),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("pest parse error: {0}")]
    Pest(#[from] pest:: error:: Error<Rule>),

    #[error("empty parse result")]
    EmptyParse,
}

/// Evaluate logical expression with variable substitutions
pub fn evaluate(expr: &Expr, vars: &HashMap<String, bool>) -> bool {
    match expr {
        Expr:: Var(name) => *vars.get(name).unwrap_or(&false),
        Expr:: Not(inner) => !evaluate(inner, vars),
        Expr:: And(left, right) => evaluate(left, vars) && evaluate(right, vars),
        Expr:: Or(left, right) => evaluate(left, vars) || evaluate(right, vars),
        Expr:: Implies(left, right) => !evaluate(left, vars) || evaluate(right, vars), // thats equivalent to impication! trust me!
    }
}

/// Parse input string into logical expression AST
pub fn parse_expression(input: &str) -> Result<Expr, ParseError> {
    let mut parse_tree = LogicParser:: parse(Rule:: program, input)?;
    let pair = parse_tree.next().ok_or(ParseError:: EmptyParse)?;
    let ast = build_ast(pair);
    println!("AST: {:?}", ast); 
    // Maybe thats not a good idea to print AST in parse function, but i guess it is bettter than do this in each test 0-0
    Ok(ast)
}

/// Build AST recursively
fn build_ast(pair: pest::iterators::Pair<Rule>) -> Expr {
    let pair_str = pair.as_str();
    let mut inner = pair.clone().into_inner();

    match pair.as_rule() {
        Rule:: program => build_ast(inner.next().unwrap()),

        Rule:: implication => {
            let left = build_ast(inner.next().unwrap());
            if let Some(right) = inner.next() {
                Expr:: Implies(Box:: new(left), Box:: new(build_ast(right)))
            } else {
                left
            }
        }

        Rule:: or => inner.map(build_ast).reduce(|l, r| Expr:: Or(Box::new(l), Box::new(r))).unwrap(),

        Rule:: and => inner.map(build_ast).reduce(|l, r| Expr:: And(Box::new(l), Box::new(r))).unwrap(),

        Rule:: not => {
            let first = build_ast(inner.next().unwrap());
            if pair_str.trim_start().to_uppercase().starts_with("NOT") {
                Expr::Not(Box::new(first))
            } else {
                first
            }
        }

        Rule:: primary => inner.next().map_or(Expr:: Var(pair_str.to_string()), build_ast),

        Rule:: variable => Expr:: Var(pair_str.to_string()),

        _ => panic!("Unexpected rule: {:?}", pair.as_rule()),
    }
}
