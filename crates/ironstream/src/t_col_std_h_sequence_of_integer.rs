// FILE: t_col_std_h_sequence_of_integer.rs
// occt: TColStd_HSequenceOfInteger

/// TColStd_HSequenceOfInteger is a deprecated alias for a handle (heap-allocated) sequence of integers.
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColStdHSequenceOfInteger {
    data: Vec<i32>,
}

impl TColStdHSequenceOfInteger {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColStdHSequenceOfInteger {
            data: Vec::new(),
        }
    }

    /// Appends an element to the sequence.
    pub fn append(&mut self, value: i32) {
        self.data.push(value);
    }

    /// Prepends an element to the sequence.
    pub fn prepend(&mut self, value: i32) {
        self.data.insert(0, value);
    }

    /// Inserts an element at the given 1-based index.
    pub fn insert_before(&mut self, idx: i32, value: i32) {
        if idx < 1 || idx as usize > self.data.len() + 1 {
            panic!("Insert index {} out of range [1, {}]", idx, self.data.len() + 1);
        }
        self.data.insert((idx - 1) as usize, value);
    }

    /// Removes an element at the given 1-based index.
    pub fn remove(&mut self, idx: i32) -> Option<i32> {
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
    pub fn at(&self, idx: i32) -> Option<&i32> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&self.data[(idx - 1) as usize])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut i32> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&mut self.data[(idx - 1) as usize])
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: i32) {
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

impl Default for TColStdHSequenceOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColStdHSequenceOfInteger::new();
        assert_eq!(seq.length(), 0);

        seq.append(10);
        seq.append(20);
        seq.append(30);
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColStdHSequenceOfInteger::new();
        seq.append(100);
        seq.append(200);

        assert_eq!(seq.at(1), Some(&100));
        assert_eq!(seq.at(2), Some(&200));
        assert_eq!(seq.at(3), None);
    }

    #[test]
    fn test_insert_before() {
        let mut seq = TColStdHSequenceOfInteger::new();
        seq.append(1);
        seq.append(3);
        seq.insert_before(2, 2);

        assert_eq!(seq.at(1), Some(&1));
        assert_eq!(seq.at(2), Some(&2));
        assert_eq!(seq.at(3), Some(&3));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColStdHSequenceOfInteger::new();
        seq.append(5);
        seq.append(10);

        let removed = seq.remove(1);
        assert_eq!(removed, Some(5));
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn test_set() {
        let mut seq = TColStdHSequenceOfInteger::new();
        seq.append(42);
        seq.set(1, 99);

        assert_eq!(seq.at(1), Some(&99));
    }
}
