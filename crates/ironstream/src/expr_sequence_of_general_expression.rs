// FILE: expr_sequence_of_general_expression.rs
// occt: Expr_SequenceOfGeneralExpression

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a handle to a GeneralExpression (simulated via Rc).
pub type GeneralExpressionHandle = Rc<RefCell<GeneralExpression>>;

/// A general expression node.
#[derive(Clone, Debug)]
pub struct GeneralExpression {
    // Placeholder for expression content
}

/// Deprecated: Sequence of handles to GeneralExpression.
/// Use SequenceOfGeneralExpression instead of directly using Vec<GeneralExpressionHandle>.
/// This is a newtype alias over a Vec for type safety, matching OCCT's deprecated typedef.
#[derive(Clone, Debug)]
pub struct SequenceOfGeneralExpression {
    items: Vec<GeneralExpressionHandle>,
}

impl SequenceOfGeneralExpression {
    /// Create an empty sequence.
    pub fn new() -> Self {
        SequenceOfGeneralExpression {
            items: Vec::new(),
        }
    }

    /// Append an expression handle to the sequence.
    pub fn append(&mut self, expr: GeneralExpressionHandle) {
        self.items.push(expr);
    }

    /// Return the number of items in the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the i-th item (1-indexed, like OCCT).
    pub fn at(&self, i: usize) -> Option<GeneralExpressionHandle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else {
            None
        }
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Iterate over the expressions.
    pub fn iter(&self) -> std::slice::Iter<GeneralExpressionHandle> {
        self.items.iter()
    }
}

impl Default for SequenceOfGeneralExpression {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        let seq = SequenceOfGeneralExpression::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_single() {
        let mut seq = SequenceOfGeneralExpression::new();
        let expr = Rc::new(RefCell::new(GeneralExpression {}));
        seq.append(expr.clone());

        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
        assert!(seq.at(1).is_some());
        assert!(seq.at(2).is_none());
    }

    #[test]
    fn test_append_multiple() {
        let mut seq = SequenceOfGeneralExpression::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        let e2 = Rc::new(RefCell::new(GeneralExpression {}));
        let e3 = Rc::new(RefCell::new(GeneralExpression {}));

        seq.append(e1);
        seq.append(e2);
        seq.append(e3);

        assert_eq!(seq.len(), 3);
        assert!(seq.at(1).is_some());
        assert!(seq.at(2).is_some());
        assert!(seq.at(3).is_some());
        assert!(seq.at(4).is_none());
    }

    #[test]
    fn test_clear() {
        let mut seq = SequenceOfGeneralExpression::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        seq.append(e1);

        assert_eq!(seq.len(), 1);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = SequenceOfGeneralExpression::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        let e2 = Rc::new(RefCell::new(GeneralExpression {}));

        seq.append(e1);
        seq.append(e2);

        let count = seq.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_clone() {
        let mut seq = SequenceOfGeneralExpression::new();
        let e1 = Rc::new(RefCell::new(GeneralExpression {}));
        seq.append(e1);

        let seq2 = seq.clone();
        assert_eq!(seq.len(), seq2.len());
    }
}
