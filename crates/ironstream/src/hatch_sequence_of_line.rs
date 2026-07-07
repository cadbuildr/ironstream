// FILE: hatch_sequence_of_line.rs
// occt: Hatch_SequenceOfLine

/// Represents a line in hatching.
#[derive(Clone, Debug, PartialEq)]
pub struct HatchLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

/// Sequence of Hatch_Line objects.
/// In OCCT, this was NCollection_Sequence<Hatch_Line>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct HatchSequenceOfLine {
    items: Vec<HatchLine>,
}

impl HatchSequenceOfLine {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        HatchSequenceOfLine { items: Vec::new() }
    }

    /// Returns the length of the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Appends an element to the end of the sequence.
    pub fn append(&mut self, value: HatchLine) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&HatchLine> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut HatchLine> {
        if index > 0 && index <= self.items.len() {
            self.items.get_mut(index - 1)
        } else {
            None
        }
    }

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<HatchLine> {
        self.items.iter()
    }
}

impl Default for HatchSequenceOfLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = HatchSequenceOfLine::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = HatchSequenceOfLine::new();
        let line = HatchLine { x1: 0.0, y1: 0.0, x2: 10.0, y2: 10.0 };
        seq.append(line);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = HatchSequenceOfLine::new();
        let line = HatchLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 };
        seq.append(line.clone());

        // 1-indexed access
        assert_eq!(seq.value(1), Some(&line));
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = HatchSequenceOfLine::new();
        let line = HatchLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 };
        seq.append(line);

        if let Some(val) = seq.value_mut(1) {
            val.x2 = 20.0;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.x2, 20.0);
    }

    #[test]
    fn test_clear() {
        let mut seq = HatchSequenceOfLine::new();
        seq.append(HatchLine { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 });
        seq.append(HatchLine { x1: 2.0, y1: 2.0, x2: 3.0, y2: 3.0 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = HatchSequenceOfLine::new();
        let line1 = HatchLine { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 };
        let line2 = HatchLine { x1: 2.0, y1: 2.0, x2: 3.0, y2: 3.0 };
        seq.append(line1.clone());
        seq.append(line2.clone());

        let mut iter = seq.iter();
        assert_eq!(iter.next(), Some(&line1));
        assert_eq!(iter.next(), Some(&line2));
        assert_eq!(iter.next(), None);
    }
}
