// FILE: t_col_geom_sequence_of_bounded_curve.rs
// occt: TColGeom_SequenceOfBoundedCurve

/// TColGeom_SequenceOfBoundedCurve is a deprecated alias for a sequence of bounded curves.
/// This is a Rust port implementing OCCT's sequence semantics (1-based indexing).
pub struct TColGeomSequenceOfBoundedCurve {
    data: Vec<Option<String>>,
}

impl TColGeomSequenceOfBoundedCurve {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        TColGeomSequenceOfBoundedCurve {
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

impl Default for TColGeomSequenceOfBoundedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut seq = TColGeomSequenceOfBoundedCurve::new();
        assert_eq!(seq.length(), 0);

        seq.append(Some("arc".to_string()));
        seq.append(Some("line".to_string()));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_at_1_based_indexing() {
        let mut seq = TColGeomSequenceOfBoundedCurve::new();
        seq.append(Some("first".to_string()));
        seq.append(Some("second".to_string()));

        assert_eq!(seq.at(1), Some(&Some("first".to_string())));
        assert_eq!(seq.at(2), Some(&Some("second".to_string())));
        assert_eq!(seq.at(3), None);
    }

    #[test]
    fn test_insert_before() {
        let mut seq = TColGeomSequenceOfBoundedCurve::new();
        seq.append(Some("first".to_string()));
        seq.append(Some("third".to_string()));
        seq.insert_before(2, Some("second".to_string()));

        assert_eq!(seq.length(), 3);
        assert_eq!(seq.at(2), Some(&Some("second".to_string())));
    }

    #[test]
    fn test_remove() {
        let mut seq = TColGeomSequenceOfBoundedCurve::new();
        seq.append(Some("a".to_string()));
        seq.append(Some("b".to_string()));

        let removed = seq.remove(1);
        assert_eq!(removed, Some(Some("a".to_string())));
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn test_clear() {
        let mut seq = TColGeomSequenceOfBoundedCurve::new();
        seq.append(Some("x".to_string()));
        seq.clear();
        assert_eq!(seq.length(), 0);
    }
}
