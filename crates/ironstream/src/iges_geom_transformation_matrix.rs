// FILE: iges_geom_transformation_matrix.rs
// occt: IGESGeom_TransformationMatrix

/// Defines IGESTransformationMatrix, Type <124> in package IGESGeom.
/// A 4x4 transformation matrix for IGES entities.
#[derive(Clone, Debug)]
pub struct TransformationMatrix {
    /// 4x4 transformation matrix (row-major)
    matrix: [[f64; 4]; 4],
    /// Entity type for IGES (always 124)
    entity_type: i32,
}

impl TransformationMatrix {
    pub fn new() -> Self {
        TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            entity_type: 124,
        }
    }

    /// Sets the transformation matrix.
    pub fn set_matrix(&mut self, matrix: [[f64; 4]; 4]) {
        self.matrix = matrix;
    }

    /// Returns the transformation matrix.
    pub fn matrix(&self) -> [[f64; 4]; 4] {
        self.matrix
    }

    /// Returns the entity type (always 124).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat = TransformationMatrix::new();
        assert_eq!(mat.entity_type(), 124);
        let m = mat.matrix();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[2][2], 1.0);
        assert_eq!(m[3][3], 1.0);
    }

    #[test]
    fn test_set_matrix() {
        let mut mat = TransformationMatrix::new();
        let new_matrix = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        mat.set_matrix(new_matrix);
        assert_eq!(mat.matrix(), new_matrix);
    }
}
