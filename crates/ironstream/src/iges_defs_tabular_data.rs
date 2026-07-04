// FILE: iges_defs_tabular_data.rs
// occt: IGESDefs_TabularData

//! Tabular data entity for IGES definitions.

#[derive(Clone, Debug)]
pub struct TabularData {
    name: String,
    rows: usize,
    cols: usize,
}

impl TabularData {
    pub fn new(name: &str, rows: usize, cols: usize) -> Self {
        TabularData {
            name: name.to_string(),
            rows,
            cols,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Default for TabularData {
    fn default() -> Self {
        Self::new("", 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = TabularData::new("table1", 10, 5);
        assert_eq!(data.name(), "table1");
        assert_eq!(data.rows(), 10);
        assert_eq!(data.cols(), 5);
    }

    #[test]
    fn test_default() {
        let data = TabularData::default();
        assert_eq!(data.name(), "");
        assert_eq!(data.rows(), 0);
        assert_eq!(data.cols(), 0);
    }
}
