// FILE: bin_tools_shape_set_base.rs
// occt: BinTools_ShapeSetBase

use std::collections::HashMap;

/// Base class for shape sets with binary serialization.
pub struct BinToolsShapeSetBase {
    shapes: HashMap<u32, Vec<u8>>,
    locations: HashMap<u32, Vec<u8>>,
    next_shape_id: u32,
    next_location_id: u32,
}

impl BinToolsShapeSetBase {
    /// Create a new shape set base.
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            locations: HashMap::new(),
            next_shape_id: 1,
            next_location_id: 1,
        }
    }

    /// Add a shape.
    pub fn add_shape(&mut self, shape_data: Vec<u8>) -> u32 {
        let id = self.next_shape_id;
        self.shapes.insert(id, shape_data);
        self.next_shape_id += 1;
        id
    }

    /// Add a location.
    pub fn add_location(&mut self, location_data: Vec<u8>) -> u32 {
        let id = self.next_location_id;
        self.locations.insert(id, location_data);
        self.next_location_id += 1;
        id
    }

    /// Get a shape by id.
    pub fn get_shape(&self, id: u32) -> Option<&[u8]> {
        self.shapes.get(&id).map(|v| v.as_slice())
    }

    /// Get a location by id.
    pub fn get_location(&self, id: u32) -> Option<&[u8]> {
        self.locations.get(&id).map(|v| v.as_slice())
    }

    /// Get the number of shapes.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Get the number of locations.
    pub fn location_count(&self) -> usize {
        self.locations.len()
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.locations.clear();
        self.next_shape_id = 1;
        self.next_location_id = 1;
    }
}

impl Default for BinToolsShapeSetBase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = BinToolsShapeSetBase::new();
        assert_eq!(set.shape_count(), 0);
        assert_eq!(set.location_count(), 0);
    }

    #[test]
    fn test_add_shape() {
        let mut set = BinToolsShapeSetBase::new();
        let id = set.add_shape(vec![1, 2, 3]);

        assert_eq!(id, 1);
        assert_eq!(set.get_shape(1), Some(&[1, 2, 3][..]));
        assert_eq!(set.shape_count(), 1);
    }

    #[test]
    fn test_add_location() {
        let mut set = BinToolsShapeSetBase::new();
        let id = set.add_location(vec![10, 20]);

        assert_eq!(id, 1);
        assert_eq!(set.get_location(1), Some(&[10, 20][..]));
        assert_eq!(set.location_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut set = BinToolsShapeSetBase::new();
        set.add_shape(vec![1]);
        set.add_location(vec![2]);

        set.clear();
        assert_eq!(set.shape_count(), 0);
        assert_eq!(set.location_count(), 0);
    }
}
