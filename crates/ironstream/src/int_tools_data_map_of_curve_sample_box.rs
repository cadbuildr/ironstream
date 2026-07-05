// FILE: int_tools_data_map_of_curve_sample_box.rs
// occt: IntTools_DataMapOfCurveSampleBox

use std::collections::HashMap;

/// Bounding box representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BBox {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

impl BBox {
    /// Create a new bounding box.
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) -> Self {
        BBox {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    /// Check if a point is inside the bounding box.
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.x_min
            && x <= self.x_max
            && y >= self.y_min
            && y <= self.y_max
            && z >= self.z_min
            && z <= self.z_max
    }
}

/// Deprecated alias for a data map of curve sample to bounding box.
#[derive(Clone, Debug)]
pub struct IntTools_DataMapOfCurveSampleBox {
    map: HashMap<u32, BBox>,
}

impl IntTools_DataMapOfCurveSampleBox {
    /// Create a new data map.
    pub fn new() -> Self {
        IntTools_DataMapOfCurveSampleBox {
            map: HashMap::new(),
        }
    }

    /// Bind a curve sample to a bounding box.
    pub fn bind(&mut self, curve_sample_id: u32, bbox: BBox) {
        self.map.insert(curve_sample_id, bbox);
    }

    /// Find a bounding box by curve sample ID.
    pub fn find(&self, curve_sample_id: u32) -> Option<BBox> {
        self.map.get(&curve_sample_id).copied()
    }

    /// Get the number of entries.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for IntTools_DataMapOfCurveSampleBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_new() {
        let bbox = BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(bbox.x_min, 0.0);
        assert_eq!(bbox.x_max, 1.0);
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert!(bbox.contains(0.5, 0.5, 0.5));
        assert!(!bbox.contains(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_map_new() {
        let map = IntTools_DataMapOfCurveSampleBox::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = IntTools_DataMapOfCurveSampleBox::new();
        let bbox = BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        map.bind(1, bbox);
        assert_eq!(map.find(1), Some(bbox));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut map = IntTools_DataMapOfCurveSampleBox::new();
        map.bind(1, BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0));
        map.clear();
        assert!(map.is_empty());
    }
}
