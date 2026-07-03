// FILE: b_rep_tools_shape_set.rs
// occt: BRepTools_ShapeSet

use std::collections::HashMap;

/// Manages a set of shapes with serialization support.
pub struct BRepToolsShapeSet {
    shapes: HashMap<u32, Vec<u8>>,
    shape_names: HashMap<u32, String>,
    next_id: u32,
}

impl BRepToolsShapeSet {
    /// Create a new shape set.
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            shape_names: HashMap::new(),
            next_id: 1,
        }
    }

    /// Add a shape to the set.
    pub fn add_shape(&mut self, data: Vec<u8>, name: String) -> u32 {
        let id = self.next_id;
        self.shapes.insert(id, data);
        self.shape_names.insert(id, name);
        self.next_id += 1;
        id
    }

    /// Get a shape by id.
    pub fn get_shape(&self, id: u32) -> Option<&[u8]> {
        self.shapes.get(&id).map(|v| v.as_slice())
    }

    /// Get the name of a shape.
    pub fn get_name(&self, id: u32) -> Option<&str> {
        self.shape_names.get(&id).map(|s| s.as_str())
    }

    /// Remove a shape.
    pub fn remove_shape(&mut self, id: u32) {
        self.shapes.remove(&id);
        self.shape_names.remove(&id);
    }

    /// Get the number of shapes.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Clear all shapes.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.shape_names.clear();
        self.next_id = 1;
    }

    /// Get all shape ids.
    pub fn shape_ids(&self) -> Vec<u32> {
        self.shapes.keys().copied().collect()
    }

    /// Serialize the shape set.
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.shape_count().to_le_bytes());
        for (&id, data) in &self.shapes {
            result.extend(id.to_le_bytes());
            result.extend(data.len().to_le_bytes());
            result.extend(data.iter());
        }
        result
    }
}

impl Default for BRepToolsShapeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = BRepToolsShapeSet::new();
        assert_eq!(set.shape_count(), 0);
    }

    #[test]
    fn test_add_shape() {
        let mut set = BRepToolsShapeSet::new();
        let id = set.add_shape(vec![1, 2, 3], "shape1".to_string());

        assert_eq!(id, 1);
        assert_eq!(set.shape_count(), 1);
        assert_eq!(set.get_shape(1), Some(&[1, 2, 3][..]));
        assert_eq!(set.get_name(1), Some("shape1"));
    }

    #[test]
    fn test_remove_shape() {
        let mut set = BRepToolsShapeSet::new();
        let id = set.add_shape(vec![1, 2, 3], "shape1".to_string());
        set.remove_shape(id);

        assert_eq!(set.shape_count(), 0);
        assert_eq!(set.get_shape(id), None);
    }

    #[test]
    fn test_multiple_shapes() {
        let mut set = BRepToolsShapeSet::new();
        let id1 = set.add_shape(vec![1], "s1".to_string());
        let id2 = set.add_shape(vec![2], "s2".to_string());

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(set.shape_count(), 2);
    }

    #[test]
    fn test_shape_ids() {
        let mut set = BRepToolsShapeSet::new();
        set.add_shape(vec![1], "s1".to_string());
        set.add_shape(vec![2], "s2".to_string());

        let ids = set.shape_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }
}
