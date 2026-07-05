// FILE: top_tools_data_map_of_integer_shape.rs
// occt: TopTools_DataMapOfIntegerShape

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
pub struct DataMapOfIntegerShape {
    data: HashMap<i32, Shape>,
}

impl DataMapOfIntegerShape {
    pub fn new() -> Self {
        DataMapOfIntegerShape {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, shape: Shape) -> bool {
        self.data.insert(key, shape).is_none()
    }

    pub fn find(&self, key: i32) -> Option<&Shape> {
        self.data.get(&key)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfIntegerShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_map() {
        let mut map = DataMapOfIntegerShape::new();
        map.bind(1, Shape::new(10));
        assert!(map.find(1).is_some());
    }
}
