// FILE: int_tools_list_of_box.rs
// occt: IntTools_ListOfBox

use std::vec::Vec;

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
}

/// Deprecated alias for a list of bounding boxes.
#[derive(Clone, Debug)]
pub struct IntTools_ListOfBox {
    boxes: Vec<BBox>,
}

impl IntTools_ListOfBox {
    /// Create a new list.
    pub fn new() -> Self {
        IntTools_ListOfBox {
            boxes: Vec::new(),
        }
    }

    /// Append a box to the list.
    pub fn append(&mut self, bbox: BBox) {
        self.boxes.push(bbox);
    }

    /// Get the number of boxes.
    pub fn length(&self) -> usize {
        self.boxes.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.boxes.is_empty()
    }

    /// Get a box by index.
    pub fn box_at(&self, index: usize) -> Option<BBox> {
        self.boxes.get(index).copied()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.boxes.clear();
    }
}

impl Default for IntTools_ListOfBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = IntTools_ListOfBox::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = IntTools_ListOfBox::new();
        let bbox = BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        list.append(bbox);
        assert_eq!(list.length(), 1);
        assert_eq!(list.box_at(0), Some(bbox));
    }

    #[test]
    fn test_clear() {
        let mut list = IntTools_ListOfBox::new();
        list.append(BBox::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0));
        list.clear();
        assert!(list.is_empty());
    }
}
