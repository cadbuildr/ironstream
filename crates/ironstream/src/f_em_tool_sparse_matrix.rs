// FILE: f_em_tool_sparse_matrix.rs
// occt: FEmTool_SparseMatrix

/// Sparse matrix for FEM.
pub struct FemToolSparseMatrix {
    rows: usize,
    cols: usize,
    nnz: usize,
}

impl FemToolSparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        FemToolSparseMatrix {
            rows,
            cols,
            nnz: 0,
        }
    }

    pub fn nb_rows(&self) -> usize {
        self.rows
    }

    pub fn nb_cols(&self) -> usize {
        self.cols
    }

    pub fn nb_non_zeros(&self) -> usize {
        self.nnz
    }

    pub fn insert(&mut self, _i: usize, _j: usize, _val: f64) {
        self.nnz += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat = FemToolSparseMatrix::new(5, 5);
        assert_eq!(mat.nb_rows(), 5);
        assert_eq!(mat.nb_non_zeros(), 0);
    }

    #[test]
    fn test_insert() {
        let mut mat = FemToolSparseMatrix::new(5, 5);
        mat.insert(0, 0, 1.5);
        assert_eq!(mat.nb_non_zeros(), 1);
    }
}
