// FILE: top_tools_indexed_data_map_of_shape_real.rs
// occt: TopTools_IndexedDataMapOfShapeReal

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape { id: usize }

impl Shape { pub fn new(id: usize) -> Self { Shape { id } } }

#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeReal {
    entries: Vec<(Shape, f64)>,
}

impl IndexedDataMapOfShapeReal {
    pub fn new() -> Self { IndexedDataMapOfShapeReal { entries: Vec::new() } }
    pub fn add(&mut self, s: Shape, v: f64) -> usize {
        if let Some(p) = self.entries.iter().position(|(k, _)| k == &s) {
            self.entries[p] = (s, v);
            p + 1
        } else {
            self.entries.push((s, v));
            self.entries.len()
        }
    }
    pub fn find(&self, s: &Shape) -> Option<f64> {
        self.entries.iter().find(|(k, _)| k == s).map(|(_, v)| *v)
    }
    pub fn size(&self) -> usize { self.entries.len() }
}

impl Default for IndexedDataMapOfShapeReal {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let mut m = IndexedDataMapOfShapeReal::new();
        m.add(Shape::new(1), 3.14);
        assert_eq!(m.find(&Shape::new(1)), Some(3.14));
    }
}
