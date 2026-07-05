// FILE: top_tools_h_array1_of_list_of_shape.rs
// occt: TopTools_HArray1OfListOfShape

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Shape { id: usize }

#[derive(Clone, Debug)]
pub struct ListOfShape { shapes: Vec<Shape> }

impl ListOfShape {
    pub fn new() -> Self { ListOfShape { shapes: Vec::new() } }
    pub fn append(&mut self, s: Shape) { self.shapes.push(s); }
}

impl Default for ListOfShape {
    fn default() -> Self { Self::new() }
}

struct Content { data: Vec<ListOfShape>, lower: usize }

#[derive(Clone)]
pub struct HArray1OfListOfShape { inner: Arc<Content> }

impl HArray1OfListOfShape {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower == 0 { panic!("1-based indexing"); }
        let size = upper - lower + 1;
        HArray1OfListOfShape {
            inner: Arc::new(Content {
                data: (0..size).map(|_| ListOfShape::new()).collect(),
                lower,
            }),
        }
    }
    pub fn lower(&self) -> usize { self.inner.lower }
    pub fn upper(&self) -> usize { self.inner.lower + self.inner.data.len() - 1 }
    pub fn length(&self) -> usize { self.inner.data.len() }
}

impl std::fmt::Debug for HArray1OfListOfShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray1OfListOfShape").field("lower", &self.lower()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_h_array1() {
        let arr = HArray1OfListOfShape::new(1, 5);
        assert_eq!(arr.length(), 5);
    }
}
