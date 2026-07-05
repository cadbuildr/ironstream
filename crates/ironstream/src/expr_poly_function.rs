// FILE: expr_poly_function.rs
// occt: Expr_PolyFunction

#[derive(Debug, Clone)]
pub struct ExprPolyFunction {
    name: String,
    variables: Vec<String>,
}

impl ExprPolyFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), variables: Vec::new() }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn add_variable(&mut self, v: impl Into<String>) { self.variables.push(v.into()); }
    pub fn variables(&self) -> &[String] { &self.variables }
    pub fn copy(&self) -> Self {
        Self { name: self.name.clone(), variables: self.variables.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut f = ExprPolyFunction::new("polyf");
        f.add_variable("x");
        assert_eq!(f.name(), "polyf");
    }
}
