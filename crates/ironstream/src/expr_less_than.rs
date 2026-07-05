// FILE: expr_less_than.rs
// occt: Expr_LessThan

#[derive(Debug, Clone)]
pub struct ExprLessThan {
    first: String,
    second: String,
}

impl ExprLessThan {
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self { first: exp1.into(), second: exp2.into() }
    }
    pub fn first(&self) -> &str { &self.first }
    pub fn second(&self) -> &str { &self.second }
    pub fn is_satisfied(&self) -> bool {
        if let (Ok(v1), Ok(v2)) = (self.first.parse::<f64>(), self.second.parse::<f64>()) {
            v1 < v2
        } else { false }
    }
    pub fn copy(&self) -> Self { Self { first: self.first.clone(), second: self.second.clone() } }
    pub fn string(&self) -> String { format!("{} < {}", self.first, self.second) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { assert!(ExprLessThan::new("3", "5").is_satisfied()); }
}
