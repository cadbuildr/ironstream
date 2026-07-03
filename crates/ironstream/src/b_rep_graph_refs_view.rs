// FILE: b_rep_graph_refs_view.rs
// occt: BRepGraph_RefsView

use std::collections::HashMap;

/// View for accessing references in the graph.
pub struct BRepGraphRefsView {
    graph_id: usize,
    refs: HashMap<u32, Vec<u8>>,
}

impl BRepGraphRefsView {
    /// Create a new refs view.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            refs: HashMap::new(),
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Get a reference by id.
    pub fn get_ref(&self, ref_id: u32) -> Option<&[u8]> {
        self.refs.get(&ref_id).map(|v| v.as_slice())
    }

    /// Add a reference.
    pub fn add_ref(&mut self, ref_id: u32, data: Vec<u8>) {
        self.refs.insert(ref_id, data);
    }

    /// Remove a reference.
    pub fn remove_ref(&mut self, ref_id: u32) {
        self.refs.remove(&ref_id);
    }

    /// Get the number of references.
    pub fn ref_count(&self) -> usize {
        self.refs.len()
    }

    /// Clear all references.
    pub fn clear(&mut self) {
        self.refs.clear();
    }

    /// Iterate over reference ids.
    pub fn iter_ref_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.refs.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = BRepGraphRefsView::new(1);
        assert_eq!(view.graph_id(), 1);
        assert_eq!(view.ref_count(), 0);
    }

    #[test]
    fn test_add_ref() {
        let mut view = BRepGraphRefsView::new(1);
        view.add_ref(1, vec![1, 2, 3]);

        assert_eq!(view.get_ref(1), Some(&[1, 2, 3][..]));
        assert_eq!(view.ref_count(), 1);
    }

    #[test]
    fn test_remove_ref() {
        let mut view = BRepGraphRefsView::new(1);
        view.add_ref(1, vec![1, 2, 3]);
        view.remove_ref(1);

        assert_eq!(view.get_ref(1), None);
        assert_eq!(view.ref_count(), 0);
    }

    #[test]
    fn test_clear() {
        let mut view = BRepGraphRefsView::new(1);
        view.add_ref(1, vec![1, 2, 3]);
        view.add_ref(2, vec![4, 5, 6]);

        view.clear();
        assert_eq!(view.ref_count(), 0);
    }

    #[test]
    fn test_iter_ref_ids() {
        let mut view = BRepGraphRefsView::new(1);
        view.add_ref(1, vec![1, 2, 3]);
        view.add_ref(2, vec![4, 5, 6]);

        let ids: Vec<_> = view.iter_ref_ids().collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }
}
