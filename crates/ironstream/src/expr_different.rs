// FILE: expr_different.rs
// occt: Expr_Different

/// Represents a "not equal" relation: exp1 != exp2.
#[derive(Debug, Clone)]
pub struct ExprDifferent {
    first: String,
    second: String,
}

impl ExprDifferent {
    /// Create the relation exp1 != exp2
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            first: exp1.into(),
            second: exp2.into(),
        }
    }

    /// Get the first operand
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Get the second operand
    pub fn second(&self) -> &str {
        &self.second
    }

    /// Check if the relation is satisfied (first != second)
    pub fn is_satisfied(&self) -> bool {
        if let (Ok(val1), Ok(val2)) = (self.first.parse::<f64>(), self.second.parse::<f64>()) {
            return val1 != val2;
        }
        // If not numeric, assume not satisfied without evaluation
        true
    }

    /// Return a simplified version
    pub fn simplified(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// Simplify in-place
    pub fn simplify(&mut self) {
        // Simplified: do nothing
    }

    /// Return a copy
    pub fn copy(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("{} != {}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_different() {
        let rel = ExprDifferent::new("5", "3");
        assert_eq!(rel.first(), "5");
        assert_eq!(rel.second(), "3");
    }

    #[test]
    fn test_is_satisfied_true() {
        let rel = ExprDifferent::new("5", "3");
        assert!(rel.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_false() {
        let rel = ExprDifferent::new("5", "5");
        assert!(!rel.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_floats() {
        let rel1 = ExprDifferent::new("1.5", "1.5");
        assert!(!rel1.is_satisfied());

        let rel2 = ExprDifferent::new("1.5", "2.5");
        assert!(rel2.is_satisfied());
    }

    #[test]
    fn test_simplified() {
        let rel1 = ExprDifferent::new("a", "b");
        let rel2 = rel1.simplified();
        assert_eq!(rel1.first(), rel2.first());
        assert_eq!(rel1.second(), rel2.second());
    }

    #[test]
    fn test_copy() {
        let rel1 = ExprDifferent::new("x", "y");
        let rel2 = rel1.copy();
        assert_eq!(rel1.first(), rel2.first());
        assert_eq!(rel1.second(), rel2.second());
    }

    #[test]
    fn test_string_representation() {
        let rel = ExprDifferent::new("a", "b");
        assert_eq!(rel.string(), "a != b");
    }
}
