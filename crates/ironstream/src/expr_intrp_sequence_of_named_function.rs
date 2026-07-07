// FILE: expr_intrp_sequence_of_named_function.rs
// occt: ExprIntrp_SequenceOfNamedFunction

use std::collections::VecDeque;

/// Deprecated typedef for backward compatibility.
/// A sequence of NamedFunction handles, implemented as a VecDeque.
pub struct ExprIntrpSequenceOfNamedFunction {
    inner: VecDeque<String>,
}

impl ExprIntrpSequenceOfNamedFunction {
    /// Create a new empty sequence
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    /// Append a named function to the sequence
    pub fn append(&mut self, name: String) {
        self.inner.push_back(name);
    }

    /// Return the length of the sequence
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Return true if sequence is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear the sequence
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Get item at index (1-based in OCCT)
    pub fn value(&self, index: usize) -> Option<&str> {
        if index == 0 {
            return None;
        }
        self.inner.get(index - 1).map(|s| s.as_str())
    }
}

impl Default for ExprIntrpSequenceOfNamedFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_sequence() {
        let seq = ExprIntrpSequenceOfNamedFunction::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_length() {
        let mut seq = ExprIntrpSequenceOfNamedFunction::new();
        seq.append("func1".to_string());
        seq.append("func2".to_string());
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_value_access() {
        let mut seq = ExprIntrpSequenceOfNamedFunction::new();
        seq.append("first".to_string());
        seq.append("second".to_string());
        assert_eq!(seq.value(1), Some("first"));
        assert_eq!(seq.value(2), Some("second"));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_clear_sequence() {
        let mut seq = ExprIntrpSequenceOfNamedFunction::new();
        seq.append("test".to_string());
        assert_eq!(seq.len(), 1);
        seq.clear();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }
}
