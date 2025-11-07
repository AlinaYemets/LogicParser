use logic_parser:: {parse_expression, evaluate, Expr};
use std:: collections:: HashMap;
use std:: env;
use std:: fs;
use std:: io:: {self, Write};

/// Collect vars and free predicates
fn collect_ent(expr: &Expr, vars: &mut Vec<String>, preds: &mut Vec<String>, under_quant: bool) {
    match expr {
        Expr:: Var(name) if !vars.contains(name) => vars.push(name.clone()),
        Expr:: Predicate(name, _) if !preds.contains(name) && !under_quant => preds.push(name.clone()),
        Expr:: Not(e) => collect_ent(e, vars, preds, under_quant),
        Expr:: And(l, r) | Expr:: Or(l, r) | Expr:: Implies(l, r) | Expr:: Equiv(l, r) => {
            collect_ent(l, vars, preds, under_quant);
            collect_ent(r, vars, preds, under_quant);
        }
        Expr:: ForAll(_, inner) | Expr:: Exists(_, inner) => collect_ent(inner, vars, preds, true),
        _ => {}
    }
}

/// Read a boolean value from the user
fn read_bool(prompt: &str) -> bool {
    loop {
        print!("{prompt}");
        io:: stdout().flush().ok();
        let mut input = String::new();
        if io:: stdin().read_line(&mut input).is_err() { continue; }
        match input.trim().to_lowercase().as_str() {
            "" | "false" | "f" | "0" => return false,
            "true" | "t" | "1" => return true,
            _ => println!("Enter true/false (Enter = false)."),
        }
    }
}

/// Parse expressions from a file and evaluate them
fn parse_file(filename: &str) {
    let content = fs:: read_to_string(filename).expect("Failed to read the file");

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }

        println!("---\nLine {}: {}", i + 1, line);
        match parse_expression(line) {
            Ok(ast) => {
                let mut vars = Vec::new();
                let mut preds = Vec::new();
                collect_ent(&ast, &mut vars, &mut preds, false);

                let mut values: HashMap<String, bool> = HashMap::new();

                for v in vars.iter().chain(preds.iter()) {
                    let prompt = if preds.contains(v) { format!("{v}() = (true/false, Enter = false): ") }
                                 else { format!("{v} = (true/false, Enter = false): ") };
                    values.insert(v.clone(), read_bool(&prompt));
                }

                println!("Values: {:?}", values);
                println!("Result: {}", evaluate(&ast, &values));
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}

/// Display help information
fn show_help() {
    println!("Logic Parser CLI");
    println!("Usage:");
    println!("  parse <filename>   Parse and evaluate expressions from a file");
    println!("  help               Show this help message");
    println!("  credits            Show author and project information");
}

/// Display credits information
fn show_credits() {
    println!("Logic Parser");
    println!("Author: Alina Yemets");
    println!("Project: Logic expression parser and evaluator in Rust");
}

fn main() {
    let args: Vec<String> = env:: args().collect();

    if args.len() < 2 {
        println!("No command provided. Use 'help' for usage.");
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Filename not provided. Usage: parse <filename>");
            } else {
                parse_file(&args[2]);
            }
        }
        "help" => show_help(),
        "credits" => show_credits(),
        _ => eprintln!("Unknown command. Use 'help' for usage."),
    }
}