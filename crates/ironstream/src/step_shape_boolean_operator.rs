// FILE: step_shape_boolean_operator.rs
// occt: StepShape_BooleanOperator

/// Enumeration for boolean operators
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BooleanOperator {
    /// Difference operation (A - B)
    Difference,
    /// Intersection operation (A ∩ B)
    Intersection,
    /// Union operation (A ∪ B)
    Union,
}

impl BooleanOperator {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            BooleanOperator::Difference => "Difference",
            BooleanOperator::Intersection => "Intersection",
            BooleanOperator::Union => "Union",
        }
    }

    /// Try to parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Difference" => Some(BooleanOperator::Difference),
            "Intersection" => Some(BooleanOperator::Intersection),
            "Union" => Some(BooleanOperator::Union),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_operator_difference() {
        assert_eq!(BooleanOperator::Difference.as_str(), "Difference");
    }

    #[test]
    fn test_boolean_operator_intersection() {
        assert_eq!(BooleanOperator::Intersection.as_str(), "Intersection");
    }

    #[test]
    fn test_boolean_operator_union() {
        assert_eq!(BooleanOperator::Union.as_str(), "Union");
    }

    #[test]
    fn test_from_str_difference() {
        assert_eq!(
            BooleanOperator::from_str("Difference"),
            Some(BooleanOperator::Difference)
        );
    }

    #[test]
    fn test_from_str_intersection() {
        assert_eq!(
            BooleanOperator::from_str("Intersection"),
            Some(BooleanOperator::Intersection)
        );
    }

    #[test]
    fn test_from_str_union() {
        assert_eq!(
            BooleanOperator::from_str("Union"),
            Some(BooleanOperator::Union)
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(BooleanOperator::from_str("Invalid"), None);
    }

    #[test]
    fn test_equality() {
        assert_eq!(BooleanOperator::Union, BooleanOperator::Union);
        assert_ne!(BooleanOperator::Union, BooleanOperator::Difference);
    }
}
