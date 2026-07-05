// FILE: t_col_std_h_sequence_of_h_ascii_string.rs
// occt: TColStd_HSequenceOfHAsciiString

/// TColStd_HSequenceOfHAsciiString is a deprecated alias for a handle sequence of handle ASCII strings.
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColStdHSequenceOfHAsciiString {
    data: Vec<Option<String>>,
}

impl TColStdHSequenceOfHAsciiString {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColStdHSequenceOfHAsciiString {
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

impl Default for TColStdHSequenceOfHAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColStdHSequenceOfHAsciiString::new();
        assert_eq!(seq.length(), 0);

        seq.append(Some("str1".to_string()));
        seq.append(Some("str2".to_string()));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColStdHSequenceOfHAsciiString::new();
        seq.append(Some("first".to_string()));
        seq.append(None);
        seq.append(Some("third".to_string()));

        assert_eq!(seq.at(1), Some(&Some("first".to_string())));
        assert_eq!(seq.at(2), Some(&None));
        assert_eq!(seq.at(3), Some(&Some("third".to_string())));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColStdHSequenceOfHAsciiString::new();
        seq.append(Some("a".to_string()));
        seq.append(None);

        let removed = seq.remove(1);
        assert_eq!(removed, Some(Some("a".to_string())));
        assert_eq!(seq.length(), 1);
    }
}
