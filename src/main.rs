use logic_parser:: {parse_expression, evaluate, Expr};
use std:: collections:: HashMap;
use std:: env;
use std:: fs;
use std:: io:: {self, Write};

/// Collect all variable names from an expression
fn collect_vars(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr:: Var(name) => {
            if !vars.contains(name) {
                vars.push(name.clone());
            }
        } // checks to avoid dupl
        Expr:: Not(e) => collect_vars(e, vars),
        Expr:: And(l, r) | Expr:: Or(l, r) | Expr:: Implies(l, r) => {
            collect_vars(l, vars);
            collect_vars(r, vars);
        }
    }
}

/// Read a boolean value from the user
fn read_bool(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
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
                collect_vars(&ast, &mut vars);

                let mut values: HashMap<String, bool> = HashMap::new();
                for v in &vars {
                    let prompt = format!("{} = (true/false, Enter = false): ", v);
                    values.insert(v.clone(), read_bool(&prompt));
                }

                println!("Values: {:?}", values);
                let result = evaluate(&ast, &values);
                println!("Result: {}", result);
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
