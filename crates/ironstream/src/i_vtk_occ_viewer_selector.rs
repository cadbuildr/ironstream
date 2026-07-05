// FILE: i_vtk_occ_viewer_selector.rs
// occt: IVtkOCC_ViewerSelector

use std::collections::HashSet;

/// VTK-OCC selector for managing selection of objects in the viewport.
#[derive(Clone, Debug)]
pub struct IVtkOCC_ViewerSelector {
    selected_ids: HashSet<u32>,
}

impl IVtkOCC_ViewerSelector {
    /// Create a new viewer selector.
    pub fn new() -> Self {
        IVtkOCC_ViewerSelector {
            selected_ids: HashSet::new(),
        }
    }

    /// Select an object by ID.
    pub fn select(&mut self, id: u32) {
        self.selected_ids.insert(id);
    }

    /// Deselect an object by ID.
    pub fn deselect(&mut self, id: u32) {
        self.selected_ids.remove(&id);
    }

    /// Check if an object is selected.
    pub fn is_selected(&self, id: u32) -> bool {
        self.selected_ids.contains(&id)
    }

    /// Clear all selections.
    pub fn clear_all(&mut self) {
        self.selected_ids.clear();
    }

    /// Get the number of selected objects.
    pub fn selection_count(&self) -> usize {
        self.selected_ids.len()
    }
}

impl Default for IVtkOCC_ViewerSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_selector() {
        let selector = IVtkOCC_ViewerSelector::new();
        assert_eq!(selector.selection_count(), 0);
    }

    #[test]
    fn test_select() {
        let mut selector = IVtkOCC_ViewerSelector::new();
        selector.select(1);
        assert!(selector.is_selected(1));
        assert_eq!(selector.selection_count(), 1);
    }

    #[test]
    fn test_deselect() {
        let mut selector = IVtkOCC_ViewerSelector::new();
        selector.select(2);
        selector.deselect(2);
        assert!(!selector.is_selected(2));
        assert_eq!(selector.selection_count(), 0);
    }

    #[test]
    fn test_multiple_selections() {
        let mut selector = IVtkOCC_ViewerSelector::new();
        selector.select(10);
        selector.select(20);
        selector.select(30);
        assert_eq!(selector.selection_count(), 3);
        assert!(selector.is_selected(10));
        assert!(selector.is_selected(20));
        assert!(selector.is_selected(30));
    }

    #[test]
    fn test_clear_all() {
        let mut selector = IVtkOCC_ViewerSelector::new();
        selector.select(1);
        selector.select(2);
        selector.clear_all();
        assert_eq!(selector.selection_count(), 0);
    }
}
