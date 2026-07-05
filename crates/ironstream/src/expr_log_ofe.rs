// FILE: expr_log_ofe.rs
// occt: Expr_LogOfe

#[derive(Debug, Clone)]
pub struct ExprLogOfe {
    operand: String,
}

impl ExprLogOfe {
    pub fn new(expr: impl Into<String>) -> Self { Self { operand: expr.into() } }
    pub fn operand(&self) -> &str { &self.operand }
    pub fn copy(&self) -> Self { Self { operand: self.operand.clone() } }
    pub fn is_identical(&self, other: &ExprLogOfe) -> bool { self.operand == other.operand }
    pub fn is_linear(&self) -> bool { false }
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        if let Ok(v) = self.operand.parse::<f64>() { return Ok(v.ln()); }
        for (i, v) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *v { return Ok(vals[i].ln()); }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }
    pub fn string(&self) -> String { format!("Ln({})", self.operand) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { assert!((ExprLogOfe::new("1").evaluate(&[], &[]).unwrap() - 0.0).abs() < 1e-10); }
}
