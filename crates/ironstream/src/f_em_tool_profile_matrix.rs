// FILE: f_em_tool_profile_matrix.rs
// occt: FEmTool_ProfileMatrix

/// Profile matrix for FEM.
pub struct FemToolProfileMatrix {
    rows: usize,
    cols: usize,
}

impl FemToolProfileMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        FemToolProfileMatrix { rows, cols }
    }

    pub fn nb_rows(&self) -> usize {
        self.rows
    }

    pub fn nb_cols(&self) -> usize {
        self.cols
    }

    pub fn profile_width(&self) -> usize {
        self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat = FemToolProfileMatrix::new(5, 5);
        assert_eq!(mat.nb_rows(), 5);
        assert_eq!(mat.nb_cols(), 5);
    }

    #[test]
    fn test_profile_width() {
        let mat = FemToolProfileMatrix::new(3, 7);
        assert_eq!(mat.profile_width(), 7);
    }
}
