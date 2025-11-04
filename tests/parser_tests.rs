use anyhow:: Result;
use logic_parser:: {evaluate, parse_expression};
use std:: collections:: HashMap;

#[test]
fn test_variable_only() -> Result<()> {
    let expr = parse_expression("A32")?;
    let mut vars = HashMap::new();
    vars.insert("A32".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_not_eval() -> Result<()> {
    let expr = parse_expression("NOT A")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_and_eval() -> Result<()> {
    let expr = parse_expression("A AND B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_or_eval() -> Result<()> {
    let expr = parse_expression("A OR B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), false);
    vars.insert("B".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_implies_eval() -> Result<()> {
    let expr = parse_expression("A -> B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_parentheses_eval() -> Result<()> {
    let expr = parse_expression("(A AND B) OR C")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    vars.insert("C".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_complex_expression() -> Result<()> {
    let expr = parse_expression("(A -> B) AND NOT C")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    vars.insert("C".into(), false);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_multiple_and_or() -> Result<()> {
    let expr = parse_expression("A AND B OR C AND NOT D")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    vars.insert("C".into(), true);
    vars.insert("D".into(), false);
    assert_eq!(evaluate(&expr, &vars), true); // (A AND B) OR (C AND NOT D) = false OR true = true
    Ok(())
}

#[test]
fn test_nested_implication() -> Result<()> {
    let expr = parse_expression("A -> (B -> C)")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    vars.insert("C".into(), false);
    assert_eq!(evaluate(&expr, &vars), false); // true -> (true -> false) = true -> false = false
    Ok(())
}

#[test]
fn test_full_program() -> Result<()> {
    let expr = parse_expression("NOT (A AND (B OR C)) -> D")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    vars.insert("C".into(), false);
    vars.insert("D".into(), true);
    assert_eq!(evaluate(&expr, &vars), true); // NOT (true and false) -> true = true
    Ok(())
}
