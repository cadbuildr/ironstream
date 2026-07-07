// FILE: t_col_std_sequence_of_address.rs
// occt: TColStd_SequenceOfAddress

/// TColStd_SequenceOfAddress is a deprecated alias for a sequence of addresses (pointers represented as i64).
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColStdSequenceOfAddress {
    data: Vec<i64>,
}

impl TColStdSequenceOfAddress {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColStdSequenceOfAddress {
            data: Vec::new(),
        }
    }

    /// Appends an element to the sequence.
    pub fn append(&mut self, value: i64) {
        self.data.push(value);
    }

    /// Prepends an element to the sequence.
    pub fn prepend(&mut self, value: i64) {
        self.data.insert(0, value);
    }

    /// Inserts an element at the given 1-based index.
    pub fn insert_before(&mut self, idx: i32, value: i64) {
        if idx < 1 || idx as usize > self.data.len() + 1 {
            panic!("Insert index {} out of range [1, {}]", idx, self.data.len() + 1);
        }
        self.data.insert((idx - 1) as usize, value);
    }

    /// Removes an element at the given 1-based index.
    pub fn remove(&mut self, idx: i32) -> Option<i64> {
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
    pub fn at(&self, idx: i32) -> Option<&i64> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&self.data[(idx - 1) as usize])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut i64> {
        if idx < 1 || idx as usize > self.data.len() {
            return None;
        }
        Some(&mut self.data[(idx - 1) as usize])
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: i64) {
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

impl Default for TColStdSequenceOfAddress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColStdSequenceOfAddress::new();
        assert_eq!(seq.length(), 0);

        seq.append(0x1000i64);
        seq.append(0x2000i64);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColStdSequenceOfAddress::new();
        seq.append(0xABCDi64);
        seq.append(0xDEADi64);

        assert_eq!(seq.at(1), Some(&0xABCDi64));
        assert_eq!(seq.at(2), Some(&0xDEADi64));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColStdSequenceOfAddress::new();
        seq.append(100i64);
        seq.append(200i64);

        let removed = seq.remove(1);
        assert_eq!(removed, Some(100i64));
        assert_eq!(seq.length(), 1);
    }
}
