// FILE: expr_equal.rs
// occt: Expr_Equal

/// Represents an equality relation: exp1 = exp2.
#[derive(Debug, Clone)]
pub struct ExprEqual {
    first: String,
    second: String,
}

impl ExprEqual {
    /// Create the relation exp1 = exp2
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            first: exp1.into(),
            second: exp2.into(),
        }
    }

    /// Get the left side
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Get the right side
    pub fn second(&self) -> &str {
        &self.second
    }

    /// Check if the relation is satisfied.
    /// OCCT semantics: Expr_Equal::IsSatisfied simplifies both members and
    /// compares with IsIdentical; for numeric values this is an exact
    /// comparison (Expr_NumericValue::IsIdentical uses `==`, no tolerance).
    pub fn is_satisfied(&self) -> bool {
        if let (Ok(val1), Ok(val2)) = (self.first.parse::<f64>(), self.second.parse::<f64>()) {
            return val1 == val2;
        }
        // If not numeric, assume not satisfied without evaluation
        false
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
        format!("{} = {}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_equal() {
        let rel = ExprEqual::new("5", "5");
        assert_eq!(rel.first(), "5");
        assert_eq!(rel.second(), "5");
    }

    #[test]
    fn test_is_satisfied_true() {
        let rel = ExprEqual::new("5", "5");
        assert!(rel.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_false() {
        let rel = ExprEqual::new("5", "3");
        assert!(!rel.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_floats() {
        let rel1 = ExprEqual::new("1.0", "1.0");
        assert!(rel1.is_satisfied());

        let rel2 = ExprEqual::new("1.0", "1.5");
        assert!(!rel2.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_near_equal() {
        // OCCT compares numeric values exactly (Expr_NumericValue::IsIdentical
        // uses `==`), so nearly-equal values do NOT satisfy the relation.
        let rel = ExprEqual::new("1.0", "1.0000000001");
        assert!(!rel.is_satisfied());
    }

    #[test]
    fn test_simplified() {
        let rel1 = ExprEqual::new("a", "b");
        let rel2 = rel1.simplified();
        assert_eq!(rel1.first(), rel2.first());
        assert_eq!(rel1.second(), rel2.second());
    }

    #[test]
    fn test_copy() {
        let rel1 = ExprEqual::new("x", "y");
        let rel2 = rel1.copy();
        assert_eq!(rel1.first(), rel2.first());
        assert_eq!(rel1.second(), rel2.second());
    }

    #[test]
    fn test_string_representation() {
        let rel = ExprEqual::new("a", "b");
        assert_eq!(rel.string(), "a = b");
    }
}
