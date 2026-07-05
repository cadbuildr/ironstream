// FILE: i_vtk_tools_sub_poly_data_filter.rs
// occt: IVtkTools_SubPolyDataFilter

use std::collections::HashSet;

/// Filter for extracting subsets of polygonal data.
#[derive(Clone, Debug)]
pub struct IVtkTools_SubPolyDataFilter {
    cell_ids: HashSet<u32>,
}

impl IVtkTools_SubPolyDataFilter {
    /// Create a new sub poly data filter.
    pub fn new() -> Self {
        IVtkTools_SubPolyDataFilter {
            cell_ids: HashSet::new(),
        }
    }

    /// Add a cell ID to the filter.
    pub fn add_cell_id(&mut self, id: u32) {
        self.cell_ids.insert(id);
    }

    /// Remove a cell ID from the filter.
    pub fn remove_cell_id(&mut self, id: u32) {
        self.cell_ids.remove(&id);
    }

    /// Check if a cell ID is in the filter.
    pub fn contains_cell_id(&self, id: u32) -> bool {
        self.cell_ids.contains(&id)
    }

    /// Get the number of cells in the filter.
    pub fn cell_count(&self) -> usize {
        self.cell_ids.len()
    }

    /// Clear all cell IDs.
    pub fn clear(&mut self) {
        self.cell_ids.clear();
    }
}

impl Default for IVtkTools_SubPolyDataFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filter() {
        let filter = IVtkTools_SubPolyDataFilter::new();
        assert_eq!(filter.cell_count(), 0);
    }

    #[test]
    fn test_add_cell_id() {
        let mut filter = IVtkTools_SubPolyDataFilter::new();
        filter.add_cell_id(1);
        assert!(filter.contains_cell_id(1));
        assert_eq!(filter.cell_count(), 1);
    }

    #[test]
    fn test_remove_cell_id() {
        let mut filter = IVtkTools_SubPolyDataFilter::new();
        filter.add_cell_id(2);
        filter.remove_cell_id(2);
        assert!(!filter.contains_cell_id(2));
        assert_eq!(filter.cell_count(), 0);
    }

    #[test]
    fn test_multiple_cells() {
        let mut filter = IVtkTools_SubPolyDataFilter::new();
        filter.add_cell_id(10);
        filter.add_cell_id(20);
        filter.add_cell_id(30);
        assert_eq!(filter.cell_count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut filter = IVtkTools_SubPolyDataFilter::new();
        filter.add_cell_id(1);
        filter.add_cell_id(2);
        filter.clear();
        assert_eq!(filter.cell_count(), 0);
    }
}
