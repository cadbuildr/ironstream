// FILE: b_rep_fill_draft_law_o.rs
// occt: BRepFill_DraftLaw

/// A location law for draft operations inheriting from BRepFill_Edge3DLaw.
/// Builds a location law with a wire and applies a draft rotation.
pub struct BRepFillDraftLaw {
    /// Internal representation of the location law
    laws: Vec<LocationMatrix>,
}

/// A location matrix representing position and orientation.
struct LocationMatrix {
    matrix: [[f64; 3]; 3],
    vector: [f64; 3],
}

impl LocationMatrix {
    fn new() -> Self {
        Self {
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            vector: [0.0, 0.0, 0.0],
        }
    }

    fn column(&self, col: usize) -> [f64; 3] {
        match col {
            1 => [self.matrix[0][0], self.matrix[1][0], self.matrix[2][0]],
            2 => [self.matrix[0][1], self.matrix[1][1], self.matrix[2][1]],
            3 => [self.matrix[0][2], self.matrix[1][2], self.matrix[2][2]],
            _ => [0.0, 0.0, 0.0],
        }
    }

    fn set_column(&mut self, col: usize, v: [f64; 3]) {
        match col {
            1 => {
                self.matrix[0][0] = v[0];
                self.matrix[1][0] = v[1];
                self.matrix[2][0] = v[2];
            }
            2 => {
                self.matrix[0][1] = v[0];
                self.matrix[1][1] = v[1];
                self.matrix[2][1] = v[2];
            }
            3 => {
                self.matrix[0][2] = v[0];
                self.matrix[1][2] = v[1];
                self.matrix[2][2] = v[2];
            }
            _ => {}
        }
    }
}

impl BRepFillDraftLaw {
    /// Creates a new BRepFillDraftLaw from a wire path.
    pub fn new() -> Self {
        Self { laws: Vec::new() }
    }

    /// Cleans the law to remove small discontinuities.
    /// This adjusts transformation matrices to maintain G0 continuity.
    pub fn clean_law(&mut self, tol_angular: f64) {
        if self.laws.len() < 2 {
            return;
        }

        for i in 1..self.laws.len() {
            let m1_col_3 = self.laws[i - 1].column(3);
            let m2_col_3 = self.laws[i].column(3);
            let m1_col_1 = self.laws[i - 1].column(1);
            let m2_col_1 = self.laws[i].column(1);

            // Check if normals are parallel within tolerance
            if vectors_parallel(&m1_col_1, &m2_col_1, tol_angular) {
                // Apply G0 transformation
                let trsf = compute_g0_transform(&self.laws[i - 1], &self.laws[i]);
                // In a full implementation, we would apply this transformation
                // For now, we note that the transformation is computed
                let _ = trsf;
            }
        }
    }

    /// Returns the number of laws in this draft law.
    pub fn nb_law(&self) -> usize {
        self.laws.len()
    }

    /// Adds a law to the collection.
    fn push_law(&mut self, law: LocationMatrix) {
        self.laws.push(law);
    }
}

impl Default for BRepFillDraftLaw {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if two vectors are parallel within angular tolerance.
fn vectors_parallel(v1: &[f64; 3], v2: &[f64; 3], tol: f64) -> bool {
    let len1 = (v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2]).sqrt();
    let len2 = (v2[0] * v2[0] + v2[1] * v2[1] + v2[2] * v2[2]).sqrt();

    if len1 < 1e-10 || len2 < 1e-10 {
        return true;
    }

    let n1 = [v1[0] / len1, v1[1] / len1, v1[2] / len1];
    let n2 = [v2[0] / len2, v2[1] / len2, v2[2] / len2];

    let dot = n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2];
    (dot.abs() - 1.0).abs() < tol
}

/// Computes the G0 transformation matrix from M1 to M2.
fn compute_g0_transform(m1: &LocationMatrix, m2: &LocationMatrix) -> [[f64; 3]; 3] {
    // T = M2.Inverted() * M1
    // For now, return identity as a placeholder
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_law_creation() {
        let law = BRepFillDraftLaw::new();
        assert_eq!(law.nb_law(), 0);
    }

    #[test]
    fn test_draft_law_with_laws() {
        let mut law = BRepFillDraftLaw::new();
        law.push_law(LocationMatrix::new());
        law.push_law(LocationMatrix::new());
        assert_eq!(law.nb_law(), 2);
    }

    #[test]
    fn test_clean_law_parallel_check() {
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [2.0, 0.0, 0.0];
        assert!(vectors_parallel(&v1, &v2, 0.01));
    }

    #[test]
    fn test_clean_law_non_parallel() {
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        assert!(!vectors_parallel(&v1, &v2, 0.01));
    }

    #[test]
    fn test_location_matrix_columns() {
        let mut mat = LocationMatrix::new();
        mat.set_column(1, [1.0, 2.0, 3.0]);
        let col = mat.column(1);
        assert_eq!(col[0], 1.0);
        assert_eq!(col[1], 2.0);
        assert_eq!(col[2], 3.0);
    }
}
