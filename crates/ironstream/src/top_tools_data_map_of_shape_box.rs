// FILE: top_tools_data_map_of_shape_box.rs
// occt: TopTools_DataMapOfShapeBox

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
pub struct Box {
    xmin: f64,
    xmax: f64,
}

impl Box {
    pub fn new(xmin: f64, xmax: f64) -> Self {
        Box { xmin, xmax }
    }
}

#[derive(Clone, Debug)]
pub struct DataMapOfShapeBox {
    data: HashMap<Shape, Box>,
}

impl DataMapOfShapeBox {
    pub fn new() -> Self {
        DataMapOfShapeBox {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: Shape, bx: Box) -> bool {
        self.data.insert(shape, bx).is_none()
    }

    pub fn find(&self, shape: &Shape) -> Option<&Box> {
        self.data.get(shape)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfShapeBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = DataMapOfShapeBox::new();
        map.bind(Shape::new(1), Box::new(0.0, 1.0));
        assert!(map.find(&Shape::new(1)).is_some());
    }
}
