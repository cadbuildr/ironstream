// FILE: expr_product.rs
// occt: Expr_Product

#[derive(Debug, Clone)]
pub struct ExprProduct {
    first: String,
    second: String,
}

impl ExprProduct {
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self { first: exp1.into(), second: exp2.into() }
    }
    pub fn first(&self) -> &str { &self.first }
    pub fn second(&self) -> &str { &self.second }
    pub fn shallow_simplified(&self) -> Self {
        Self { first: self.first.clone(), second: self.second.clone() }
    }
    pub fn copy(&self) -> Self { Self { first: self.first.clone(), second: self.second.clone() } }
    pub fn is_identical(&self, other: &ExprProduct) -> bool {
        self.first == other.first && self.second == other.second
    }
    pub fn is_linear(&self) -> bool { false }
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        let v1 = if let Ok(n) = self.first.parse::<f64>() { n }
            else { let mut f = false; let mut r = 0.0;
                for (i, v) in vars.iter().enumerate() {
                    if i < vals.len() && self.first == *v { r = vals[i]; f = true; break; }
                }
                if !f { return Err(format!("Unknown: {}", self.first)); } r
            };
        let v2 = if let Ok(n) = self.second.parse::<f64>() { n }
            else { let mut f = false; let mut r = 0.0;
                for (i, v) in vars.iter().enumerate() {
                    if i < vals.len() && self.second == *v { r = vals[i]; f = true; break; }
                }
                if !f { return Err(format!("Unknown: {}", self.second)); } r
            };
        Ok(v1 * v2)
    }
    pub fn string(&self) -> String { format!("({} * {})", self.first, self.second) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let p = ExprProduct::new("3", "4");
        assert_eq!(p.evaluate(&[], &[]).unwrap(), 12.0);
    }
}
