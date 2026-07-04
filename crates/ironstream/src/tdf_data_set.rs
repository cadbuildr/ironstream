// FILE: tdf_data_set.rs
// occt: TDF_DataSet

/// A set of labels and attributes forming a connected data set.
pub struct TdfDataSet {
    size: usize,
}

impl TdfDataSet {
    /// Creates a new data set.
    pub fn new() -> Self {
        TdfDataSet { size: 0 }
    }

    /// Adds a label to the data set.
    pub fn add_label(&mut self) {
        self.size += 1;
    }

    /// Returns the number of elements in the data set.
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for TdfDataSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_set() {
        let set = TdfDataSet::new();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_data_set_add() {
        let mut set = TdfDataSet::new();
        set.add_label();
        assert_eq!(set.size(), 1);
    }
}
