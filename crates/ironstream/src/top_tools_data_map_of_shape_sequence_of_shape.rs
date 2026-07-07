// FILE: top_tools_data_map_of_shape_sequence_of_shape.rs
// occt: TopTools_DataMapOfShapeSequenceOfShape

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shape { id: usize }

impl Shape {
    pub fn new(id: usize) -> Self { Shape { id } }
}

#[derive(Clone, Debug)]
pub struct SequenceOfShape { shapes: Vec<Shape> }

impl SequenceOfShape {
    pub fn new() -> Self { SequenceOfShape { shapes: Vec::new() } }
    pub fn append(&mut self, s: Shape) { self.shapes.push(s); }
    pub fn size(&self) -> usize { self.shapes.len() }
}

impl Default for SequenceOfShape {
    fn default() -> Self { Self::new() }
}

#[derive(Clone, Debug)]
pub struct DataMapOfShapeSequenceOfShape {
    data: HashMap<Shape, SequenceOfShape>,
}

impl DataMapOfShapeSequenceOfShape {
    pub fn new() -> Self { DataMapOfShapeSequenceOfShape { data: HashMap::new() } }
    pub fn bind(&mut self, k: Shape, v: SequenceOfShape) -> bool { self.data.insert(k, v).is_none() }
    pub fn find(&self, k: &Shape) -> Option<&SequenceOfShape> { self.data.get(k) }
    pub fn size(&self) -> usize { self.data.len() }
}

impl Default for DataMapOfShapeSequenceOfShape {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let mut map = DataMapOfShapeSequenceOfShape::new();
        let mut seq = SequenceOfShape::new();
        seq.append(Shape::new(2));
        map.bind(Shape::new(1), seq);
        assert!(map.find(&Shape::new(1)).is_some());
    }
}
