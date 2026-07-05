// FILE: top_tools_data_map_of_oriented_shape_integer.rs
// occt: TopTools_DataMapOfOrientedShapeInteger

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrientedShape {
    id: usize,
}

impl OrientedShape {
    pub fn new(id: usize) -> Self {
        OrientedShape { id }
    }
}

#[derive(Clone, Debug)]
pub struct DataMapOfOrientedShapeInteger {
    data: HashMap<OrientedShape, i32>,
}

impl DataMapOfOrientedShapeInteger {
    pub fn new() -> Self {
        DataMapOfOrientedShapeInteger {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: OrientedShape, value: i32) -> bool {
        self.data.insert(shape, value).is_none()
    }

    pub fn find(&self, shape: &OrientedShape) -> Option<i32> {
        self.data.get(shape).copied()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfOrientedShapeInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = DataMapOfOrientedShapeInteger::new();
        map.bind(OrientedShape::new(1), 42);
        assert_eq!(map.find(&OrientedShape::new(1)), Some(42));
    }
}
