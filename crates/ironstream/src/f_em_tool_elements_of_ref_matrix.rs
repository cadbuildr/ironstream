// FILE: f_em_tool_elements_of_ref_matrix.rs
// occt: FEmTool_ElementsOfRefMatrix

/// Elements of reference matrix for FEM.
pub struct FemToolElementsOfRefMatrix {
    values: Vec<Vec<f64>>,
}

impl FemToolElementsOfRefMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        FemToolElementsOfRefMatrix {
            values: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        if i < self.values.len() && j < self.values[0].len() {
            self.values[i][j] = val;
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        if i < self.values.len() && j < self.values[0].len() {
            self.values[i][j]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat = FemToolElementsOfRefMatrix::new(3, 3);
        assert_eq!(mat.get(0, 0), 0.0);
    }

    #[test]
    fn test_set_get() {
        let mut mat = FemToolElementsOfRefMatrix::new(3, 3);
        mat.set(1, 1, 5.0);
        assert_eq!(mat.get(1, 1), 5.0);
    }
}
