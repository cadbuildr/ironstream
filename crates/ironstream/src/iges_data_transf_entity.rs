// FILE: iges_data_transf_entity.rs
// occt: IGESData_TransfEntity

//! Entity with transformation properties.

#[derive(Clone, Debug)]
pub struct TransfEntity {
    matrix: [[f64; 4]; 4],
}

impl TransfEntity {
    pub fn new() -> Self {
        let mut matrix = [[0.0; 4]; 4];
        // Identity matrix
        matrix[0][0] = 1.0;
        matrix[1][1] = 1.0;
        matrix[2][2] = 1.0;
        matrix[3][3] = 1.0;
        TransfEntity { matrix }
    }

    pub fn identity() -> Self {
        Self::new()
    }

    pub fn matrix(&self) -> &[[f64; 4]; 4] {
        &self.matrix
    }

    pub fn set_translation(&mut self, tx: f64, ty: f64, tz: f64) {
        self.matrix[0][3] = tx;
        self.matrix[1][3] = ty;
        self.matrix[2][3] = tz;
    }
}

impl Default for TransfEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let transf = TransfEntity::new();
        let matrix = transf.matrix();
        // Check identity matrix
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[1][1], 1.0);
        assert_eq!(matrix[2][2], 1.0);
        assert_eq!(matrix[3][3], 1.0);
    }

    #[test]
    fn test_identity() {
        let transf = TransfEntity::identity();
        let matrix = transf.matrix();
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[1][1], 1.0);
    }

    #[test]
    fn test_set_translation() {
        let mut transf = TransfEntity::new();
        transf.set_translation(10.0, 20.0, 30.0);
        let matrix = transf.matrix();
        assert_eq!(matrix[0][3], 10.0);
        assert_eq!(matrix[1][3], 20.0);
        assert_eq!(matrix[2][3], 30.0);
    }

    #[test]
    fn test_default() {
        let transf = TransfEntity::default();
        let matrix = transf.matrix();
        assert_eq!(matrix[0][0], 1.0);
    }
}
