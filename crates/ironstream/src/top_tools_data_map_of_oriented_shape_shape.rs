// FILE: top_tools_data_map_of_oriented_shape_shape.rs
// occt: TopTools_DataMapOfOrientedShapeShape

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shape {
    id: usize,
}

impl Shape {
    pub fn new(id: usize) -> Self {
        Shape { id }
    }
}

#[derive(Clone, Debug)]
pub struct DataMapOfOrientedShapeShape {
    data: HashMap<Shape, Shape>,
}

impl DataMapOfOrientedShapeShape {
    pub fn new() -> Self {
        DataMapOfOrientedShapeShape {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: Shape, value: Shape) -> bool {
        self.data.insert(key, value).is_none()
    }

    pub fn find(&self, key: &Shape) -> Option<&Shape> {
        self.data.get(key)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfOrientedShapeShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = DataMapOfOrientedShapeShape::new();
        map.bind(Shape::new(1), Shape::new(2));
        assert!(map.find(&Shape::new(1)).is_some());
    }
}
