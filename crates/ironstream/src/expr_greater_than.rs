// FILE: expr_greater_than.rs
// occt: Expr_GreaterThan

/// Represents a greater-than relation: exp1 > exp2.
#[derive(Debug, Clone)]
pub struct ExprGreaterThan {
    first: String,
    second: String,
}

impl ExprGreaterThan {
    /// Create the relation exp1 > exp2
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

    /// Check if satisfied
    pub fn is_satisfied(&self) -> bool {
        if let (Ok(val1), Ok(val2)) = (self.first.parse::<f64>(), self.second.parse::<f64>()) {
            return val1 > val2;
        }
        false
    }

    /// Simplified
    pub fn simplified(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// Simplify in-place
    pub fn simplify(&mut self) {}

    /// Copy
    pub fn copy(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// String representation
    pub fn string(&self) -> String {
        format!("{} > {}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let rel = ExprGreaterThan::new("5", "3");
        assert_eq!(rel.first(), "5");
        assert_eq!(rel.second(), "3");
    }

    #[test]
    fn test_is_satisfied() {
        assert!(ExprGreaterThan::new("5", "3").is_satisfied());
        assert!(!ExprGreaterThan::new("3", "5").is_satisfied());
        assert!(!ExprGreaterThan::new("5", "5").is_satisfied());
    }

    #[test]
    fn test_copy() {
        let rel1 = ExprGreaterThan::new("x", "y");
        let rel2 = rel1.copy();
        assert_eq!(rel1.first(), rel2.first());
    }

    #[test]
    fn test_string() {
        let rel = ExprGreaterThan::new("a", "b");
        assert_eq!(rel.string(), "a > b");
    }
}
