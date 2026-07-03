// FILE: blend_func_tensor.rs
// occt: BlendFunc_Tensor

/// A 3D tensor structure to store gradient of gradient values
#[derive(Clone)]
pub struct BlendFuncTensor {
    data: Vec<f64>,
    nbrow: usize,
    nbcol: usize,
    nbmat: usize,
}

impl BlendFuncTensor {
    /// Constructor
    pub fn new(nb_row: usize, nb_col: usize, nb_mat: usize) -> Self {
        let size = nb_row * nb_col * nb_mat;
        Self {
            data: vec![0.0; size],
            nbrow: nb_row,
            nbcol: nb_col,
            nbmat: nb_mat,
        }
    }

    /// Initialize all elements to a value
    pub fn init(&mut self, value: f64) {
        for v in &mut self.data {
            *v = value;
        }
    }

    /// Get value at (row, col, mat)
    pub fn value(&self, row: usize, col: usize, mat: usize) -> f64 {
        if row >= self.nbrow || col >= self.nbcol || mat >= self.nbmat {
            0.0
        } else {
            self.data[mat * self.nbrow * self.nbcol + row * self.nbcol + col]
        }
    }

    /// Set value at (row, col, mat)
    pub fn set_value(&mut self, row: usize, col: usize, mat: usize, val: f64) {
        if row < self.nbrow && col < self.nbcol && mat < self.nbmat {
            let idx = mat * self.nbrow * self.nbcol + row * self.nbcol + col;
            self.data[idx] = val;
        }
    }

    pub fn nb_row(&self) -> usize {
        self.nbrow
    }

    pub fn nb_col(&self) -> usize {
        self.nbcol
    }

    pub fn nb_mat(&self) -> usize {
        self.nbmat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let tensor = BlendFuncTensor::new(2, 3, 4);
        assert_eq!(tensor.nb_row(), 2);
        assert_eq!(tensor.nb_col(), 3);
        assert_eq!(tensor.nb_mat(), 4);
    }

    #[test]
    fn test_init() {
        let mut tensor = BlendFuncTensor::new(2, 2, 2);
        tensor.init(5.0);
        assert_eq!(tensor.value(0, 0, 0), 5.0);
        assert_eq!(tensor.value(1, 1, 1), 5.0);
    }

    #[test]
    fn test_set_get_value() {
        let mut tensor = BlendFuncTensor::new(3, 3, 3);
        tensor.set_value(1, 2, 0, 7.5);
        assert_eq!(tensor.value(1, 2, 0), 7.5);
    }

    #[test]
    fn test_bounds() {
        let tensor = BlendFuncTensor::new(2, 2, 2);
        assert_eq!(tensor.value(10, 10, 10), 0.0);
    }
}
