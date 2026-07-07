// FILE: top_tools_indexed_data_map_of_shape_shape.rs
// occt: TopTools_IndexedDataMapOfShapeShape

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape { id: usize }

impl Shape { pub fn new(id: usize) -> Self { Shape { id } } }

#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeShape {
    entries: Vec<(Shape, Shape)>,
}

impl IndexedDataMapOfShapeShape {
    pub fn new() -> Self { IndexedDataMapOfShapeShape { entries: Vec::new() } }
    pub fn add(&mut self, k: Shape, v: Shape) -> usize {
        if let Some(p) = self.entries.iter().position(|(x, _)| x == &k) {
            self.entries[p] = (k, v);
            p + 1
        } else {
            self.entries.push((k, v));
            self.entries.len()
        }
    }
    pub fn find(&self, k: &Shape) -> Option<&Shape> {
        self.entries.iter().find(|(x, _)| x == k).map(|(_, v)| v)
    }
    pub fn size(&self) -> usize { self.entries.len() }
}

impl Default for IndexedDataMapOfShapeShape {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let mut m = IndexedDataMapOfShapeShape::new();
        m.add(Shape::new(1), Shape::new(2));
        assert!(m.find(&Shape::new(1)).is_some());
    }
}
