// FILE: bop_tools_box_selector.rs
// occt: BOPTools_BoxSelector

/// Selector for box elements. Template-based, but we specialize for 3D.
#[derive(Clone, Debug)]
pub struct BopToolsBoxSelector {
    indices: Vec<i32>,
}

impl BopToolsBoxSelector {
    /// Create an empty selector.
    pub fn new() -> Self {
        BopToolsBoxSelector {
            indices: Vec::new(),
        }
    }

    /// Clear the indices.
    pub fn clear(&mut self) {
        self.indices.clear();
    }

    /// Get the indices.
    pub fn indices(&self) -> &[i32] {
        &self.indices
    }

    /// Add an index.
    pub fn add_index(&mut self, idx: i32) {
        self.indices.push(idx);
    }
}

impl Default for BopToolsBoxSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let selector = BopToolsBoxSelector::new();
        assert_eq!(selector.indices().len(), 0);
    }

    #[test]
    fn test_add_index() {
        let mut selector = BopToolsBoxSelector::new();
        selector.add_index(5);
        selector.add_index(10);
        assert_eq!(selector.indices().len(), 2);
        assert_eq!(selector.indices()[0], 5);
        assert_eq!(selector.indices()[1], 10);
    }

    #[test]
    fn test_clear() {
        let mut selector = BopToolsBoxSelector::new();
        selector.add_index(1);
        selector.clear();
        assert_eq!(selector.indices().len(), 0);
    }
}
