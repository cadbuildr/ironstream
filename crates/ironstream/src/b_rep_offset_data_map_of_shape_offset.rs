// FILE: b_rep_offset_data_map_of_shape_offset.rs
// occt: BRepOffset_DataMapOfShapeOffset

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    value: f64,
    mode: i32,
}

impl Offset {
    pub fn new(value: f64, mode: i32) -> Self {
        Offset { value, mode }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn mode(&self) -> i32 {
        self.mode
    }
}

pub struct BrepoffsetDataMapOfShapeOffset {
    data: HashMap<usize, Offset>,
}

impl BrepoffsetDataMapOfShapeOffset {
    pub fn new() -> Self {
        BrepoffsetDataMapOfShapeOffset {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, shape_id: usize, offset: Offset) {
        self.data.insert(shape_id, offset);
    }

    pub fn get(&self, shape_id: usize) -> Option<Offset> {
        self.data.get(&shape_id).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, shape_id: usize) -> Option<Offset> {
        self.data.remove(&shape_id)
    }
}

impl Default for BrepoffsetDataMapOfShapeOffset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_creation() {
        let offset = Offset::new(0.5, 1);
        assert_eq!(offset.value(), 0.5);
        assert_eq!(offset.mode(), 1);
    }

    #[test]
    fn test_map_add_get() {
        let mut map = BrepoffsetDataMapOfShapeOffset::new();
        let offset = Offset::new(0.1, 2);
        map.add(1, offset);
        assert_eq!(map.get(1).unwrap().value(), 0.1);
    }
}
