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
    Predicate(String, Vec<String>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Xand(Box<Expr>, Box<Expr>),
    Nand(Box<Expr>, Box<Expr>),
    Nor(Box<Expr>, Box<Expr>), 
    Implies(Box<Expr>, Box<Expr>),
    Equiv(Box<Expr>, Box<Expr>),
    ForAll(String, Box<Expr>),
    Exists(String, Box<Expr>),
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
        Expr:: Predicate(name, args) => {
            let key = if args.is_empty() {
                name.clone()
            } else {
                format!("{}:{}", name, args.join(","))
            };
            *vars.get(&key).unwrap_or(&false)
        }
        Expr:: Not(inner) => !evaluate(inner, vars),
        Expr:: And(left, right) => evaluate(left, vars) && evaluate(right, vars),
        Expr:: Or(left, right) => evaluate(left, vars) || evaluate(right, vars),
        Expr:: Xor(left, right) => evaluate(left, vars) ^ evaluate(right, vars),
        Expr:: Xand(left, right) => evaluate(left, vars) && !evaluate(right, vars) || !evaluate(left, vars) && evaluate(right, vars),
        Expr:: Nand(left, right) => !(evaluate(left, vars) && evaluate(right, vars)),
        Expr:: Nor(left, right) => !(evaluate(left, vars) || evaluate(right, vars)),
        Expr:: Implies(left, right) => !evaluate(left, vars) || evaluate(right, vars),
        Expr:: Equiv(left, right) => evaluate(left, vars) == evaluate(right, vars),
        Expr:: ForAll(var, body) => {
            let mut m = vars.clone();
            m.insert(var.clone(), false);
            if !evaluate(body, &m) {
                return false;
            }
            m.insert(var.clone(), true);
            evaluate(body, &m)
        }
        Expr:: Exists(var, body) => {
            let mut m = vars.clone();
            m.insert(var.clone(), false);
            if evaluate(body, &m) {
                return true;
            }
            m.insert(var.clone(), true);
            evaluate(body, &m)
        }
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
fn build_ast(pair: pest:: iterators:: Pair<Rule>) -> Expr {
    let pair_str = pair.as_str();
    let rule = pair.as_rule();
    let mut inner = pair.into_inner();

    match rule {
        Rule:: program => build_ast(inner.next().unwrap()),

        Rule:: equivalence => {
            let left = build_ast(inner.next().unwrap());
            if let Some(right) = inner.next() {
                Expr:: Equiv(Box::new(left), Box::new(build_ast(right)))
            } else {
                left
            }
        }

        Rule:: implication => {
            let left = build_ast(inner.next().unwrap());
            if let Some(right) = inner.next() {
                Expr:: Implies(Box::new(left), Box::new(build_ast(right)))
            } else {
                left
            }
        }

        Rule:: or => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: Or(Box::new(l), Box::new(r)))
        }

        Rule:: xand => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: Xand(Box::new(l), Box::new(r)))
        }

        Rule:: xor => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: Xor(Box::new(l), Box::new(r)))
        }

        Rule:: nand => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: Nand(Box::new(l), Box::new(r)))
        }

        Rule:: nor => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: Nor(Box::new(l), Box::new(r)))
        }

        Rule:: and => {
            let mut parts = inner.map(|p| build_ast(p));
            let first = parts.next().unwrap();
            parts.fold(first, |l, r| Expr:: And(Box::new(l), Box::new(r)))
        }

        Rule:: not => {
            let first = build_ast(inner.next().unwrap());
            if pair_str.trim_start().to_uppercase().starts_with("NOT") {
                Expr:: Not(Box::new(first))
            } else {
                first
            }
        }

        Rule:: primary => inner.next().map_or(Expr:: Var(pair_str.to_string()), |p| build_ast(p)),

        Rule:: variable => Expr:: Var(pair_str.to_string()),

        Rule:: quantifier => {
            let s = pair_str.trim();
            let var = inner.next().unwrap().as_str().to_string();
            let body = build_ast(inner.next().unwrap());

            match s.chars().next() {
                Some('∀') => Expr:: ForAll(var, Box::new(body)),
                Some('∃') => Expr:: Exists(var, Box::new(body)),
                _ => panic!("Unexpected quantifier token: {:?}", s),
            }
        }

        Rule:: predicate => {
            let s = pair_str.trim();
            let name = s.split('(').next().unwrap().trim().to_string();
            let args = if let (Some(start), Some(end)) = (s.find('('), s.rfind(')')) {
                let inside = &s[start + 1..end].trim();
                if inside.is_empty() {
                    Vec::new()
                } else {
                    inside.split(',').map(|a| a.trim().to_string()).collect()
                }
            } else {
                Vec::new()
            };
            Expr:: Predicate(name, args)
        }

        _ => panic!("Unexpected rule: {:?}", rule),
    }
}
