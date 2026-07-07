// FILE: iges_defs_attribute_table.rs
// occt: IGESDefs_AttributeTable

//! Attribute table entity for IGES.

#[derive(Clone, Debug)]
pub struct AttributeTable {
    rows: usize,
    cols: usize,
}

impl AttributeTable {
    pub fn new(rows: usize, cols: usize) -> Self {
        AttributeTable { rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl Default for AttributeTable {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let table = AttributeTable::new(10, 5);
        assert_eq!(table.rows(), 10);
        assert_eq!(table.cols(), 5);
        assert_eq!(table.size(), (10, 5));
    }

    #[test]
    fn test_default() {
        let table = AttributeTable::default();
        assert_eq!(table.rows(), 0);
        assert_eq!(table.cols(), 0);
    }
}
