// FILE: expr_named_function.rs
// occt: Expr_NamedFunction

#[derive(Debug, Clone)]
pub struct ExprNamedFunction {
    name: String,
    variables: Vec<String>,
}

impl ExprNamedFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), variables: Vec::new() }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn add_variable(&mut self, v: impl Into<String>) { self.variables.push(v.into()); }
    pub fn copy(&self) -> Self {
        Self { name: self.name.clone(), variables: self.variables.clone() }
    }
    pub fn string(&self) -> String { self.name.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let f = ExprNamedFunction::new("myfunc");
        assert_eq!(f.name(), "myfunc");
    }
}
