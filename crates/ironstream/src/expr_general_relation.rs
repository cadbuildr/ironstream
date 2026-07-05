// FILE: expr_general_relation.rs
// occt: Expr_GeneralRelation

/// Abstract base trait defining the general purposes of any relation between expressions.
pub trait ExprGeneralRelation {
    /// Returns the current status of the relation
    fn is_satisfied(&self) -> bool;

    /// Tests if linear between its unknowns
    fn is_linear(&self) -> bool;

    /// Returns a GeneralRelation after simplification
    fn simplified(&self) -> Box<dyn ExprGeneralRelation>;

    /// Simplify in place
    fn simplify(&mut self);

    /// Returns a copy
    fn copy(&self) -> Box<dyn ExprGeneralRelation>;

    /// Returns the number of sub-relations
    fn nb_of_sub_relations(&self) -> usize;

    /// Returns the number of single relations
    fn nb_of_single_relations(&self) -> usize;

    /// Returns the sub-relation at the given index
    fn sub_relation(&self, index: usize) -> Option<Box<dyn ExprGeneralRelation>>;

    /// Tests if contains the given expression
    fn contains(&self, expr: &str) -> bool;

    /// Replace variable with expression
    fn replace(&mut self, var: &str, with: &str);

    /// Return string representation
    fn string(&self) -> String;
}

/// A simple concrete implementation
#[derive(Debug, Clone)]
pub struct SimpleRelation {
    left: String,
    operator: String,
    right: String,
}

impl SimpleRelation {
    /// Create a new relation
    pub fn new(left: impl Into<String>, op: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            left: left.into(),
            operator: op.into(),
            right: right.into(),
        }
    }

    /// Get left side
    pub fn left(&self) -> &str {
        &self.left
    }

    /// Get operator
    pub fn operator(&self) -> &str {
        &self.operator
    }

    /// Get right side
    pub fn right(&self) -> &str {
        &self.right
    }
}

impl ExprGeneralRelation for SimpleRelation {
    fn is_satisfied(&self) -> bool {
        // Simplified evaluation
        if let (Ok(l), Ok(r)) = (self.left.parse::<f64>(), self.right.parse::<f64>()) {
            match self.operator.as_str() {
                "=" => (l - r).abs() < 1e-10,
                "!=" => (l - r).abs() >= 1e-10,
                "<" => l < r,
                ">" => l > r,
                "<=" => l <= r,
                ">=" => l >= r,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_linear(&self) -> bool {
        true
    }

    fn simplified(&self) -> Box<dyn ExprGeneralRelation> {
        Box::new(self.clone())
    }

    fn simplify(&mut self) {
        // Simplified: do nothing
    }

    fn copy(&self) -> Box<dyn ExprGeneralRelation> {
        Box::new(self.clone())
    }

    fn nb_of_sub_relations(&self) -> usize {
        1
    }

    fn nb_of_single_relations(&self) -> usize {
        1
    }

    fn sub_relation(&self, index: usize) -> Option<Box<dyn ExprGeneralRelation>> {
        if index == 1 {
            Some(self.copy())
        } else {
            None
        }
    }

    fn contains(&self, expr: &str) -> bool {
        self.left.contains(expr) || self.right.contains(expr)
    }

    fn replace(&mut self, var: &str, with: &str) {
        self.left = self.left.replace(var, with);
        self.right = self.right.replace(var, with);
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left, self.operator, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_relation() {
        let rel = SimpleRelation::new("x", "=", "5");
        assert_eq!(rel.left(), "x");
        assert_eq!(rel.operator(), "=");
        assert_eq!(rel.right(), "5");
    }

    #[test]
    fn test_is_satisfied_equal() {
        let rel = SimpleRelation::new("5", "=", "5");
        assert!(rel.is_satisfied());

        let rel2 = SimpleRelation::new("5", "=", "3");
        assert!(!rel2.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_not_equal() {
        let rel = SimpleRelation::new("5", "!=", "3");
        assert!(rel.is_satisfied());

        let rel2 = SimpleRelation::new("5", "!=", "5");
        assert!(!rel2.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_less_than() {
        let rel = SimpleRelation::new("3", "<", "5");
        assert!(rel.is_satisfied());

        let rel2 = SimpleRelation::new("5", "<", "3");
        assert!(!rel2.is_satisfied());
    }

    #[test]
    fn test_is_satisfied_greater_than() {
        let rel = SimpleRelation::new("5", ">", "3");
        assert!(rel.is_satisfied());
    }

    #[test]
    fn test_is_linear() {
        let rel = SimpleRelation::new("x", "=", "y");
        assert!(rel.is_linear());
    }

    #[test]
    fn test_copy() {
        let rel1 = SimpleRelation::new("a", "=", "b");
        let rel2 = rel1.copy();
        assert_eq!(rel1.string(), rel2.string());
    }

    #[test]
    fn test_nb_of_sub_relations() {
        let rel = SimpleRelation::new("x", "=", "1");
        assert_eq!(rel.nb_of_sub_relations(), 1);
    }

    #[test]
    fn test_contains() {
        let rel = SimpleRelation::new("x + y", "=", "z");
        assert!(rel.contains("x"));
        assert!(rel.contains("z"));
        assert!(!rel.contains("w"));
    }

    #[test]
    fn test_replace() {
        let mut rel = SimpleRelation::new("x", "=", "2 * x");
        rel.replace("x", "y");
        assert_eq!(rel.left(), "y");
        assert!(rel.right().contains("y"));
    }

    #[test]
    fn test_string() {
        let rel = SimpleRelation::new("a", "<=", "b");
        assert_eq!(rel.string(), "a <= b");
    }
}
