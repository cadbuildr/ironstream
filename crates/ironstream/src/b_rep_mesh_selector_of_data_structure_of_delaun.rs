// FILE: b_rep_mesh_selector_of_data_structure_of_delaun.rs
// occt: BRepMesh_SelectorOfDataStructureOfDelaun

/// Selector for Delaunay data structure elements.
pub struct SelectorOfDataStructureOfDelaun {
    selected_indices: Vec<usize>,
}

impl SelectorOfDataStructureOfDelaun {
    /// Constructor.
    pub fn new() -> Self {
        SelectorOfDataStructureOfDelaun {
            selected_indices: Vec::new(),
        }
    }

    /// Selects elements.
    pub fn select(&mut self, index: usize) {
        self.selected_indices.push(index);
    }

    /// Returns selected indices.
    pub fn selected(&self) -> &[usize] {
        &self.selected_indices
    }
}

impl Default for SelectorOfDataStructureOfDelaun {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_new() {
        let selector = SelectorOfDataStructureOfDelaun::new();
        assert!(selector.selected().is_empty());
    }

    #[test]
    fn test_selector_select() {
        let mut selector = SelectorOfDataStructureOfDelaun::new();
        selector.select(0);
        selector.select(1);
        assert_eq!(selector.selected().len(), 2);
    }
}
