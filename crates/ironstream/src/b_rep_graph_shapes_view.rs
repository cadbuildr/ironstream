// FILE: b_rep_graph_shapes_view.rs
// occt: BRepGraph_ShapesView

use std::collections::HashMap;

/// View for accessing shapes in the graph.
pub struct BRepGraphShapesView {
    graph_id: usize,
    shapes: HashMap<u32, Vec<u8>>,
}

impl BRepGraphShapesView {
    /// Create a new shapes view.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            shapes: HashMap::new(),
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Get a shape by id.
    pub fn get_shape(&self, shape_id: u32) -> Option<&[u8]> {
        self.shapes.get(&shape_id).map(|v| v.as_slice())
    }

    /// Add a shape.
    pub fn add_shape(&mut self, shape_id: u32, data: Vec<u8>) {
        self.shapes.insert(shape_id, data);
    }

    /// Remove a shape.
    pub fn remove_shape(&mut self, shape_id: u32) {
        self.shapes.remove(&shape_id);
    }

    /// Get the number of shapes.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Clear all shapes.
    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    /// Iterate over shape ids.
    pub fn iter_shape_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.shapes.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = BRepGraphShapesView::new(1);
        assert_eq!(view.graph_id(), 1);
        assert_eq!(view.shape_count(), 0);
    }

    #[test]
    fn test_add_shape() {
        let mut view = BRepGraphShapesView::new(1);
        view.add_shape(1, vec![1, 2, 3]);

        assert_eq!(view.get_shape(1), Some(&[1, 2, 3][..]));
        assert_eq!(view.shape_count(), 1);
    }

    #[test]
    fn test_remove_shape() {
        let mut view = BRepGraphShapesView::new(1);
        view.add_shape(1, vec![1, 2, 3]);
        view.remove_shape(1);

        assert_eq!(view.get_shape(1), None);
        assert_eq!(view.shape_count(), 0);
    }

    #[test]
    fn test_clear() {
        let mut view = BRepGraphShapesView::new(1);
        view.add_shape(1, vec![1, 2, 3]);
        view.add_shape(2, vec![4, 5, 6]);

        view.clear();
        assert_eq!(view.shape_count(), 0);
    }

    #[test]
    fn test_iter_shape_ids() {
        let mut view = BRepGraphShapesView::new(1);
        view.add_shape(1, vec![1, 2, 3]);
        view.add_shape(2, vec![4, 5, 6]);

        let ids: Vec<_> = view.iter_shape_ids().collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }
}
