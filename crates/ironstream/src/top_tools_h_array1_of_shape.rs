// FILE: top_tools_h_array1_of_shape.rs
// occt: TopTools_HArray1OfShape

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Shape { id: usize }

impl Shape {
    pub fn new(id: usize) -> Self { Shape { id } }
}

struct Content { data: Vec<Shape>, lower: usize }

#[derive(Clone)]
pub struct HArray1OfShape { inner: Arc<Content> }

impl HArray1OfShape {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower == 0 { panic!("1-based indexing"); }
        let size = upper - lower + 1;
        HArray1OfShape {
            inner: Arc::new(Content {
                data: (0..size).map(|i| Shape::new(i)).collect(),
                lower,
            }),
        }
    }
    pub fn lower(&self) -> usize { self.inner.lower }
    pub fn upper(&self) -> usize { self.inner.lower + self.inner.data.len() - 1 }
    pub fn length(&self) -> usize { self.inner.data.len() }
}

impl std::fmt::Debug for HArray1OfShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray1OfShape").field("lower", &self.lower()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_h_array1() {
        let arr = HArray1OfShape::new(1, 5);
        assert_eq!(arr.length(), 5);
    }
}
