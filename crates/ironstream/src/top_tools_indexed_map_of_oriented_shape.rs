// FILE: top_tools_indexed_map_of_oriented_shape.rs
// occt: TopTools_IndexedMapOfOrientedShape

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrientedShape { id: usize }

impl OrientedShape { pub fn new(id: usize) -> Self { OrientedShape { id } } }

#[derive(Clone, Debug)]
pub struct IndexedMapOfOrientedShape {
    data: HashSet<OrientedShape>,
}

impl IndexedMapOfOrientedShape {
    pub fn new() -> Self { IndexedMapOfOrientedShape { data: HashSet::new() } }
    pub fn add(&mut self, s: OrientedShape) -> bool { self.data.insert(s) }
    pub fn contains(&self, s: &OrientedShape) -> bool { self.data.contains(s) }
    pub fn remove(&mut self, s: &OrientedShape) -> bool { self.data.remove(s) }
    pub fn size(&self) -> usize { self.data.len() }
}

impl Default for IndexedMapOfOrientedShape {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let mut m = IndexedMapOfOrientedShape::new();
        let s = OrientedShape::new(1);
        assert!(m.add(s.clone()));
        assert!(m.contains(&s));
    }
}
