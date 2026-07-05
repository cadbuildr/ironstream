// FILE: t_short_sequence_of_short_real.rs
// occt: TShort_SequenceOfShortReal

/// TShort_SequenceOfShortReal: Sequence of float values.
///
/// Deprecated OCCT typedef for backward compatibility.
/// Equivalent to NCollection_Sequence<float> with 1-based indexing.
#[derive(Clone, Debug)]
pub struct TShortSequenceOfShortReal {
    data: Vec<f32>,
}

impl TShortSequenceOfShortReal {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TShortSequenceOfShortReal { data: Vec::new() }
    }

    /// Appends a value at the end of the sequence.
    pub fn append(&mut self, value: f32) {
        self.data.push(value);
    }

    /// Prepends a value at the beginning of the sequence.
    pub fn prepend(&mut self, value: f32) {
        self.data.insert(0, value);
    }

    /// Returns the length of the sequence.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Returns the value at 1-based index (OCCT semantics).
    /// Panics if index is out of bounds.
    pub fn value(&self, index: usize) -> f32 {
        if index == 0 {
            panic!("Index 0 is invalid; OCCT uses 1-based indexing");
        }
        self.data[index - 1]
    }

    /// Sets the value at 1-based index (OCCT semantics).
    /// Panics if index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: f32) {
        if index == 0 {
            panic!("Index 0 is invalid; OCCT uses 1-based indexing");
        }
        self.data[index - 1] = value;
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes all elements.
    pub fn empty(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the values (0-based for Rust iteration).
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.data.iter()
    }

    /// Returns the lower bound (always 1 in OCCT).
    pub fn lower(&self) -> usize {
        1
    }

    /// Returns the upper bound (same as length, 1-based).
    pub fn upper(&self) -> usize {
        self.data.len()
    }
}

impl Default for TShortSequenceOfShortReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let seq = TShortSequenceOfShortReal::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(1.5);
        seq.append(2.5);
        seq.append(3.5);
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(1), 1.5);
        assert_eq!(seq.value(2), 2.5);
        assert_eq!(seq.value(3), 3.5);
    }

    #[test]
    fn test_prepend() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(2.0);
        seq.prepend(1.0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), 1.0);
        assert_eq!(seq.value(2), 2.0);
    }

    #[test]
    fn test_set_value() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(1.0);
        seq.append(2.0);
        seq.set_value(1, 10.0);
        assert_eq!(seq.value(1), 10.0);
        assert_eq!(seq.value(2), 2.0);
    }

    #[test]
    fn test_clear() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(1.0);
        seq.append(2.0);
        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_bounds() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(1.0);
        seq.append(2.0);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut seq = TShortSequenceOfShortReal::new();
        seq.append(1.5);
        seq.append(2.5);
        seq.append(3.5);
        let values: Vec<f32> = seq.iter().copied().collect();
        assert_eq!(values, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    #[should_panic(expected = "Index 0 is invalid")]
    fn test_value_zero_index_panics() {
        let seq = TShortSequenceOfShortReal::new();
        let _ = seq.value(0);
    }
}
