// FILE: top_tools_data_map_of_shape_real.rs
// occt: TopTools_DataMapOfShapeReal

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
pub struct DataMapOfShapeReal {
    data: HashMap<Shape, f64>,
}

impl DataMapOfShapeReal {
    pub fn new() -> Self {
        DataMapOfShapeReal {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: Shape, value: f64) -> bool {
        self.data.insert(shape, value).is_none()
    }

    pub fn find(&self, shape: &Shape) -> Option<f64> {
        self.data.get(shape).copied()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfShapeReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = DataMapOfShapeReal::new();
        map.bind(Shape::new(1), 3.14);
        assert_eq!(map.find(&Shape::new(1)), Some(3.14));
    }
}
