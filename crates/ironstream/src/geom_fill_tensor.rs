// FILE: geom_fill_tensor.rs
// occt: GeomFill_Tensor

/// Tensor for surface computation
pub struct Tensor {
    rows: usize,
    cols: usize,
    depth: usize,
}

impl Tensor {
    pub fn new(rows: usize, cols: usize, depth: usize) -> Self {
        Tensor { rows, cols, depth }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let tensor = Tensor::new(3, 4, 5);
        assert_eq!(tensor.rows(), 3);
        assert_eq!(tensor.cols(), 4);
        assert_eq!(tensor.depth(), 5);
    }

    #[test]
    fn test_zero_dimensions() {
        let tensor = Tensor::new(0, 0, 0);
        assert_eq!(tensor.rows(), 0);
    }
}
