// FILE: expr_log_of10.rs
// occt: Expr_LogOf10

#[derive(Debug, Clone)]
pub struct ExprLogOf10 {
    operand: String,
}

impl ExprLogOf10 {
    pub fn new(expr: impl Into<String>) -> Self { Self { operand: expr.into() } }
    pub fn operand(&self) -> &str { &self.operand }
    pub fn copy(&self) -> Self { Self { operand: self.operand.clone() } }
    pub fn is_identical(&self, other: &ExprLogOf10) -> bool { self.operand == other.operand }
    pub fn is_linear(&self) -> bool { false }
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        if let Ok(v) = self.operand.parse::<f64>() { return Ok(v.log10()); }
        for (i, v) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *v { return Ok(vals[i].log10()); }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }
    pub fn string(&self) -> String { format!("Log10({})", self.operand) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { assert!((ExprLogOf10::new("10").evaluate(&[], &[]).unwrap() - 1.0).abs() < 1e-10); }
}
