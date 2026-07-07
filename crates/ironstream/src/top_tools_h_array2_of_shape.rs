// FILE: top_tools_h_array2_of_shape.rs
// occt: TopTools_HArray2OfShape

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Shape { id: usize }

impl Shape { pub fn new(id: usize) -> Self { Shape { id } } }

struct Content { data: Vec<Shape>, rows: usize, cols: usize, rl: usize, cl: usize }

#[derive(Clone)]
pub struct HArray2OfShape { inner: Arc<Content> }

impl HArray2OfShape {
    pub fn new(rl: usize, ru: usize, cl: usize, cu: usize) -> Self {
        if rl == 0 || cl == 0 { panic!("1-based"); }
        let rows = ru - rl + 1;
        let cols = cu - cl + 1;
        HArray2OfShape {
            inner: Arc::new(Content {
                data: (0..rows * cols).map(|i| Shape::new(i)).collect(),
                rows,
                cols,
                rl,
                cl,
            }),
        }
    }
    pub fn num_rows(&self) -> usize { self.inner.rows }
    pub fn num_cols(&self) -> usize { self.inner.cols }
}

impl std::fmt::Debug for HArray2OfShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray2OfShape").field("rows", &self.inner.rows).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_h_array2() {
        let arr = HArray2OfShape::new(1, 2, 1, 3);
        assert_eq!(arr.num_rows(), 2);
        assert_eq!(arr.num_cols(), 3);
    }
}
