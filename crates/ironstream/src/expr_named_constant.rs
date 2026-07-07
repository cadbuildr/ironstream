// FILE: expr_named_constant.rs
// occt: Expr_NamedConstant

#[derive(Debug, Clone)]
pub struct ExprNamedConstant {
    name: String,
    value: f64,
}

impl ExprNamedConstant {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self { name: name.into(), value }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> f64 { self.value }
    pub fn copy(&self) -> Self { Self { name: self.name.clone(), value: self.value } }
    pub fn string(&self) -> String { format!("{}({})", self.name, self.value) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let c = ExprNamedConstant::new("pi", 3.14159);
        assert_eq!(c.name(), "pi");
        assert!((c.value() - 3.14159).abs() < 1e-5);
    }
}
