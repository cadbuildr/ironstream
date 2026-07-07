// FILE: t_col_std_sequence_of_extended_string.rs
// occt: TColStd_SequenceOfExtendedString

/// TColStd_SequenceOfExtendedString is a deprecated alias for a sequence of extended strings.
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColStdSequenceOfExtendedString {
    data: Vec<String>,
}

impl TColStdSequenceOfExtendedString {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColStdSequenceOfExtendedString {
            data: Vec::new(),
        }
    }

    /// Appends an element to the sequence.
    pub fn append(&mut self, value: String) {
        self.data.push(value);
    }

    /// Prepends an element to the sequence.
    pub fn prepend(&mut self, value: String) {
        self.data.insert(0, value);
    }

    /// Inserts an element at the given 1-based index.
    pub fn insert_before(&mut self, idx: i32, value: String) {
        if idx < 1 || idx as usize > self.data.len() + 1 {
            panic!("Insert index {} out of range [1, {}]", idx, self.data.len() + 1);
        }
        self.data.insert((idx - 1) as usize, value);
    }

    /// Removes an element at the given 1-based index.
    pub fn remove(&mut self, idx: i32) -> Option<String> {
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
    pub fn at(&self, idx: i32) -> Option<&String> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&self.data[(idx - 1) as usize])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut String> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&mut self.data[(idx - 1) as usize])
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: String) {
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

impl Default for TColStdSequenceOfExtendedString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColStdSequenceOfExtendedString::new();
        assert_eq!(seq.length(), 0);

        seq.append("unicode".to_string());
        seq.append("text".to_string());
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColStdSequenceOfExtendedString::new();
        seq.append("first".to_string());
        seq.append("second".to_string());

        assert_eq!(seq.at(1), Some(&"first".to_string()));
        assert_eq!(seq.at(2), Some(&"second".to_string()));
        assert_eq!(seq.at(3), None);
    }

    #[test]
    fn test_insert_before() {
        let mut seq = TColStdSequenceOfExtendedString::new();
        seq.append("a".to_string());
        seq.append("c".to_string());
        seq.insert_before(2, "b".to_string());

        assert_eq!(seq.at(2), Some(&"b".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColStdSequenceOfExtendedString::new();
        seq.append("x".to_string());
        seq.append("y".to_string());

        let removed = seq.remove(1);
        assert_eq!(removed, Some("x".to_string()));
        assert_eq!(seq.length(), 1);
    }
}
