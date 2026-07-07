// FILE: top_tools_indexed_data_map_of_shape_address.rs
// occt: TopTools_IndexedDataMapOfShapeAddress

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape { id: usize }

impl Shape { pub fn new(id: usize) -> Self { Shape { id } } }

#[derive(Clone, Debug)]
pub struct Address { addr: usize }

impl Address { pub fn new(addr: usize) -> Self { Address { addr } } }

#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeAddress {
    entries: Vec<(Shape, Address)>,
}

impl IndexedDataMapOfShapeAddress {
    pub fn new() -> Self { IndexedDataMapOfShapeAddress { entries: Vec::new() } }
    pub fn add(&mut self, s: Shape, a: Address) -> usize {
        if let Some(p) = self.entries.iter().position(|(k, _)| k == &s) {
            self.entries[p] = (s, a);
            p + 1
        } else {
            self.entries.push((s, a));
            self.entries.len()
        }
    }
    pub fn find(&self, s: &Shape) -> Option<&Address> {
        self.entries.iter().find(|(k, _)| k == s).map(|(_, v)| v)
    }
    pub fn size(&self) -> usize { self.entries.len() }
}

impl Default for IndexedDataMapOfShapeAddress {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let mut m = IndexedDataMapOfShapeAddress::new();
        m.add(Shape::new(1), Address::new(100));
        assert!(m.find(&Shape::new(1)).is_some());
    }
}
