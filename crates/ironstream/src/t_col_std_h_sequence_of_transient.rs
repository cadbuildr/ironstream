// FILE: t_col_std_h_sequence_of_transient.rs
// occt: TColStd_HSequenceOfTransient

/// TColStd_HSequenceOfTransient is a deprecated alias for a handle (heap-allocated) sequence of transient objects.
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColStdHSequenceOfTransient {
    data: Vec<Option<String>>,
}

impl TColStdHSequenceOfTransient {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColStdHSequenceOfTransient {
            data: Vec::new(),
        }
    }

    /// Appends an element to the sequence.
    pub fn append(&mut self, value: Option<String>) {
        self.data.push(value);
    }

    /// Prepends an element to the sequence.
    pub fn prepend(&mut self, value: Option<String>) {
        self.data.insert(0, value);
    }

    /// Inserts an element at the given 1-based index.
    pub fn insert_before(&mut self, idx: i32, value: Option<String>) {
        if idx < 1 || idx as usize > self.data.len() + 1 {
            panic!("Insert index {} out of range [1, {}]", idx, self.data.len() + 1);
        }
        self.data.insert((idx - 1) as usize, value);
    }

    /// Removes an element at the given 1-based index.
    pub fn remove(&mut self, idx: i32) -> Option<Option<String>> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(self.data.remove((idx - 1) as usize))
    }

    /// Returns the length of the sequence.
    pub fn length(&self) -> i32 {
        self.data.len() as i32
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&Option<String>> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&self.data[(idx - 1) as usize])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut Option<String>> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&mut self.data[(idx - 1) as usize])
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: Option<String>) {
        if idx < 1 || idx as usize > self.data.len() {
            panic!("Index {} out of bounds [1, {}]", idx, self.data.len());
        }
        self.data[(idx - 1) as usize] = value;
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TColStdHSequenceOfTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColStdHSequenceOfTransient::new();
        assert_eq!(seq.length(), 0);

        seq.append(Some("obj1".to_string()));
        seq.append(None);
        seq.append(Some("obj3".to_string()));
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColStdHSequenceOfTransient::new();
        seq.append(Some("first".to_string()));
        seq.append(None);
        seq.append(Some("third".to_string()));

        assert_eq!(seq.at(1), Some(&Some("first".to_string())));
        assert_eq!(seq.at(2), Some(&None));
        assert_eq!(seq.at(3), Some(&Some("third".to_string())));
    }

    #[test]
    fn test_insert_before() {
        let mut seq = TColStdHSequenceOfTransient::new();
        seq.append(Some("a".to_string()));
        seq.append(Some("c".to_string()));
        seq.insert_before(2, Some("b".to_string()));

        assert_eq!(seq.length(), 3);
        assert_eq!(seq.at(2), Some(&Some("b".to_string())));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColStdHSequenceOfTransient::new();
        seq.append(Some("x".to_string()));
        seq.append(None);

        let removed = seq.remove(1);
        assert_eq!(removed, Some(Some("x".to_string())));
        assert_eq!(seq.length(), 1);
    }
}
