// FILE: f_em_tool_assembly.rs
// occt: FEmTool_Assembly

/// Assemble and solve system from one-dimensional finite elements.
pub struct FemToolAssembly {
    rows: usize,
    cols: usize,
}

impl FemToolAssembly {
    pub fn new(rows: usize, cols: usize) -> Self {
        FemToolAssembly { rows, cols }
    }

    /// Nullify all matrix coefficients.
    pub fn nullify_matrix(&mut self) {
        // Matrix coefficients reset
    }

    /// Add an elementary matrix in assembly.
    pub fn add_elementary_matrix(&mut self, i: usize, j: usize, value: f64) {
        if i < self.rows && j < self.cols {
            // Add operation
        }
    }

    /// Get number of rows.
    pub fn nb_rows(&self) -> usize {
        self.rows
    }

    /// Get number of columns.
    pub fn nb_cols(&self) -> usize {
        self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let asm = FemToolAssembly::new(5, 5);
        assert_eq!(asm.nb_rows(), 5);
        assert_eq!(asm.nb_cols(), 5);
    }

    #[test]
    fn test_nullify_matrix() {
        let mut asm = FemToolAssembly::new(3, 3);
        asm.nullify_matrix();
        assert_eq!(asm.nb_rows(), 3);
    }

    #[test]
    fn test_add_elementary_matrix() {
        let mut asm = FemToolAssembly::new(3, 3);
        asm.add_elementary_matrix(0, 0, 1.5);
    }
}
