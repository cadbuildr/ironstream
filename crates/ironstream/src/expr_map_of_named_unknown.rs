// FILE: expr_map_of_named_unknown.rs
// occt: Expr_MapOfNamedUnknown

use std::collections::HashMap;

pub struct ExprMapOfNamedUnknown {
    data: HashMap<String, String>,
}

impl ExprMapOfNamedUnknown {
    pub fn new() -> Self { Self { data: HashMap::new() } }
    pub fn insert(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.data.insert(k.into(), v.into());
    }
    pub fn get(&self, k: &str) -> Option<&str> { self.data.get(k).map(|s| s.as_str()) }
    pub fn len(&self) -> usize { self.data.len() }
}

impl Default for ExprMapOfNamedUnknown {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut m = ExprMapOfNamedUnknown::new();
        m.insert("x", "1.0");
        assert_eq!(m.get("x"), Some("1.0"));
    }
}
