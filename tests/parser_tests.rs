use anyhow::Result;
use logic_parser::{evaluate, parse_expression};
use std::collections::HashMap;

#[test]
fn test_var_only() -> Result<()> {
    let expr = parse_expression("A32")?;
    let mut vars = HashMap::new();
    vars.insert("A32".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_pred_no_args() -> Result<()> {
    let expr = parse_expression("P()")?;
    let mut vars = HashMap::new();
    vars.insert("P".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_pred_with_args() -> Result<()> {
    let expr = parse_expression("Friend(alice,bob)")?;
    let mut vars = HashMap::new();
    vars.insert("Friend:alice,bob".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_not() -> Result<()> {
    let expr = parse_expression("NOT A")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_and() -> Result<()> {
    let expr = parse_expression("A AND B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_or() -> Result<()> {
    let expr = parse_expression("A OR B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), false);
    vars.insert("B".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_precedence_and_or() -> Result<()> {
    // (A AND B) OR C  -> ensure grouping works via grammar precedence
    let expr = parse_expression("A AND B OR C")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    vars.insert("C".into(), true);
    assert_eq!(evaluate(&expr, &vars), true); // false OR true = true
    Ok(())
}

#[test]
fn test_implication() -> Result<()> {
    let expr = parse_expression("A -> B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_implication_right_associative() -> Result<()> {
    // A -> (B -> C)  is parsed/handled as right-associative
    let expr = parse_expression("A -> B -> C")?; // equivalent to A -> (B -> C)
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    vars.insert("C".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_equiv() -> Result<()> {
    let expr = parse_expression("A <-> B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), false); // true vs false -> not equivalent
    Ok(())
}

#[test]
fn test_equiv_with_implication() -> Result<()> {
    let expr = parse_expression("A <-> (B -> C)")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    vars.insert("C".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);  // B -> C = true, A = true -> equivalence true
    Ok(())
}

#[test]
fn test_quant_forall() -> Result<()> {
    // ∀x (x OR NOT x) is a tautology 
    let expr = parse_expression("∀x (x OR NOT x)")?;
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_quant_forall_var() -> Result<()> {
    // ∀x x  should be false (since x=false case fails)
    let expr = parse_expression("∀x x")?;
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_quant_exists_var() -> Result<()> {
    // ∃x x  should be true (x=true makes it true)
    let expr = parse_expression("∃x x")?;
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_comb_quant_all_implication() -> Result<()> {
    // ∀x (x -> A)  with A = true  => both cases true -> result true
    let expr = parse_expression("∀x (x -> A)")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_comb_quant_exists_and() -> Result<()> {
    // ∃x (x AND A)  with A = true => exists x=true gives true; with A=false -> false
    let expr = parse_expression("∃x (x AND A)")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    assert_eq!(evaluate(&expr, &vars), true);
    vars.insert("A".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_complex_mixed_expr() -> Result<()> {
    // mixed: Friend(alice,bob) AND (A <-> B) OR NOT P()
    let expr = parse_expression("Friend(alice,bob) AND (A <-> B) OR NOT P()")?;
    let mut vars = HashMap::new();
    vars.insert("Friend:alice,bob".into(), true);
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    vars.insert("P".into(), false);
    // Friend true AND (A<->B true) OR NOT P() = true AND true OR true = true
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_xor_op() -> Result<()> {
    let expr = parse_expression("A XOR B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_xand_op() -> Result<()> {
    let expr = parse_expression("A XAND B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), true);
    Ok(())
}

#[test]
fn test_nand_op() -> Result<()> {
    let expr = parse_expression("A NAND B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), true);
    assert_eq!(evaluate(&expr, &vars), false);
    Ok(())
}

#[test]
fn test_nor_op() -> Result<()> {
    let expr = parse_expression("A NOR B")?;
    let mut vars = HashMap::new();
    vars.insert("A".into(), true);
    vars.insert("B".into(), false);
    assert_eq!(evaluate(&expr, &vars), false);

    Ok(())
}

