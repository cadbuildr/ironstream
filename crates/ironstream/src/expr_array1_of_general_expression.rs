// FILE: expr_array1_of_general_expression.rs
// occt: Expr_Array1OfGeneralExpression

/// Deprecated typedef for backward compatibility.
/// A 1D array of GeneralExpression handles, implemented as a Vec.
pub struct ExprArray1OfGeneralExpression {
    data: Vec<String>,
    lower: usize,
}

impl ExprArray1OfGeneralExpression {
    /// Create a new array with given lower bound and size
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower {
            upper - lower + 1
        } else {
            0
        };
        Self {
            data: vec![String::new(); size],
            lower,
        }
    }

    /// Get the lower bound
    pub fn lower_bound(&self) -> usize {
        self.lower
    }

    /// Get the upper bound
    pub fn upper_bound(&self) -> usize {
        if self.data.is_empty() {
            self.lower.saturating_sub(1)
        } else {
            self.lower + self.data.len() - 1
        }
    }

    /// Get the length
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Get value at index
    pub fn value(&self, index: usize) -> Option<&str> {
        if index >= self.lower && index <= self.upper_bound() {
            self.data.get(index - self.lower).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Set value at index
    pub fn set_value(&mut self, index: usize, val: impl Into<String>) {
        if index >= self.lower && index <= self.upper_bound() {
            if let Some(elem) = self.data.get_mut(index - self.lower) {
                *elem = val.into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array() {
        let arr = ExprArray1OfGeneralExpression::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_value_access() {
        let mut arr = ExprArray1OfGeneralExpression::new(1, 3);
        arr.set_value(1, "expr1");
        arr.set_value(2, "expr2");
        arr.set_value(3, "expr3");
        assert_eq!(arr.value(1), Some("expr1"));
        assert_eq!(arr.value(2), Some("expr2"));
        assert_eq!(arr.value(3), Some("expr3"));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_zero_based_array() {
        let arr = ExprArray1OfGeneralExpression::new(0, 2);
        assert_eq!(arr.lower_bound(), 0);
        assert_eq!(arr.upper_bound(), 2);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_empty_array() {
        let arr = ExprArray1OfGeneralExpression::new(5, 4);
        assert_eq!(arr.length(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = ExprArray1OfGeneralExpression::new(10, 12);
        arr.set_value(10, "a");
        arr.set_value(11, "b");
        arr.set_value(12, "c");
        assert_eq!(arr.value(10), Some("a"));
        assert_eq!(arr.value(11), Some("b"));
        assert_eq!(arr.value(12), Some("c"));
    }
}
