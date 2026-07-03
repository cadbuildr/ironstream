// FILE: b_rep_graph_supplement_editor.rs
// occt: BRepGraph_SupplementEditor

use std::collections::HashMap;

/// Editor for supplementary graph data.
pub struct BRepGraphSupplementEditor {
    supplement_data: HashMap<u32, Vec<u8>>,
    graph_id: usize,
}

impl BRepGraphSupplementEditor {
    /// Create a new supplement editor.
    pub fn new(graph_id: usize) -> Self {
        Self {
            supplement_data: HashMap::new(),
            graph_id,
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Add supplement data for an item.
    pub fn add_supplement(&mut self, item_id: u32, data: Vec<u8>) {
        self.supplement_data.insert(item_id, data);
    }

    /// Get supplement data for an item.
    pub fn get_supplement(&self, item_id: u32) -> Option<&[u8]> {
        self.supplement_data.get(&item_id).map(|v| v.as_slice())
    }

    /// Remove supplement data for an item.
    pub fn remove_supplement(&mut self, item_id: u32) {
        self.supplement_data.remove(&item_id);
    }

    /// Update supplement data.
    pub fn update_supplement(&mut self, item_id: u32, data: Vec<u8>) {
        self.supplement_data.insert(item_id, data);
    }

    /// Get the number of supplements.
    pub fn supplement_count(&self) -> usize {
        self.supplement_data.len()
    }

    /// Clear all supplements.
    pub fn clear(&mut self) {
        self.supplement_data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let editor = BRepGraphSupplementEditor::new(1);
        assert_eq!(editor.graph_id(), 1);
        assert_eq!(editor.supplement_count(), 0);
    }

    #[test]
    fn test_add_supplement() {
        let mut editor = BRepGraphSupplementEditor::new(1);
        editor.add_supplement(1, vec![1, 2, 3]);

        assert_eq!(editor.get_supplement(1), Some(&[1, 2, 3][..]));
        assert_eq!(editor.supplement_count(), 1);
    }

    #[test]
    fn test_remove_supplement() {
        let mut editor = BRepGraphSupplementEditor::new(1);
        editor.add_supplement(1, vec![1, 2, 3]);
        editor.remove_supplement(1);

        assert_eq!(editor.get_supplement(1), None);
        assert_eq!(editor.supplement_count(), 0);
    }

    #[test]
    fn test_update_supplement() {
        let mut editor = BRepGraphSupplementEditor::new(1);
        editor.add_supplement(1, vec![1, 2, 3]);
        editor.update_supplement(1, vec![4, 5, 6]);

        assert_eq!(editor.get_supplement(1), Some(&[4, 5, 6][..]));
        assert_eq!(editor.supplement_count(), 1);
    }
}
