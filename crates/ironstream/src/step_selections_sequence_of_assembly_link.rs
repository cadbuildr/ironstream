// FILE: step_selections_sequence_of_assembly_link.rs
// occt: STEPSelections_SequenceOfAssemblyLink, STEPSelections_SequenceIteratorOfAssemblyLink

/// Deprecated typedef for backward compatibility.
/// A sequence of assembly links using 1-based indexing (OCCT semantics).
/// Corresponds to NCollection_Sequence<opencascade::handle<STEPSelections_AssemblyLink>>
pub struct StepSelectionsSequenceOfAssemblyLink {
    data: Vec<Option<String>>,
}

impl StepSelectionsSequenceOfAssemblyLink {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        StepSelectionsSequenceOfAssemblyLink {
            data: vec![None],
        }
    }

    /// Append an element.
    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    /// Prepend an element.
    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
    }

    /// Get value at 1-based index.
    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index > 0 && index < self.data.len() {
            self.data[index] = Some(value);
            true
        } else {
            false
        }
    }

    /// Get length.
    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove element at index.
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index > 0 && index < self.data.len() {
            self.data.remove(index)
        } else {
            None
        }
    }

    /// Clear.
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(None);
    }

    /// Get lower bound.
    pub fn lower(&self) -> usize {
        1
    }

    /// Get upper bound.
    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
}

impl Default for StepSelectionsSequenceOfAssemblyLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_operations() {
        let mut seq = StepSelectionsSequenceOfAssemblyLink::new();
        assert!(seq.is_empty());

        seq.append("link1".to_string());
        assert_eq!(seq.len(), 1);
        assert_eq!(seq.value(1), Some(&"link1".to_string()));
    }

    #[test]
    fn test_1based_indexing() {
        let mut seq = StepSelectionsSequenceOfAssemblyLink::new();
        seq.append("first".to_string());
        seq.append("second".to_string());

        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 2);
        assert_eq!(seq.value(1), Some(&"first".to_string()));
        assert_eq!(seq.value(2), Some(&"second".to_string()));
    }

    #[test]
    fn test_set_and_remove() {
        let mut seq = StepSelectionsSequenceOfAssemblyLink::new();
        seq.append("a".to_string());
        seq.append("b".to_string());

        assert!(seq.set_value(1, "changed".to_string()));
        assert_eq!(seq.value(1), Some(&"changed".to_string()));

        let removed = seq.remove(2);
        assert_eq!(removed, Some("b".to_string()));
        assert_eq!(seq.len(), 1);
    }
}
